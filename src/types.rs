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

pub(crate) type Client = Arc<HttpClient<compat::CompatConnector>>;
pub(crate) type Cache<K = Uri, V = String> = Arc<Mutex<HashMap<K, V>>>;

#[cfg(feature = "smol_rt")]
pub(crate) mod compat {
    use std::future::Future;
    use smol::{Task, Async};
    use hyper::Uri;
    use crate::error::{ClientError, UrlNotParsed, IOError, NativeTLSError};
    use futures::future::BoxFuture;
    use std::task::{Context, Poll};
    use snafu::{OptionExt, ResultExt};
    use std::net::{TcpStream, Shutdown};
    use async_native_tls::TlsStream;
    use std::pin::Pin;
    use std::io;
    use futures::{AsyncWrite, AsyncRead};

    #[derive(Clone)]
    pub(crate) struct CompatExecutor;

    impl<F: Future + Send + 'static> hyper::rt::Executor<F> for CompatExecutor {
        fn execute(&self, fut: F) {
            Task::spawn(async { drop(fut.await) }).detach();
        }
    }

    /// Connects to URLs.
    #[derive(Clone)]
    pub(crate) struct CompatConnector;

    impl CompatConnector {
        pub(crate) fn new() -> Self {
            Self
        }
    }

    impl hyper::service::Service<Uri> for CompatConnector {
        type Response = CompatStream;
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
                        Ok(CompatStream::Plain(stream))
                    }
                    Some("https") => {
                        // In case of HTTPS, establish a secure TLS connection first.
                        let addr = format!("{}:{}", uri.host().unwrap(), uri.port_u16().unwrap_or(443));
                        let stream = Async::<TcpStream>::connect(addr).await.context(IOError)?;
                        let stream = async_native_tls::connect(host, stream)
                            .await
                            .context(NativeTLSError)?;
                        Ok(CompatStream::Tls(stream))
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
    pub(crate) enum CompatStream {
        /// A plain TCP connection.
        Plain(Async<TcpStream>),

        /// A TCP connection secured by TLS.
        Tls(TlsStream<Async<TcpStream>>),
    }

    impl hyper::client::connect::Connection for CompatStream {
        fn connected(&self) -> hyper::client::connect::Connected {
            hyper::client::connect::Connected::new()
        }
    }

    impl tokio::io::AsyncRead for CompatStream {
        fn poll_read(
            mut self: Pin<&mut Self>,
            cx: &mut Context<'_>,
            buf: &mut [u8],
        ) -> Poll<io::Result<usize>> {
            match &mut *self {
                CompatStream::Plain(s) => Pin::new(s).poll_read(cx, buf),
                CompatStream::Tls(s) => Pin::new(s).poll_read(cx, buf),
            }
        }
    }

    impl tokio::io::AsyncWrite for CompatStream {
        fn poll_write(
            mut self: Pin<&mut Self>,
            cx: &mut Context<'_>,
            buf: &[u8],
        ) -> Poll<io::Result<usize>> {
            match &mut *self {
                CompatStream::Plain(s) => Pin::new(s).poll_write(cx, buf),
                CompatStream::Tls(s) => Pin::new(s).poll_write(cx, buf),
            }
        }

        fn poll_flush(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<io::Result<()>> {
            match &mut *self {
                CompatStream::Plain(s) => Pin::new(s).poll_flush(cx),
                CompatStream::Tls(s) => Pin::new(s).poll_flush(cx),
            }
        }

        fn poll_shutdown(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<io::Result<()>> {
            match &mut *self {
                CompatStream::Plain(s) => {
                    s.get_ref().shutdown(Shutdown::Write)?;
                    Poll::Ready(Ok(()))
                }
                CompatStream::Tls(s) => Pin::new(s).poll_close(cx),
            }
        }
    }
}

#[cfg(feature = "async_std_rt")]
pub(crate) mod compat {
    use async_std::{io, net::TcpStream, task};
    use futures::io::AsyncWriteExt;
    use hyper::{
        body::HttpBody as _,
        client::connect::{Connected, Connection},
        rt::Executor,
        Body, Client, Uri
    };
    use pin_project::pin_project;
    use std::{
        error::Error,
        future::Future,
        pin::Pin,
        result::Result,
        task::{Context, Poll},
    };
    use tokio::io::{AsyncRead as TokioAsyncRead, AsyncWrite as TokioAsyncWrite};
    use async_native_tls::TlsStream;
    use futures::{AsyncWrite, AsyncRead};
    use std::net::Shutdown;
    use crate::error::{ClientError, UrlNotParsed, NativeTLSError, IOError, };
    use snafu::{ResultExt, OptionExt};
    use futures::future::BoxFuture;

    pub(crate) struct CompatExecutor;

    impl<Fut> Executor<Fut> for CompatExecutor
        where
            Fut: Future + Send + 'static,
            Fut::Output: Send + 'static,
    {
        fn execute(&self, fut: Fut) {
            task::spawn(async move { fut.await });
        }
    }

    #[derive(Clone)]
    pub(crate) struct CompatConnector;

    impl CompatConnector {
        pub(crate) fn new() -> Self {
            Self
        }
    }

    pub(crate) enum CompatStream {
        /// A plain TCP connection.
        Plain(TcpStream),

        /// A TCP connection secured by TLS.
        Tls(TlsStream<TcpStream>),
    }

    impl hyper::client::connect::Connection for CompatStream {
        fn connected(&self) -> hyper::client::connect::Connected {
            hyper::client::connect::Connected::new()
        }
    }

    impl TokioAsyncRead for CompatStream
    {
        fn poll_read(
            mut self: Pin<&mut Self>,
            cx: &mut Context<'_>,
            buf: &mut [u8],
        ) -> Poll<io::Result<usize>> {
            match &mut *self {
                CompatStream::Plain(s) => Pin::new(s).poll_read(cx, buf),
                CompatStream::Tls(s) => Pin::new(s).poll_read(cx, buf),
            }
        }
    }

    impl TokioAsyncWrite for CompatStream
    {
        fn poll_write(
            mut self: Pin<&mut Self>,
            cx: &mut Context<'_>,
            buf: &[u8],
        ) -> Poll<io::Result<usize>> {
            match &mut *self {
                CompatStream::Plain(s) => Pin::new(s).poll_write(cx, buf),
                CompatStream::Tls(s) => Pin::new(s).poll_write(cx, buf),
            }
        }

        fn poll_flush(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<std::io::Result<()>> {
            match &mut *self {
                CompatStream::Plain(s) => Pin::new(s).poll_flush(cx),
                CompatStream::Tls(s) => Pin::new(s).poll_flush(cx),
            }
        }

        fn poll_shutdown(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<std::io::Result<()>> {
            match &mut *self {
                CompatStream::Plain(s) => {
                    Pin::new(s).poll_close(cx)
                }
                CompatStream::Tls(s) => Pin::new(s).poll_close(cx),
            }
        }
    }

    impl hyper::service::Service<Uri> for CompatConnector {
        type Response = CompatStream;
        type Error = ClientError;
        type Future = BoxFuture<'static, Result<Self::Response, Self::Error>>;

        fn poll_ready(&mut self, _cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
            Poll::Ready(Ok(()))
        }

        fn call(&mut self, uri: Uri) -> Self::Future {
            Box::pin(async move {
                let host = uri.host().context(UrlNotParsed)?;

                match uri.scheme_str() {
                    Some("http") => {
                        let addr = format!("{}:{}", uri.host().unwrap(), uri.port_u16().unwrap_or(80));
                        let stream = TcpStream::connect(addr).await.context(IOError)?;
                        Ok(CompatStream::Plain(stream))
                    }
                    Some("https") => {
                        // In case of HTTPS, establish a secure TLS connection first.
                        let addr = format!("{}:{}", uri.host().unwrap(), uri.port_u16().unwrap_or(443));
                        let stream = TcpStream::connect(addr).await.context(IOError)?;
                        let stream = async_native_tls::connect(host, stream)
                            .await
                            .context(NativeTLSError)?;
                        Ok(CompatStream::Tls(stream))
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
}

#[cfg(feature = "tokio_rt")]
pub(crate) mod compat {
    use hyper_tls::HttpsConnector;
    use hyper::client::HttpConnector;

    pub type CompatConnector = HttpsConnector<HttpConnector>;
}