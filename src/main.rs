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

    // The tls acceptor needs to be created before dropping privileges
    // in order to read the TLS key (assuming it's only readable by root)
    let acceptor = conf.tls_acceptor()?;

    if let Err(e) = privdrop::PrivDrop::default()
        .user(&conf.user)
        .group(&conf.group)
        .apply()
    {
        log::warn!(
            "Couldn't drop privileges to user {}, group {}: {}",
            &conf.user,
            &conf.group,
            e
        );
    }

    // Handle sigint
    ctrlc::set_handler(move || {
        log::warn!("Interrupt caught ...");
        std::process::exit(1);
    })
    .expect("Error initializing SIGINT handler");

    let bind_addr_string = format!("{}:{}", conf.ip, conf.port);

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
