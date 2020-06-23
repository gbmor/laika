use async_std::{
    io,
    net::{TcpListener, ToSocketAddrs},
    prelude::*,
    task,
};

mod conf;
mod files;
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

    // Handle sigint
    ctrlc::set_handler(move || {
        log::warn!("Interrupt caught ...");
        std::process::exit(1);
    })
    .expect("Error initializing SIGINT handler");

    task::block_on(async {
        let bind_addr_string = format!("{}:{}", conf.ip, conf.port);
        let bind_addr = bind_addr_string.to_socket_addrs().await;
        let bind_addr = bind_addr.unwrap().next().unwrap();
        let listener = TcpListener::bind(&bind_addr)
            .await
            .expect(&format!("Could not bind to {}:{}", conf.ip, conf.port));

        log::info!("Bound to {}", bind_addr_string);

        // Now we drop privileges. The default port is not a privileged port,
        // but we shouldn't fail if the user specifies a port <1024.
        conf.drop_privs();

        let mut incoming = listener.incoming();

        while let Some(strm) = incoming.next().await {
            let mut stream = if let Ok(s) = strm {
                s
            } else {
                log::error!("Connection error: {:?}", strm.unwrap_err());
                continue;
            };

            let conf = conf.clone();
            let acceptor = acceptor.clone();

            task::spawn(async move {
                let result =
                    handlers::entrance(&acceptor, &mut stream, &conf).await;
                match result {
                    Ok(_) => {}
                    Err(e) => {
                        log::error!("{}", e);
                    }
                }
            });
        }
    });

    Ok(())
}
