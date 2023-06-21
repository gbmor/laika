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

use std::error::Error;
use std::fmt::Debug;
use std::fs;
use std::io;
use std::path;
use std::sync::Arc;

use argh::FromArgs;
use serde::{Deserialize, Serialize};
use tokio::net::TcpListener;
use tokio_rustls::rustls::{Certificate, PrivateKey};
use tokio_rustls::{rustls, TlsAcceptor};

use crate::err::Supernova;

/// Configuration options for laika.
#[derive(FromArgs)]
struct Args {
    /// address:port for laika to bind to.
    #[argh(option, short = 'b', description = "address:port")]
    bind_address: Option<String>,

    // config file path.
    #[argh(option, short = 'c', description = "config file path")]
    config: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
struct ConfYaml {
    bind_address: String,
    tls_key: path::PathBuf,
    tls_cert: path::PathBuf,
    index_file_name: String,
    log_file: path::PathBuf,
    root_directory: path::PathBuf,
    debug: bool,
}

#[derive(Debug, Clone)]
pub struct Conf {
    addr: String,
    certs: Vec<Certificate>,
    key: PrivateKey,
    index_file_name: String,
    log_file: path::PathBuf,
    root_directory: path::PathBuf,
    debug: bool,
}

impl Conf {
    pub fn new() -> Result<Conf, Supernova> {
        let args: Args = argh::from_env();

        let config_file_path = match args.config {
            None => path::PathBuf::from("laika.yaml"),
            Some(a) => path::PathBuf::from(a),
        };

        let config_fd = match fs::File::open(config_file_path) {
            Ok(fd) => fd,
            Err(e) => {
                let msg = format!("Could not open config file: {}", e);
                return Err(Supernova::boom(&msg));
            }
        };
        let config_yaml: ConfYaml = match serde_yaml::from_reader(config_fd) {
            Ok(v) => v,
            Err(e) => {
                let msg = format!("Could not parse config file: {}", e);
                return Err(Supernova::boom(&msg));
            }
        };

        let cert_fd = match fs::File::open(config_yaml.tls_cert) {
            Err(e) => {
                let msg = format!("Could not open TLS certificate file: {}", e);
                return Err(Supernova::boom(&msg));
            }
            Ok(fd) => fd,
        };

        let certs: Vec<Certificate> = match rustls_pemfile::certs(&mut io::BufReader::new(cert_fd))
        {
            Ok(v) => v.into_iter().map(Certificate).collect(),
            Err(e) => {
                let msg = format!("Could not parse TLS certificate file: {}", e);
                return Err(Supernova::boom(&msg));
            }
        };

        let key_fd = match fs::File::open(config_yaml.tls_key) {
            Err(e) => {
                let msg = format!("Could not open TLS key file: {}", e);
                return Err(Supernova::boom(&msg));
            }
            Ok(key) => key,
        };

        let key = match rustls_pemfile::pkcs8_private_keys(&mut io::BufReader::new(key_fd)) {
            Ok(v) => {
                let keys: Vec<PrivateKey> = v.into_iter().map(PrivateKey).collect();
                keys[0].clone()
            }
            Err(e) => {
                let msg = format!("Could not parse TLS key file: {}", e);
                return Err(Supernova::boom(&msg));
            }
        };

        let addr = match args.bind_address {
            Some(v) => v,
            None => config_yaml.bind_address,
        };

        let index_file_name = config_yaml.index_file_name;
        let log_file = config_yaml.log_file;
        let debug = config_yaml.debug;
        let root_directory = config_yaml.root_directory;

        Ok(Conf {
            addr,
            certs,
            key,
            index_file_name,
            log_file,
            root_directory,
            debug,
        })
    }

    pub fn bind_address(&self) -> &str {
        &self.addr
    }
    pub fn debug(&self) -> bool {
        self.debug
    }
    pub fn index_file_name(&self) -> String {
        self.index_file_name.clone()
    }
    pub fn log_file(&self) -> path::PathBuf {
        self.log_file.to_owned()
    }
    pub fn root_directory(&self) -> path::PathBuf {
        self.root_directory.to_owned()
    }
    pub fn tls_cert(&self) -> Vec<Certificate> {
        self.certs.to_owned()
    }
    pub fn tls_key(&self) -> PrivateKey {
        self.key.to_owned()
    }

    pub async fn get_listener(&self) -> Result<(TcpListener, TlsAcceptor), Box<dyn Error>> {
        let tls_config = rustls::ServerConfig::builder()
            .with_safe_defaults()
            .with_no_client_auth()
            .with_single_cert(self.tls_cert(), self.tls_key())?;

        let tls_acceptor = TlsAcceptor::from(Arc::new(tls_config));
        let tcp_listener = TcpListener::bind(self.bind_address()).await?;

        Ok((tcp_listener, tls_acceptor))
    }
}
