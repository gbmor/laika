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

use std::net::SocketAddr;
use std::str;

use tokio::io::AsyncReadExt;
use tokio::io::AsyncWriteExt;
use tokio::net::TcpStream;
use tokio_rustls::server::TlsStream;
use url::Url;

use crate::conf::Conf;
use crate::err::Supernova;
use crate::file;
use crate::response;

pub async fn flush_and_kill(stream: &mut TlsStream<TcpStream>, remote_address: SocketAddr) {
    if let Err(e) = stream.flush().await {
        log::error!("Could not flush writer to {}: {}", remote_address, e);
    };
    if let Err(e) = stream.shutdown().await {
        log::error!(
            "Could not shut down connection to {}: {}",
            remote_address,
            e
        );
    };
}

pub async fn entrance(
    stream: &mut TlsStream<TcpStream>,
    remote_address: SocketAddr,
) -> Result<Url, Supernova> {
    let mut req_buf: [u8; 1024] = [0; 1024];

    let n = match stream.read(&mut req_buf).await {
        Ok(n) => n,
        Err(e) => {
            let msg = format!("failed to read from socket: {}", e);
            return Err(Supernova::boom(&msg));
        }
    };

    let req_str = match str::from_utf8(&req_buf[..n - 1]) {
        Ok(v) => v,
        Err(e) => {
            let msg = format!("failed to parse request as UTF-8 string: {}", e);
            return Err(Supernova::boom(&msg).with_code(response::Code::BadRequest));
        }
    };

    log::info!("REQ {} :: {}", remote_address, req_str);

    if req_str.contains("../") || req_str.contains("/..") {
        let msg = format!("directory traversal attempted: {}", req_str);
        return Err(Supernova::boom(&msg).with_code(response::Code::BadRequest));
    };

    let url = match Url::parse(req_str) {
        Ok(v) => v,
        Err(e) => {
            let msg = format!("could not parse request as URL: {}", e);
            return Err(Supernova::boom(&msg).with_code(response::Code::BadRequest));
        }
    };

    let url_scheme = url.scheme();
    if url_scheme != "gemini" {
        let msg = format!("invalid URL scheme. refusing to proxy to: {}", url_scheme);
        return Err(Supernova::boom(&msg).with_code(response::Code::ProxyRequestRefused));
    }

    Ok(url)
}

pub async fn route(
    conf: &Conf,
    stream: &mut TlsStream<TcpStream>,
    remote_address: SocketAddr,
    req_url: Url,
) -> Result<(), Supernova> {
    let path = req_url.path();
    let root_directory = conf.root_directory();
    let root_directory_str = root_directory.display();

    let fixed_path = if path.is_empty() {
        format!("{}/{}", root_directory_str, conf.index_file_name())
    } else {
        format!("{}{}", root_directory_str, path)
    };

    log::debug!(
        "REQ {} :: full local request path: {}",
        remote_address,
        fixed_path
    );

    let (mut fd, mime) = file::get(&fixed_path).await?;

    let header = response::Code::Success.get_header(&mime);

    log::debug!(
        "REQ {} :: file {} has mime {}",
        remote_address,
        fixed_path,
        mime
    );

    let n = match stream.write(&header).await {
        Ok(n) => n,
        Err(e) => {
            let msg = format!("could not write header to tls socket: {}", e);
            return Err(Supernova::boom(&msg));
        }
    };

    let n = match tokio::io::copy(&mut fd, stream).await {
        Ok(v) => v as usize + n,
        Err(e) => {
            let msg = format!("could not write body to tls socket: {}", e);
            return Err(Supernova::boom(&msg));
        }
    };

    let bytes_written = match stream.write(response::footer_bytes()).await {
        Ok(v) => v + n,
        Err(e) => {
            let msg = format!("could not write footer to tls socket: {}", e);
            return Err(Supernova::boom(&msg));
        }
    };

    log::info!("REQ {} :: {} bytes written", remote_address, bytes_written);

    Ok(())
}
