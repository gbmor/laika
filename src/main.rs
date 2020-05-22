use ctrlc;
use tokio::{net::TcpListener, prelude::*};

mod conf;

type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

#[tokio::main]
async fn main() -> Result<()> {
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

    // Placeholder listener - will be removed ASAP.
    let mut listener =
        TcpListener::bind(&format!("{}:{}", conf.ip, conf.port)).await?;
    loop {
        let (mut socket, _) = listener.accept().await?;

        tokio::spawn(async move {
            let mut buf: [u8; 1024] = [0; 1024];

            loop {
                let n = match socket.read(&mut buf).await {
                    Ok(n) if n == 0 => return,
                    Ok(n) => n,
                    Err(e) => {
                        eprintln!("failed to read from socket; err = {:?}", e);
                        return;
                    },
                };

                if let Err(e) = socket.write_all(&buf[0..n]).await {
                    eprintln!("failed to write to socket; err = {:?}", e);
                    return;
                }
            }
        });
    }
}

#[test]
fn it_works() {
    // lol
    unimplemented!("TODO: Implement this");
}
