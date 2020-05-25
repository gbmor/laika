use async_std::{io, net::TcpStream, prelude::*};
use async_tls::{server::TlsStream, TlsAcceptor};
use url::Url;

use std::{fs, str};

use crate::{conf, response};

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
        let msg = format!(
            "{} BAD REQUEST: Invalid Scheme\r\n",
            response::BAD_REQUEST
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

    let fullpath = if fs::metadata(&fixedpath)?.file_type().is_dir() {
        format!("{}/index.gmi", fixedpath)
    } else {
        format!("{}", fixedpath)
    };

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

    let header =
        format!("{} text/gemini; charset=utf-8\r\n", response::SUCCESS);
    tls_stream.write_all(header.as_bytes()).await?;
    tls_stream.write_all(&fi).await?;
    tls_stream
        .write_all(response::FOOTER_TEXT.as_bytes())
        .await?;

    Ok(())
}
