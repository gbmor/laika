use async_std::{
    io,
    net::{TcpListener, ToSocketAddrs},
    prelude::*,
    task,
};

mod conf;
mod handlers;

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

    let bind_addr = format!("{}:{}", conf.ip, conf.port);
    let acceptor = conf.tls_acceptor()?;

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
                let result = handlers::echo(&acceptor, &mut stream).await;
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
