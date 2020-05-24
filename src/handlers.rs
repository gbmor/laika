use async_std::{io, net::TcpStream, prelude::*};
use async_tls::TlsAcceptor;
use url::Url;

use std::str;

pub async fn entrance(
    acceptor: &TlsAcceptor,
    tcp_stream: &mut TcpStream,
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

    let _url = match Url::parse(req_str) {
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

    tls_stream.write_all(req_str.as_bytes()).await?;
    Ok(())
}
