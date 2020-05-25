// Response codes
pub const INPUT: usize = 10;
pub const SUCCESS: usize = 20;
pub const SUCCESS_END_OF_CLIENT_CERT_SESSION: usize = 21;
pub const REDIRECT_TEMPORARY: usize = 30;
pub const REDIRECT_PERMANENT: usize = 31;
pub const TEMPORARY_FAILURE: usize = 40;
pub const SERVER_UNAVAILABLE: usize = 41;
pub const CGI_ERROR: usize = 42;
pub const PROXY_ERROR: usize = 43;
pub const SLOW_DOWN: usize = 44;
pub const PERMANENT_FAILURE: usize = 50;
pub const NOT_FOUND: usize = 51;
pub const GONE: usize = 52;
pub const PROXY_REQUEST_REFUSED: usize = 53;
pub const BAD_REQUEST: usize = 59;
pub const CLIENT_CERT_REQUIRED: usize = 60;
pub const TRANSIENT_CERT_REQUESTED: usize = 61;
pub const AUTHORISED_CERT_REQUIRED: usize = 62;
pub const CERT_NOT_ACCEPTED: usize = 63;
pub const FUTURE_CERT_REJECTED: usize = 64;
pub const EXPIRED_CERT_REJECTED: usize = 65;

pub const FOOTER_TEXT: &str =
    "\n\n~~~~ served by laika ~~~~~~~~~\nhttps://github.com/gbmor/laika\n\n";
