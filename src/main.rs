use async_std::{
    io,
    net::{TcpListener, ToSocketAddrs},
    prelude::*,
    task,
};

mod conf;
mod handlers;
mod logging;

#[allow(dead_code)]
mod response;

fn main() -> io::Result<()> {
    let conf = conf::Conf::get();
    logging::init(&conf.logfile);

    println!();
    println!("{}", conf.vers);
    println!("github.com/gbmor/laika");
    println!();

    log::info!("laika starting ...");

    // Handle sigint
    ctrlc::set_handler(move || {
        log::warn!("Interrupt caught ...");
        std::process::exit(1);
    })
    .expect("Error initializing SIGINT handler");

    let bind_addr_string = format!("{}:{}", conf.ip, conf.port);
    let acceptor = conf.tls_acceptor()?;

    task::block_on(async {
        let bind_addr = bind_addr_string.to_socket_addrs().await;
        let bind_addr = bind_addr.unwrap().next().unwrap();
        let listener = TcpListener::bind(&bind_addr)
            .await
            .expect(&format!("Could not bind to {}:{}", conf.ip, conf.port));
        log::info!("Bound to {}", bind_addr_string);

        let mut incoming = listener.incoming();

        while let Some(stream) = incoming.next().await {
            let acceptor = acceptor.clone();
            let mut stream = stream.unwrap();
            let conf = conf.clone();

            task::spawn(async move {
                let result =
                    handlers::entrance(&acceptor, &mut stream, &conf).await;
                match result {
                    Ok(_) => {},
                    Err(e) => {
                        log::error!("{:?}", e);
                    },
                }
            });
        }
    });

    Ok(())
}
