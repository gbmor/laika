/* Copyright (C) 2023  Ben Morrison <ben@gbmor.org>
 *
 * This program is free software: you can redistribute it and/or modify
 * it under the terms of the GNU Affero General Public License as published by
 * the Free Software Foundation, either version 3 of the License, or
 * (at your option) any later version.
 *
 * This program is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
 * GNU Affero General Public License for more details.
 *
 * You should have received a copy of the GNU Affero General Public License
 * along with this program.  If not, see <https: *www.gnu.org/licenses/>.
 */

use std::fmt;

pub const GEMINI_MIME: &str = "text/gemini";

// Response codes
#[allow(dead_code)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Code {
    Unknown = 99,
    Input = 10,
    SensitiveInput = 11,
    Success = 20,
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
    ClientCertificateRequired = 60,
    CertificateNotAuthorised = 61,
    CertificateNotValid = 62,
}

impl Code {
    pub fn get_header(&self, meta: &str) -> Vec<u8> {
        let msg = if *self == Code::Success {
            format!("{} {}\r\n", self, meta)
        } else {
            format!("{}\r\n", self)
        };

        msg.into_bytes()
    }
}

impl fmt::Display for Code {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let code_u8 = *self as u8;
        let msg = match self {
            Code::Unknown => format!("{} UNKNOWN", code_u8),
            Code::Input => format!("{} INPUT", code_u8),
            Code::SensitiveInput => format!("{} SENSITIVE INPUT", code_u8),
            Code::Success => format!("{}", code_u8),
            Code::RedirectTemporary => format!("{} REDIRECT - TEMPORARY", code_u8),
            Code::RedirectPermanent => format!("{} REDIRECT - PERMANENT", code_u8),
            Code::TemporaryFailure => format!("{} TEMPORARY FAILURE", code_u8),
            Code::ServerUnavailable => format!("{} SERVER UNAVAILABLE", code_u8),
            Code::CgiError => format!("{} CGI ERROR", code_u8),
            Code::ProxyError => format!("{} PROXY ERROR", code_u8),
            Code::SlowDown => format!("{} SLOW DOWN", code_u8),
            Code::PermanentFailure => format!("{} PERMANENT FAILURE", code_u8),
            Code::NotFound => format!("{} NOT FOUND", code_u8),
            Code::Gone => format!("{} GONE", code_u8),
            Code::ProxyRequestRefused => format!("{} PROXY REQUEST REFUSED", code_u8),
            Code::BadRequest => format!("{} BAD REQUEST", code_u8),
            Code::ClientCertificateRequired => format!("{} CLIENT CERTIFICATE REQUIRED", code_u8),
            Code::CertificateNotAuthorised => format!("{} CERTIFICATE NOT AUTHORISED", code_u8),
            Code::CertificateNotValid => format!("{} CERTIFICATE NOT VALID", code_u8),
        };

        write!(f, "{}", msg)
    }
}

// Appended to the bottom of .gmi files
pub fn footer_bytes<'a>() -> &'a [u8] {
    "\n\n~~~~ served by laika ~~~~~~~~~\nhttps://sr.ht/~gbmor/laika\n\n".as_bytes()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn header_check() {
        // we don't care about the metadata, just the code and the line ending
        let unknown = Code::Unknown.get_header("");
        assert!(unknown.ends_with("\r\n".as_bytes()));
        assert_eq!(&unknown[0..2], "99".as_bytes());

        let input = Code::Input.get_header("");
        assert!(input.ends_with("\r\n".as_bytes()));
        assert_eq!(&input[0..2], "10".as_bytes());

        let sensitive_input = Code::SensitiveInput.get_header("");
        assert!(sensitive_input.ends_with("\r\n".as_bytes()));
        assert_eq!(&sensitive_input[0..2], "11".as_bytes());

        let success = Code::Success.get_header("");
        assert!(success.ends_with("\r\n".as_bytes()));
        assert_eq!(&success[0..2], "20".as_bytes());

        let rdr_temp = Code::RedirectTemporary.get_header("");
        assert!(rdr_temp.ends_with("\r\n".as_bytes()));
        assert_eq!(&rdr_temp[0..2], "30".as_bytes());

        let rdr_perm = Code::RedirectPermanent.get_header("");
        assert!(rdr_perm.ends_with("\r\n".as_bytes()));
        assert_eq!(&rdr_perm[0..2], "31".as_bytes());

        let temp_fail = Code::TemporaryFailure.get_header("");
        assert!(temp_fail.ends_with("\r\n".as_bytes()));
        assert_eq!(&temp_fail[0..2], "40".as_bytes());

        let server_unavailable = Code::ServerUnavailable.get_header("");
        assert!(server_unavailable.ends_with("\r\n".as_bytes()));
        assert_eq!(&server_unavailable[0..2], "41".as_bytes());

        let cgi_err = Code::CgiError.get_header("");
        assert!(cgi_err.ends_with("\r\n".as_bytes()));
        assert_eq!(&cgi_err[0..2], "42".as_bytes());

        let proxy_err = Code::ProxyError.get_header("");
        assert!(proxy_err.ends_with("\r\n".as_bytes()));
        assert_eq!(&proxy_err[0..2], "43".as_bytes());

        let slow_down = Code::SlowDown.get_header("");
        assert!(slow_down.ends_with("\r\n".as_bytes()));
        assert_eq!(&slow_down[0..2], "44".as_bytes());

        let perm_fail = Code::PermanentFailure.get_header("");
        assert!(perm_fail.ends_with("\r\n".as_bytes()));
        assert_eq!(&perm_fail[0..2], "50".as_bytes());

        let not_found = Code::NotFound.get_header("");
        assert!(not_found.ends_with("\r\n".as_bytes()));
        assert_eq!(&not_found[0..2], "51".as_bytes());

        let gone = Code::Gone.get_header("");
        assert!(gone.ends_with("\r\n".as_bytes()));
        assert_eq!(&gone[0..2], "52".as_bytes());

        let proxy_req_refused = Code::ProxyRequestRefused.get_header("");
        assert!(proxy_req_refused.ends_with("\r\n".as_bytes()));
        assert_eq!(&proxy_req_refused[0..2], "53".as_bytes());

        let bad_req = Code::BadRequest.get_header("");
        assert!(bad_req.ends_with("\r\n".as_bytes()));
        assert_eq!(&bad_req[0..2], "59".as_bytes());

        let client_cert_required = Code::ClientCertificateRequired.get_header("");
        assert!(client_cert_required.ends_with("\r\n".as_bytes()));
        assert_eq!(&client_cert_required[0..2], "60".as_bytes());

        let cert_not_auth = Code::CertificateNotAuthorised.get_header("");
        assert!(cert_not_auth.ends_with("\r\n".as_bytes()));
        assert_eq!(&cert_not_auth[0..2], "61".as_bytes());

        let cert_not_valid = Code::CertificateNotValid.get_header("");
        assert!(cert_not_valid.ends_with("\r\n".as_bytes()));
        assert_eq!(&cert_not_valid[0..2], "62".as_bytes());
    }
}
