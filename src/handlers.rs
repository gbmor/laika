use async_std::{io, net::TcpStream, prelude::*};
use async_tls::{server::TlsStream, TlsAcceptor};
use url::Url;

use std::{fs, str};

use crate::{conf, response};

pub async fn entrance(
    acceptor: &TlsAcceptor,
    tcp_stream: &mut TcpStream,
    conf: &conf::Conf,
) -> io::Result<()> {
    let addr = tcp_stream.peer_addr()?;
    log::info!("Connection from {}", addr);

    let mut tls_stream = acceptor.accept(tcp_stream).await?;
    let mut req_buf: [u8; 1026] = [0; 1026];

    let n = match tls_stream.read(&mut req_buf).await {
        Ok(n) => n,
        Err(e) => {
            log::error!(
                "REQ from {} :: Failed to read from socket: {}",
                addr,
                e
            );
            return Err(e);
        },
    };

    let req_str = match str::from_utf8(&req_buf[..n - 1]) {
        Ok(s) => s,
        Err(e) => {
            log::error!(
                "REQ from {} :: Failed to parse request as UTF-8: {}",
                addr,
                e
            );
            "/ \r\n"
        },
    };

    let url = match Url::parse(req_str) {
        Ok(url) => url,
        Err(e) => {
            log::error!(
                "REQ from {} :: Unable to parse request as URL: {}",
                addr,
                e
            );
            return Ok(());
        },
    };

    if url.scheme() != "gemini" {
        tls_stream
            .write_all(b"59 BAD REQUEST: Invalid Scheme\r\n")
            .await?;
        log::error!("REQ from {} :: Invalid scheme: {}", addr, url.scheme());
        return Ok(());
    }

    log::info!("REQ from {} :: {}", addr, url);

    if let Err(e) = route(&mut tls_stream, &url, &conf).await {
        log::error!("REQ from {} :: Routing error: {}", addr, e);
    };

    Ok(())
}

async fn route(
    tls_stream: &mut TlsStream<&mut TcpStream>,
    req_url: &Url,
    conf: &conf::Conf,
) -> io::Result<()> {
    let path = req_url.path();

    serve_from_root(tls_stream, &conf.rootdir, path).await?;

    Ok(())
}

async fn serve_from_root(
    tls_stream: &mut TlsStream<&mut TcpStream>,
    rootdir: &str,
    path: &str,
) -> io::Result<()> {
    let fixedpath;

    if path.ends_with("/") || path == "" {
        fixedpath = format!("{}/index.gmi", path);
    } else {
        fixedpath = path.to_string();
    }

    let mut fullpath = format!("{}{}", rootdir, fixedpath);

    if fs::metadata(&fullpath)?.file_type().is_dir() {
        fullpath = format!("{}/index.gmi", fullpath);
    }

    let fi = match fs::read(&fullpath) {
        Ok(f) => f,
        Err(e) => {
            if e.kind() == std::io::ErrorKind::NotFound {
                let msg = format!("{} NOT FOUND\r\n", response::NOT_FOUND);
                log::error!(
                    "REQ :: {} NOT FOUND :: {}",
                    response::NOT_FOUND,
                    e
                );
                tls_stream.write_all(msg.as_bytes()).await?;
                return Ok(());
            }
            let msg = format!(
                "{} TEMPORARY FAILURE\r\n",
                response::TEMPORARY_FAILURE
            );
            log::error!(
                "REQ :: {} TEMPORARY FAILURE :: {}",
                response::TEMPORARY_FAILURE,
                e
            );
            tls_stream.write_all(msg.as_bytes()).await?;
            return Ok(());
        },
    };

    tls_stream
        .write_all(b"20 text/gemini; charset=utf-8\r\n")
        .await?;
    tls_stream.write_all(&fi).await?;

    Ok(())
}
