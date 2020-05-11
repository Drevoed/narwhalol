use crate::error::{ClientError, IOError, NativeTLSError, UnsupportedScheme, UrlNotParsed};
use async_native_tls::TlsStream;
use futures::future::BoxFuture;
use futures::prelude::*;
use hyper::{client::HttpConnector, Body, Client as HttpClient, Uri};
use parking_lot::Mutex;
use smol::{Async, Task};
use snafu::{OptionExt, ResultExt};
use std::collections::HashMap;
use std::io;
use std::net::{Shutdown, TcpStream};
use std::pin::Pin;
use std::sync::Arc;
use std::task::{Context, Poll};

pub(crate) type Client = Arc<HttpClient<SmolConnector>>;
pub(crate) type Cache<K = Uri, V = String> = Arc<Mutex<HashMap<K, V>>>;

#[derive(Clone)]
pub(crate) struct SmolExecutor;

impl<F: Future + Send + 'static> hyper::rt::Executor<F> for SmolExecutor {
    fn execute(&self, fut: F) {
        Task::spawn(async { drop(fut.await) }).detach();
    }
}

/// Connects to URLs.
#[derive(Clone)]
pub(crate) struct SmolConnector;

impl hyper::service::Service<Uri> for SmolConnector {
    type Response = SmolStream;
    type Error = ClientError;
    type Future = BoxFuture<'static, Result<Self::Response, Self::Error>>;

    fn poll_ready(&mut self, _: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        Poll::Ready(Ok(()))
    }

    fn call(&mut self, uri: Uri) -> Self::Future {
        Box::pin(async move {
            let host = uri.host().context(UrlNotParsed)?;

            match uri.scheme_str() {
                Some("http") => {
                    let addr = format!("{}:{}", uri.host().unwrap(), uri.port_u16().unwrap_or(80));
                    let stream = Async::<TcpStream>::connect(addr).await.context(IOError)?;
                    Ok(SmolStream::Plain(stream))
                }
                Some("https") => {
                    // In case of HTTPS, establish a secure TLS connection first.
                    let addr = format!("{}:{}", uri.host().unwrap(), uri.port_u16().unwrap_or(443));
                    let stream = Async::<TcpStream>::connect(addr).await.context(IOError)?;
                    let stream = async_native_tls::connect(host, stream)
                        .await
                        .context(NativeTLSError)?;
                    Ok(SmolStream::Tls(stream))
                }
                scheme => {
                    return Err(ClientError::UnsupportedScheme {
                        scheme: scheme.map(|s| s.to_owned()),
                    })
                }
            }
        })
    }
}

/// A TCP or TCP+TLS connection.
pub(crate) enum SmolStream {
    /// A plain TCP connection.
    Plain(Async<TcpStream>),

    /// A TCP connection secured by TLS.
    Tls(TlsStream<Async<TcpStream>>),
}

impl hyper::client::connect::Connection for SmolStream {
    fn connected(&self) -> hyper::client::connect::Connected {
        hyper::client::connect::Connected::new()
    }
}

impl tokio::io::AsyncRead for SmolStream {
    fn poll_read(
        mut self: Pin<&mut Self>,
        cx: &mut Context<'_>,
        buf: &mut [u8],
    ) -> Poll<io::Result<usize>> {
        match &mut *self {
            SmolStream::Plain(s) => Pin::new(s).poll_read(cx, buf),
            SmolStream::Tls(s) => Pin::new(s).poll_read(cx, buf),
        }
    }
}

impl tokio::io::AsyncWrite for SmolStream {
    fn poll_write(
        mut self: Pin<&mut Self>,
        cx: &mut Context<'_>,
        buf: &[u8],
    ) -> Poll<io::Result<usize>> {
        match &mut *self {
            SmolStream::Plain(s) => Pin::new(s).poll_write(cx, buf),
            SmolStream::Tls(s) => Pin::new(s).poll_write(cx, buf),
        }
    }

    fn poll_flush(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<io::Result<()>> {
        match &mut *self {
            SmolStream::Plain(s) => Pin::new(s).poll_flush(cx),
            SmolStream::Tls(s) => Pin::new(s).poll_flush(cx),
        }
    }

    fn poll_shutdown(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<io::Result<()>> {
        match &mut *self {
            SmolStream::Plain(s) => {
                s.get_ref().shutdown(Shutdown::Write)?;
                Poll::Ready(Ok(()))
            }
            SmolStream::Tls(s) => Pin::new(s).poll_close(cx),
        }
    }
}
