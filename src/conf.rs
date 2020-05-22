use clap::{App, Arg};

#[derive(Debug)]
pub struct Conf {
    pub vers: String,
    pub ip: String,
    pub port: u16,
    pub rootdir: String,
    pub tls_cert: String,
    pub tls_key: String,
}

impl Conf {
    // Parses the command line flags and returns a Conf.
    // Sets defaults if flags are omitted.
    pub fn get() -> Conf {
        let matches = App::new("laika")
            .version(clap::crate_version!())
            .author("Ben Morrison <ben@gbmor.dev>")
            .about("Gemini protocol server")
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
                Arg::with_name("rootdir")
                    .short("r")
                    .long("rootdir")
                    .value_name("Root Directory")
                    .help("Directory where gemini files reside")
                    .takes_value(true),
            )
            .arg(
                Arg::with_name("cert")
                    .short("c")
                    .long("cert")
                    .value_name("TLS Certificate")
                    .help("Path to TLS certificate")
                    .takes_value(true),
            )
            .arg(
                Arg::with_name("key")
                    .short("k")
                    .long("key")
                    .value_name("TLS Private Key")
                    .help("Path to TLS private key")
                    .takes_value(true),
            )
            .get_matches();

        Conf {
            vers: format!("laika {}", clap::crate_version!()),
            ip: matches.value_of("ip").unwrap_or("127.0.0.1").into(),
            port: matches
                .value_of("port")
                .unwrap_or("1965")
                .parse()
                .unwrap_or_else(|_| 1965),
            rootdir: matches
                .value_of("rootdir")
                .unwrap_or("/var/gemini")
                .into(),
            tls_cert: matches
                .value_of("cert")
                .unwrap_or("/etc/ssl/cert.pem")
                .into(),
            tls_key: matches
                .value_of("key")
                .unwrap_or("/etc/ssl/private/key.pem")
                .into(),
        }
    }
}
