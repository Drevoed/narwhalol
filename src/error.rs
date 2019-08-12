use crate::constants::Region;

use snafu::Snafu;

pub type ApiResult<T, E = ApiError> = Result<T, E>;

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
pub enum ApiError {
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

    #[snafu(display("reqwest errored: {}", source))]
    ReqwestError { source: reqwest::Error },

    #[snafu(display("hyper errored: {}", source))]
    HyperError { source: reqwest::hyper::Error },

    #[snafu(display("could not parse reqwest url: {}", source))]
    ReqwestUrlNotParsed { source: reqwest::UrlError }
}

impl ApiError {
    pub fn check_status(region: Region, code: u16) -> ApiResult<()> {
        match code {
            400 => BadRequest.fail(),
            401 => Unauthorized.fail(),
            403 => Forbidden.fail(),
            404 => DataNotFound.fail(),
            405 => MethodNotAllowed.fail(),
            415 => UnsupportedMediaType.fail(),
            429 => RateLimitExceeded { limit: 0_usize }.fail(),
            500 => InternalServerError.fail(),
            502 => BadGateway.fail(),
            503 => ServiceUnavailable {
                region
            }
                .fail(),
            504 => GatewayTimeout.fail(),
            _ => Ok(()),
        }
    }
}

#[cfg(test)]
mod api_error_tests {
    use super::*;
    use crate::constants::Region;
    use crate::LEAGUE_CLIENT;

    #[test]
    fn returns_correct_status_codes() {
        let bad_r_err = LEAGUE_CLIENT.get_status(400).unwrap_err();
        let unauthorized_err = LEAGUE_CLIENT.get_status(401).unwrap_err();
        let forbidden_err = LEAGUE_CLIENT.get_status(403).unwrap_err();
        let not_found_err = LEAGUE_CLIENT.get_status(404).unwrap_err();
        let method_not_allowed_err = LEAGUE_CLIENT.get_status(405).unwrap_err();
        let unsupported_media_err = LEAGUE_CLIENT.get_status(415).unwrap_err();
        let rate_err = LEAGUE_CLIENT.get_status(429).unwrap_err();
        let internal_err = LEAGUE_CLIENT.get_status(500).unwrap_err();
        let bad_g_err = LEAGUE_CLIENT.get_status(502).unwrap_err();
        let service_err = LEAGUE_CLIENT.get_status(503).unwrap_err();
        let gateway_t_err = LEAGUE_CLIENT.get_status(504).unwrap_err();
        assert_matches!(bad_r_err, ApiError::BadRequest);
        assert_matches!(unauthorized_err, ApiError::Unauthorized);
        assert_matches!(forbidden_err, ApiError::Forbidden);
        assert_matches!(not_found_err, ApiError::DataNotFound);
        assert_matches!(method_not_allowed_err, ApiError::MethodNotAllowed);
        assert_matches!(unsupported_media_err, ApiError::UnsupportedMediaType);
        assert_matches!(rate_err, ApiError::RateLimitExceeded { limit: 0 });
        assert_matches!(internal_err, ApiError::InternalServerError);
        assert_matches!(bad_g_err, ApiError::BadGateway);
        assert_matches!(
            service_err,
            ApiError::ServiceUnavailable { region: Region::NA }
        );
        assert_matches!(gateway_t_err, ApiError::GatewayTimeout)
    }
}
