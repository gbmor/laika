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

use crate::response;
use std::error::Error;
use std::fmt::Formatter;

#[derive(Clone, Debug)]
pub struct Supernova {
    message: String,
    code: response::Code,
}

impl Error for Supernova {}

impl Supernova {
    pub fn boom(message: &str) -> Supernova {
        Supernova {
            message: message.into(),
            code: response::Code::Unknown,
        }
    }

    pub fn code(&self) -> response::Code {
        self.code
    }

    pub fn with_code(&mut self, code: response::Code) -> Supernova {
        self.code = code;
        self.clone()
    }
}

impl std::fmt::Display for Supernova {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", &self.message)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_supernova() {
        let mut sn = Supernova::boom("test");
        assert_eq!(sn.code(), response::Code::Unknown);

        sn.with_code(response::Code::BadRequest);
        assert_eq!(sn.code(), response::Code::BadRequest);

        assert_eq!(format!("{}", sn), String::from("test"));
    }
}
