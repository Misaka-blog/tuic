use crate::{config::ConfigBuilder, server::Socks5Server};
use std::{env, error::Error};

mod certificate;
mod config;
mod connection;
mod error;
mod server;

pub use crate::{
    config::Config,
    connection::{Connection, ConnectionGuard},
    error::ClientError,
};

#[tokio::main]
async fn main() {
    let args: Vec<String> = env::args().collect();

    let mut cfg_builder = ConfigBuilder::new();

    let config = match cfg_builder.parse(&args) {
        Ok(cfg) => cfg,
        Err(err) => {
            eprintln!("{}\n\n{}", err, cfg_builder.get_usage());
            return;
        }
    };

    let (conn_guard, channel_msg_sender) = match ConnectionGuard::new(&config) {
        Ok(res) => res,
        Err(err) => {
            eprintln!("{}", err);
            return;
        }
    };
    conn_guard.run().await;

    let socks5_server = Socks5Server::new(&config, channel_msg_sender);

    match socks5_server.run().await {
        Ok(()) => {}
        Err(err) => {
            eprintln!("{}", err);
            return;
        }
    }
}

pub fn exit(err: Box<dyn Error>) -> ! {
    eprintln!("{}", err);
    std::process::exit(1);
}
