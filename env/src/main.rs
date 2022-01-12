use std::env;
use std::process;

fn main() {
    // HOST=myserver
    // PORT=8080
    let host_key = "HOST";
    let port_key = "PORT";
    let default_port = 8080;

    let host = match env::var(host_key) {
        Ok(val) => val,
        Err(err) => {
            println!("{}: {}", err, host_key);
            process::exit(1);
        }
    };

    let port = match env::var(port_key) {
        Ok(val) => match val.parse::<u16>() {
            Ok(port) => port,
            Err(_) => {
                println!(
                    "the port number \"{}\" is invalid. default port will be used.",
                    val
                );
                default_port
            }
        },
        Err(_) => {
            println!(
                "\"{}\" is not defined in environment variables. default port will be used.",
                port_key
            );
            default_port
        }
    };

    assert_eq!(port, 8080);
    assert_eq!(host, "myserver");
}
