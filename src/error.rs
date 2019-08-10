use snafu::{ensure, Backtrace, ErrorCompat, ResultExt, Snafu};
use crate::constants::Region;
use reqwest::Response;
use reqwest::StatusCode;
use self::ApiError::*;

#[derive(Debug, Snafu)]
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
    ServiceUnavailable {region: Region},
    #[snafu(display("Got 504: Gateway timeout"))]
    GatewayTimeout,

    #[snafu(display("reqwest errored: {}", source))]
    #[snafu(visibility(pub(crate)))]
    Other { source: reqwest::Error },
}

impl From<Response> for ApiError {
    fn from(r: Response) -> Self {
        match r.status().as_u16() {
            400 => ApiError::BadRequest,
            401 => ApiError::Unauthorized,
            403 => ApiError::Forbidden,
            404 => ApiError::DataNotFound,
            405 => ApiError::MethodNotAllowed,
            415 => ApiError::UnsupportedMediaType,
            429 => ApiError::RateLimitExceeded {limit: 0},
            500 => ApiError::BadGateway,
            504 => ApiError::GatewayTimeout,
            _ => unreachable!()
        }
    }
}

//503 handler
impl From<Region> for ApiError {
    fn from(region: Region) -> Self {
        ApiError::ServiceUnavailable {region}
    }
}
