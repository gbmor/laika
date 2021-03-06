use async_std::io;
use async_tls::TlsAcceptor;
use clap::{App, Arg};
use daemonize::Daemonize;
use rustls::{
    internal::pemfile::{certs, pkcs8_private_keys},
    Certificate, NoClientAuth, PrivateKey, ServerConfig,
};

use std::{fs::File, io::BufReader, path::Path, sync::Arc};

#[derive(Debug, Clone)]
pub struct Conf {
    pub vers: String,
    pub ip: String,
    pub logfile: String,
    pub port: u16,
    pub rootdir: String,
    pub tls_cert: String,
    pub tls_key: String,
    pub daemon: bool,
    pub user: String,
    pub group: String,
}

impl Conf {
    // Parses the command line flags and returns a Conf.
    // Sets defaults if flags are omitted.
    #[allow(clippy::too_many_lines)] // kick rocks, clippy
    pub fn get() -> Conf {
        let matches = App::new(clap::crate_name!())
            .version(clap::crate_version!())
            .author("Ben Morrison <ben@gbmor.dev>")
            .about(clap::crate_description!())
            .arg(
                Arg::with_name("ip")
                    .short("i")
                    .long("ip")
                    .value_name("IP Address")
                    .help("IP address to bind to")
                    .takes_value(true),
            )
            .arg(
                Arg::with_name("port")
                    .short("p")
                    .long("port")
                    .value_name("Port")
                    .help("Port to listen on")
                    .takes_value(true),
            )
            .arg(
                Arg::with_name("logfile")
                    .short("l")
                    .long("logfile")
                    .value_name("Path")
                    .help("Path to log file")
                    .takes_value(true),
            )
            .arg(
                Arg::with_name("rootdir")
                    .short("r")
                    .long("rootdir")
                    .value_name("Path")
                    .help("Directory where gemini files reside")
                    .takes_value(true),
            )
            .arg(
                Arg::with_name("cert")
                    .short("c")
                    .long("cert")
                    .value_name("Path")
                    .help("Path to TLS certificate file")
                    .takes_value(true),
            )
            .arg(
                Arg::with_name("key")
                    .short("k")
                    .long("key")
                    .value_name("Path")
                    .help("Path to TLS private key file")
                    .takes_value(true),
            )
            .arg(
                Arg::with_name("daemon")
                    .short("d")
                    .long("daemon")
                    .help("If set, laika will run as a daemon")
                    .takes_value(false),
            )
            .arg(
                Arg::with_name("user")
                    .short("u")
                    .long("user")
                    .value_name("User Name")
                    .help("User laika should run as")
                    .takes_value(true),
            )
            .arg(
                Arg::with_name("group")
                    .short("g")
                    .long("group")
                    .value_name("Group Name")
                    .help("Group laika should run as")
                    .takes_value(true),
            )
            .get_matches();

        Conf {
            vers: format!("laika {}", clap::crate_version!()),
            ip: matches.value_of("ip").unwrap_or("0.0.0.0").into(),
            port: matches
                .value_of("port")
                .unwrap_or("1965")
                .parse()
                .unwrap_or_else(|_| 1965),
            logfile: matches
                .value_of("logfile")
                .unwrap_or("/var/log/laika.log")
                .into(),
            rootdir: matches.value_of("rootdir").unwrap_or("/var/gemini").into(),
            tls_cert: matches
                .value_of("cert")
                .unwrap_or("/etc/ssl/laika.pem")
                .into(),
            tls_key: matches
                .value_of("key")
                .unwrap_or("/etc/ssl/private/laika.key")
                .into(),
            user: matches.value_of("user").unwrap_or("laika").into(),
            group: matches.value_of("group").unwrap_or("laika").into(),
            daemon: matches.is_present("daemon"),
        }
    }

    // Pull certificate file
    fn get_cert(path: &Path) -> io::Result<Vec<Certificate>> {
        certs(&mut BufReader::new(File::open(path)?))
            .map_err(|_| io::Error::new(io::ErrorKind::InvalidInput, "invalid cert"))
    }

    // Pull private key
    fn get_key(path: &Path) -> io::Result<Vec<PrivateKey>> {
        pkcs8_private_keys(&mut BufReader::new(File::open(path)?))
            .map_err(|_| io::Error::new(io::ErrorKind::InvalidInput, "invalid key"))
    }

    // Generate the tls server config
    fn server_config(&self) -> io::Result<ServerConfig> {
        let cert = Conf::get_cert(Path::new(&self.tls_cert))?;
        let mut key = Conf::get_key(Path::new(&self.tls_key))?;

        let mut conf = ServerConfig::new(NoClientAuth::new());
        conf.set_single_cert(cert, key.remove(0))
            .map_err(|e| io::Error::new(io::ErrorKind::InvalidInput, e))?;

        Ok(conf)
    }

    // Return a TLS acceptor
    pub fn tls_acceptor(&self) -> io::Result<TlsAcceptor> {
        let server_config = self.server_config()?;
        Ok(TlsAcceptor::from(Arc::new(server_config)))
    }

    // Either drops privileges and daemonizes, or just drops privileges.
    // Controlled by the `-d` flag. If either fails, exit(1).
    pub fn drop_privs(&self) {
        if self.daemon {
            let daemon = Daemonize::new()
                .user(self.user.as_ref())
                .group(self.group.as_ref());

            match daemon.start() {
                Ok(_) => {}
                Err(e) => {
                    log::error!("Could not daemonize: {}", e);
                    std::process::exit(1);
                }
            }
        } else if let Err(e) = privdrop::PrivDrop::default()
            .user(&self.user)
            .group(&self.group)
            .apply()
        {
            log::error!(
                "Couldn't drop privileges to user {}, group {}: {}",
                self.user,
                self.group,
                e
            );
            std::process::exit(1);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn check_default_config() {
        let conf = Conf::get();
        assert_eq!(conf.ip, "0.0.0.0");
        assert_eq!(conf.port, 1965);
        assert_eq!(conf.logfile, "/var/log/laika.log");
        assert_eq!(conf.rootdir, "/var/gemini");
        assert_eq!(conf.tls_cert, "/etc/ssl/laika.pem");
        assert_eq!(conf.tls_key, "/etc/ssl/private/laika.key");
        assert_eq!(conf.user, "laika");
        assert_eq!(conf.group, "laika");
    }
}
