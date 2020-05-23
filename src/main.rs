use async_std::{
    io,
    net::{TcpListener, TcpStream, ToSocketAddrs},
    prelude::*,
    task,
};
use async_tls::TlsAcceptor;

use std::sync::Arc;

mod conf;

async fn echo(
    acceptor: &TlsAcceptor,
    tcp_stream: &mut TcpStream,
) -> io::Result<()> {
    let addr = tcp_stream.peer_addr()?;
    println!("Conn from {}", addr);

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

fn main() -> io::Result<()> {
    let conf = conf::Conf::get();

    println!();
    println!("{}", conf.vers);
    println!("github.com/gbmor/laika");
    println!();
    println!("{:?}", conf);
    eprintln!();

    // Handle sigint
    ctrlc::set_handler(move || {
        eprintln!();
        eprintln!("Interrupt caught ...");
        std::process::exit(1);
    })
    .expect("Error initializing SIGINT handler");

    let server_conf = conf.server_config()?;
    let bind_addr = &format!("{}:{}", conf.ip, conf.port)[..];

    let acceptor = TlsAcceptor::from(Arc::new(server_conf));

    task::block_on(async {
        let bind_addr = bind_addr.to_socket_addrs().await;
        let bind_addr = bind_addr.unwrap().next().unwrap();
        let listener = TcpListener::bind(&bind_addr)
            .await
            .expect(&format!("Could not bind to {}:{}", conf.ip, conf.port));
        let mut incoming = listener.incoming();

        while let Some(stream) = incoming.next().await {
            let acceptor = acceptor.clone();
            let mut stream = stream.unwrap();

            task::spawn(async move {
                let result = echo(&acceptor, &mut stream).await;
                match result {
                    Ok(_) => {},
                    Err(e) => {
                        eprintln!("{:?}", e);
                    },
                }
            });
        }
    });

    Ok(())
}

#[test]
fn it_works() {
    // lol
    assert!(true);
}
