use async_std::{io, net::TcpStream, prelude::*};
use async_tls::server::TlsStream;

use std::fs;

use crate::response;

// Pulls a file from disk based on its path, correcting for directories.
// Returns a tuple of (Vec<u8> (file bytes), String (mime type))
pub async fn parse(
    path: &str,
    tls_stream: &mut TlsStream<&mut TcpStream>,
) -> io::Result<(Vec<u8>, String)> {
    let metadata = match fs::metadata(&path) {
        Ok(m) => m,
        Err(e) => {
            if e.kind() == std::io::ErrorKind::NotFound {
                let msg = format!("{} NOT FOUND\r\n", response::Code::NotFound);
                log::error!("REQ :: {} NOT FOUND :: {}", response::Code::NotFound, e);
                tls_stream.write_all(msg.as_bytes()).await?;
                return Err(e);
            }
            let msg = format!("{} TEMPORARY FAILURE\r\n", response::Code::TemporaryFailure);
            log::error!(
                "REQ :: {} TEMPORARY FAILURE :: {}",
                response::Code::TemporaryFailure,
                e
            );
            tls_stream.write_all(msg.as_bytes()).await?;
            return Err(e);
        }
    };

    let fullpath = if metadata.file_type().is_dir() {
        format!("{}/index.gmi", path)
    } else {
        path.to_string()
    };

    let fi = match fs::read(&fullpath) {
        Ok(f) => f,
        Err(e) => {
            if e.kind() == std::io::ErrorKind::NotFound {
                let msg = format!("{} NOT FOUND\r\n", response::Code::NotFound);
                log::error!("REQ :: {} NOT FOUND :: {}", response::Code::NotFound, e);
                tls_stream.write_all(msg.as_bytes()).await?;
                return Err(e);
            }
            let msg = format!("{} TEMPORARY FAILURE\r\n", response::Code::TemporaryFailure);
            log::error!(
                "REQ :: {} TEMPORARY FAILURE :: {}",
                response::Code::TemporaryFailure,
                e
            );
            tls_stream.write_all(msg.as_bytes()).await?;
            return Err(e);
        }
    };

    let mime = if fullpath.ends_with(".gmi") {
        "text/gemini; charset=utf-8".to_string()
    } else {
        tree_magic::from_u8(&fi)
    };

    let mime = if mime.contains("text/") && !mime.contains("; charset=utf-8") {
        format!("{}; charset=utf-8", mime)
    } else {
        mime
    };

    Ok((fi, mime))
}
