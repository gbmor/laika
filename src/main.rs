/* Copyright (C) 2023  Ben Morrison <ben@gbmor.org>
 *
 * This program is free software: you can redistribute it and/or modify
 * it under the terms of the GNU Affero General Public License as published by
 * the Free Software Foundation, either version 3 of the License, or
 * (at your option) any later version.
 *
 * This program is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
 * GNU Affero General Public License for more details.
 *
 * You should have received a copy of the GNU Affero General Public License
 * along with this program.  If not, see <https: *www.gnu.org/licenses/>.
 */

use std::process;

use tokio::io::AsyncWriteExt;

mod conf;
mod err;
mod file;
mod handlers;
mod logging;
mod response;

static LAIKA_VERSION: &str = "0.1";

#[tokio::main]
async fn main() {
    let conf = match conf::Conf::new() {
        Ok(v) => v,
        Err(e) => {
            eprintln!("{}", e);
            process::exit(1);
        }
    };

    if let Err(e) = logging::init(&conf) {
        eprintln!("Failed to initialize logger: {}", e);
        process::exit(1);
    };

    log::info!("laika {} starting", LAIKA_VERSION);
    log::info!("Binding to {}", conf.bind_address());

    log::debug!("laika config:\n{:?}", conf);

    let (tcp_listener, tls_acceptor) = match conf.get_listener().await {
        Ok((tcp, tls)) => (tcp, tls),
        Err(e) => {
            log::error!("Could not get TCP listener or TLS acceptor: {}", e);
            process::exit(1);
        }
    };

    loop {
        let (socket, remote_address) = match tcp_listener.accept().await {
            Ok(v) => v,
            Err(e) => {
                log::error!("Could not accept connection: {}", e);
                continue;
            }
        };
        let tls_acceptor = tls_acceptor.clone();
        let conf = conf.clone();

        tokio::spawn(async move {
            let mut stream = match tls_acceptor.accept(socket).await {
                Ok(s) => s,
                Err(e) => {
                    log::error!("could not negotiate TLS: {}", e);
                    return;
                }
            };

            log::info!("REQ {} :: Connected", remote_address);

            let req_url = match handlers::entrance(&mut stream, remote_address).await {
                Ok(v) => v,
                Err(e) => {
                    log::error!("REQ {} :: {}", remote_address, e);
                    if e.code() != response::Code::Unknown {
                        let header = e.code().get_header("");
                        match stream.write(&header).await {
                            Ok(_) => (),
                            Err(e) => {
                                log::error!("REQ {} :: {}", remote_address, e);
                            }
                        };
                    }
                    handlers::flush_and_kill(&mut stream, remote_address).await;
                    log::info!("REQ {} :: Terminated", remote_address);
                    return;
                }
            };

            if let Err(e) = handlers::route(&conf, &mut stream, remote_address, req_url).await {
                log::error!("REQ {} :: {}", remote_address, e);
                if e.code() != response::Code::Unknown {
                    let header = e.code().get_header("");
                    if let Err(e) = stream.write_all(&header).await {
                        log::error!("REQ {} :: {}", remote_address, e);
                    }
                }
            }

            handlers::flush_and_kill(&mut stream, remote_address).await;
            log::info!("REQ {} :: Terminated", remote_address);
        });
    }
}
