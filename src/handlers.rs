use async_std::{io, net::TcpStream, prelude::*};
use async_tls::TlsAcceptor;

pub async fn echo(
    acceptor: &TlsAcceptor,
    tcp_stream: &mut TcpStream,
) -> io::Result<()> {
    let addr = tcp_stream.peer_addr()?;
    log::info!("Connection from {}", addr);

    let handshake = acceptor.accept(tcp_stream);

    let mut tls_stream = handshake.await?;
    let mut buf: [u8; 1024] = [0; 1024];

    loop {
        let n = match tls_stream.read(&mut buf).await {
            Ok(n) if n == 0 => return Ok(()),
            Ok(n) => n,
            Err(e) => {
                eprintln!("Failed to read from socket: {}", e);
                return Err(e);
            },
        };

        if let Err(e) = tls_stream.write_all(&buf[0..n]).await {
            eprintln!("Failed to write to socket: {}", e);
            return Err(e);
        }
    }
}
