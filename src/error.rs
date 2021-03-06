//!

use crate::constants::Region;

use futures::future::{err, ok};

use futures::Future;
use snafu::Snafu;
use std::string::FromUtf8Error;

macro_rules! assert_matches {
    ($expression:expr, $($pattern:tt)+) => {
        match $expression {
            $($pattern)+ => (),
            ref e => panic!("Assertion failed: `{:?}` does not match `{}`", e, stringify!($($pattern)+))
        }
    };
}

/// Custom error type for an api errors
// TODO: Remove some of the fields and only keep the ones that the client could catch
// Example: rate-limiting, service unavailable
#[derive(Debug, Snafu)]
#[snafu(visibility(pub(crate)))]
pub enum ClientError {
    /// Bad request
    #[snafu(display("Got 400: Bad Request"))]
    BadRequest,
    /// Unauthorized
    #[snafu(display("Got 401: Unauthorized"))]
    Unauthorized,
    /// Forbidden
    #[snafu(display("Got 403: Forbidden"))]
    Forbidden,
    /// Data not found
    #[snafu(display("Got 404: Data not found"))]
    DataNotFound,
    /// Method is not allowwed
    #[snafu(display("Got 405: Method not allowed"))]
    MethodNotAllowed,
    /// Unsupported media type
    #[snafu(display("Got 415: Unsupported media type"))]
    UnsupportedMediaType,
    /// This error is returned when you have exceeded your rate limit for an api.
    // TODO: Add rate-limiting
    #[snafu(display("Got 429: Rate limit exceeded. limit: {}", limit))]
    RateLimitExceeded { limit: usize },
    /// Internal server error
    #[snafu(display("Got 500: Internal server error"))]
    InternalServerError,
    /// Bad gateway
    #[snafu(display("Got 502: Bad Gateway"))]
    BadGateway,
    /// This error is returned when the riot api servers are on maintenance.
    /// Please visit https://developer.riotgames.com/api-status/ for more information
    #[snafu(display("Got 503: Service unavailable for region {:?}", region))]
    ServiceUnavailable { region: Region },
    /// Gateway Timeout
    #[snafu(display("Got 504: Gateway timeout"))]
    GatewayTimeout,

    /// Internal url not parsed error
    #[snafu(display("could not parse url"))]
    UrlNotParsed,

    /// Hyper error
    #[snafu(display("hyper errored: {}", source))]
    HyperError { source: hyper::Error },
    /// This error is returned when the user provides no token
    #[snafu(display("Please provide the correct RIOT_API_KEY variable because {}", source))]
    NoToken { source: std::env::VarError },
    /// This error is returned when the user provides malformed token
    #[snafu(display("Provided token {} is not correct riot api token", token))]
    WrongToken { token: String },

    #[snafu(display("Unsupported scheme: {:?}", scheme))]
    UnsupportedScheme { scheme: Option<String> },

    #[snafu(display("Could not make string from ut8: {}", source))]
    FromUTF8Error { source: FromUtf8Error },

    #[snafu(display("Got native tls error: {}", source))]
    NativeTLSError { source: native_tls::Error },

    #[snafu(display("Got io error: {}", source))]
    IOError { source: std::io::Error },
}

impl ClientError {
    pub fn check_status(region: Region, code: u16) -> Result<(), ClientError> {
        use self::ClientError::*;
        match code {
            400 => Err(BadRequest),
            401 => Err(Unauthorized),
            403 => Err(Forbidden),
            404 => Err(DataNotFound),
            405 => Err(MethodNotAllowed),
            415 => Err(UnsupportedMediaType),
            429 => Err(RateLimitExceeded { limit: 0_usize }),
            500 => Err(InternalServerError),
            502 => Err(BadGateway),
            503 => Err(ServiceUnavailable { region }),
            504 => Err(GatewayTimeout),
            _ => Ok(()),
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
        let lapi = LeagueClient::new(Region::NA).unwrap();
        let bad_r_err = lapi.get_status(400).unwrap_err();
        let unauthorized_err = lapi.get_status(401).unwrap_err();
        let forbidden_err = lapi.get_status(403).unwrap_err();
        let not_found_err = lapi.get_status(404).unwrap_err();
        let method_not_allowed_err = lapi.get_status(405).unwrap_err();
        let unsupported_media_err = lapi.get_status(415).unwrap_err();
        let rate_err = lapi.get_status(429).unwrap_err();
        let internal_err = lapi.get_status(500).unwrap_err();
        let bad_g_err = lapi.get_status(502).unwrap_err();
        let service_err = lapi.get_status(503).unwrap_err();
        let gateway_t_err = lapi.get_status(504).unwrap_err();
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
