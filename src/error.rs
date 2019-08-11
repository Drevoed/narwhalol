use snafu::{ensure, Backtrace, ErrorCompat, ResultExt, Snafu};
use crate::constants::Region;
use reqwest::Response;
use reqwest::StatusCode;
use self::ApiError::*;

pub type ApiResult<T, E = ApiError> = Result<T, E>;

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
    ServiceUnavailable {region: Region},
    #[snafu(display("Got 504: Gateway timeout"))]
    GatewayTimeout,

    #[snafu(display("reqwest errored: {}", source))]
    Other { source: reqwest::Error },
}
