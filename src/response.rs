use std::fmt;

// Response codes
#[derive(Clone, Copy)]
pub enum Code {
    Input = 10,
    Success = 20,
    SuccessEndOfClientCertSession = 21,
    RedirectTemporary = 30,
    RedirectPermanent = 31,
    TemporaryFailure = 40,
    ServerUnavailable = 41,
    CgiError = 42,
    ProxyError = 43,
    SlowDown = 44,
    PermanentFailure = 50,
    NotFound = 51,
    Gone = 52,
    ProxyRequestRefused = 53,
    BadRequest = 59,
    ClientCertRequired = 60,
    TransientCertRequired = 61,
    AuthorisedCertRequired = 62,
    CertNotAccepted = 63,
    FutureCertRejected = 64,
    ExpiredCertRejected = 65,
}

impl fmt::Display for Code {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", *self as u8)
    }
}

pub fn footer_bytes<'a>() -> &'a [u8] {
    "\n\n~~~~ served by laika ~~~~~~~~~\nhttps://github.com/gbmor/laika\n\n"
        .as_bytes()
}
