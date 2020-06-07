use async_std::{io, net::TcpStream, prelude::*};
use async_tls::{server::TlsStream, TlsAcceptor};
use url::Url;

use std::str;

use crate::{conf, files, response};

// This is the initial handler for each new connection.
// Read the request, validate it, and pass it off.
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
        }
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
        }
    };

    if req_str.contains("..") {
        log::warn!(
            "REQ from {} :: Directory traversal attempted: {}",
            addr,
            req_str
        );
        let msg = format!("{} BAD REQUEST\r\n", response::Code::BadRequest);
        tls_stream.write_all(msg.as_bytes()).await?;
        return Ok(());
    }

    let url = match Url::parse(req_str) {
        Ok(url) => url,
        Err(e) => {
            log::error!(
                "REQ from {} :: Unable to parse request as URL: {}",
                addr,
                e
            );
            return Ok(());
        }
    };

    if url.scheme() != "gemini" {
        let msg = format!(
            "{} BAD REQUEST: Invalid Scheme\r\n",
            response::Code::BadRequest
        );
        tls_stream.write_all(msg.as_bytes()).await?;
        log::error!("REQ from {} :: Invalid scheme: {}", addr, url.scheme());
        return Ok(());
    }

    log::info!("REQ from {} :: {}", addr, url);

    if let Err(e) = route(&mut tls_stream, &url, &conf).await {
        log::error!("REQ from {} :: Routing error: {}", addr, e);
    };

    Ok(())
}

// Determines the next hop after the entrance handler.
async fn route(
    tls_stream: &mut TlsStream<&mut TcpStream>,
    req_url: &Url,
    conf: &conf::Conf,
) -> io::Result<()> {
    let path = req_url.path();

    serve_from_root(tls_stream, &conf.rootdir, path).await?;

    Ok(())
}

// Serves pages from the main gemini root directory.
async fn serve_from_root(
    tls_stream: &mut TlsStream<&mut TcpStream>,
    rootdir: &str,
    path: &str,
) -> io::Result<()> {
    let fixedpath = if path == "" {
        format!("{}/index.gmi", rootdir)
    } else if path.ends_with("/") {
        format!("{}{}index.gmi", rootdir, path)
    } else {
        format!("{}{}", rootdir, path)
    };

    let (fi, mime) = match files::parse(&fixedpath, tls_stream).await {
        Ok((fi, mime)) => (fi, mime),
        Err(_) => return Ok(()),
    };

    let header = format!("{} {}\r\n", response::Code::Success, mime);
    tls_stream.write_all(header.as_bytes()).await?;
    tls_stream.write_all(&fi).await?;

    if mime == "text/gemini; charset=utf-8" {
        tls_stream.write_all(response::footer_bytes()).await?;
    }

    Ok(())
}
