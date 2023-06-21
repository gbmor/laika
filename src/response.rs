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
    Unknown = 00,
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
    pub fn get_header(&self, mime: &str) -> Vec<u8> {
        let msg = if *self == Code::Success {
            format!("{} {}\r\n", self, mime)
        } else {
            format!("{}\r\n", self)
        };

        msg.into_bytes()
    }
}

impl fmt::Display for Code {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let code_str = format!("{}", *self as u8);
        let msg = match self {
            Code::Unknown => format!("{} UNKNOWN", code_str),
            Code::Input => format!("{} INPUT", code_str),
            Code::SensitiveInput => format!("{} SENSITIVE INPUT", code_str),
            Code::Success => format!("{}", code_str),
            Code::RedirectTemporary => format!("{} REDIRECT - TEMPORARY", code_str),
            Code::RedirectPermanent => format!("{} REDIRECT - PERMANENT", code_str),
            Code::TemporaryFailure => format!("{} TEMPORARY FAILURE", code_str),
            Code::ServerUnavailable => format!("{} SERVER UNAVAILABLE", code_str),
            Code::CgiError => format!("{} CGI ERROR", code_str),
            Code::ProxyError => format!("{} PROXY ERROR", code_str),
            Code::SlowDown => format!("{} SLOW DOWN", code_str),
            Code::PermanentFailure => format!("{} PERMANENT FAILURE", code_str),
            Code::NotFound => format!("{} NOT FOUND", code_str),
            Code::Gone => format!("{} GONE", code_str),
            Code::ProxyRequestRefused => format!("{} PROXY REQUEST REFUSED", code_str),
            Code::BadRequest => format!("{} BAD REQUEST", code_str),
            Code::ClientCertificateRequired => format!("{} CLIENT CERTIFICATE REQUIRED", code_str),
            Code::CertificateNotAuthorised => format!("{} CERTIFICATE NOT AUTHORISED", code_str),
            Code::CertificateNotValid => format!("{} CERTIFICATE NOT VALID", code_str),
        };

        write!(f, "{}", msg)
    }
}

// Appended to the bottom of .gmi files
pub fn footer_bytes<'a>() -> &'a [u8] {
    "\n\n~~~~ served by laika ~~~~~~~~~\nhttps://sr.ht/~gbmor/laika\n\n".as_bytes()
}
