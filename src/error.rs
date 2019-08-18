//!

use crate::constants::Region;

use snafu::Snafu;
use futures::{TryFutureExt, FutureExt, Future};
use futures::prelude::*;
use futures::future::{ok, err};

macro_rules! assert_matches {
    ($expression:expr, $($pattern:tt)+) => {
        match $expression {
            $($pattern)+ => (),
            ref e => panic!("Assertion failed: `{:?}` does not match `{}`", e, stringify!($($pattern)+))
        }
    };
}

#[derive(Debug, Snafu)]
#[snafu(visibility(pub(crate)))]
pub enum ClientError {
    #[snafu(display("Got 400: Bad Request"))]
    BadRequest,
    #[snafu(display("Got 401: Unauthorized"))]
    Unauthorized,
    #[snafu(display("Got 403: Forbidden"))]
    Forbidden,
    #[snafu(display("Got 404: Data not found"))]
    DataNotFound,
    #[snafu(display("Got 405: Method not allowed"))]
    MethodNotAllowed,
    #[snafu(display("Got 415: Unsupported media type"))]
    UnsupportedMediaType,
    #[snafu(display("Got 429: Rate limit exceeded. limit: {}", limit))]
    RateLimitExceeded { limit: usize },
    #[snafu(display("Got 500: Internal server error"))]
    InternalServerError,
    #[snafu(display("Got 502: Bad Gateway"))]
    BadGateway,
    #[snafu(display("Got 503: Service unavailable for region {:?}", region))]
    ServiceUnavailable { region: Region },
    #[snafu(display("Got 504: Gateway timeout"))]
    GatewayTimeout,

    #[snafu(display("could not parse url"))]
    UrlNotParsed,

    #[snafu(display("hyper errored: {}", source))]
    Other { source: hyper::Error },
}

impl ClientError {
    pub fn check_status(region: Region, code: u16) -> impl Future<Output = Result<(), ClientError>> {
        use self::ClientError::*;
        match code {
            400 => err(BadRequest),
            401 => err(Unauthorized),
            403 => err(Forbidden),
            404 => err(DataNotFound),
            405 => err(MethodNotAllowed),
            415 => err(UnsupportedMediaType),
            429 => err(RateLimitExceeded { limit: 0_usize }),
            500 => err(InternalServerError),
            502 => err(BadGateway),
            503 => err(ServiceUnavailable { region }),
            504 => err(GatewayTimeout),
            _ => ok(()),
        }
    }
}

#[cfg(test)]
mod api_error_tests {
    use super::*;
    use crate::api::LeagueClient;
    use crate::constants::Region;

    #[test]
    fn returns_correct_status_codes() {
        let mut runtime = tokio::runtime::current_thread::Runtime::new().unwrap();
        let lapi = LeagueClient::new(Region::NA).unwrap();
        let bad_r_err = runtime.block_on(lapi.get_status(400)).unwrap_err();
        let unauthorized_err = runtime.block_on(lapi.get_status(401)).unwrap_err();
        let forbidden_err = runtime.block_on(lapi.get_status(403)).unwrap_err();
        let not_found_err = runtime.block_on(lapi.get_status(404)).unwrap_err();
        let method_not_allowed_err = runtime.block_on(lapi.get_status(405)).unwrap_err();
        let unsupported_media_err = runtime.block_on(lapi.get_status(415)).unwrap_err();
        let rate_err = runtime.block_on(lapi.get_status(429)).unwrap_err();
        let internal_err = runtime.block_on(lapi.get_status(500)).unwrap_err();
        let bad_g_err = runtime.block_on(lapi.get_status(502)).unwrap_err();
        let service_err = runtime.block_on(lapi.get_status(503)).unwrap_err();
        let gateway_t_err = runtime.block_on(lapi.get_status(504)).unwrap_err();
        assert_matches!(bad_r_err, ClientError::BadRequest);
        assert_matches!(unauthorized_err, ClientError::Unauthorized);
        assert_matches!(forbidden_err, ClientError::Forbidden);
        assert_matches!(not_found_err, ClientError::DataNotFound);
        assert_matches!(method_not_allowed_err, ClientError::MethodNotAllowed);
        assert_matches!(unsupported_media_err, ClientError::UnsupportedMediaType);
        assert_matches!(rate_err, ClientError::RateLimitExceeded { limit: 0 });
        assert_matches!(internal_err, ClientError::InternalServerError);
        assert_matches!(bad_g_err, ClientError::BadGateway);
        assert_matches!(
            service_err,
            ClientError::ServiceUnavailable { region: Region::NA }
        );
        assert_matches!(gateway_t_err, ClientError::GatewayTimeout)
    }
}
