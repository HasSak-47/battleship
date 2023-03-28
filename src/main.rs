mod client;
mod server;

use std::env::args;

fn main() {
    let args : Vec<String> = args().into_iter().collect();

    match args[1].as_str() {
        "server" => server::main(),
        "client" => client::main(),
        _ => {}
    }
}
