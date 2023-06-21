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

use std::io;

use tokio::fs;

use crate::err::Supernova;
use crate::response;

pub async fn get(path: &str) -> Result<(fs::File, String), Supernova> {
    let metadata = match fs::metadata(path).await {
        Ok(m) => m,
        Err(e) => {
            let code = match e.kind() {
                io::ErrorKind::NotFound => response::Code::NotFound,
                _ => response::Code::PermanentFailure,
            };

            let msg = format!("{}", e);
            let wrapped_err = Supernova::boom(&msg).with_code(code);

            return Err(wrapped_err);
        }
    };

    let path = if metadata.is_dir() {
        format!("{}/index.gmi", path)
    } else {
        path.to_string()
    };

    let fd = match fs::File::open(&path).await {
        Ok(fd) => fd,
        Err(e) => {
            let code = match e.kind() {
                io::ErrorKind::NotFound => response::Code::NotFound,
                _ => response::Code::PermanentFailure,
            };

            let msg = format!("{}", e);
            let wrapped_err = Supernova::boom(&msg).with_code(code);

            return Err(wrapped_err);
        }
    };

    let mime = match tree_magic_mini::from_filepath(path.as_ref()) {
        Some(m) => {
            if path.ends_with(".gmi") {
                response::GEMINI_MIME
            } else {
                m
            }
        }
        None => response::GEMINI_MIME,
    };

    Ok((fd, mime.to_string()))
}
