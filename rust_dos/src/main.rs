use clap::{Arg, ArgAction, Command};
use std::net::UdpSocket;

fn main() -> std::io::Result<()>
{
    let matches = Command::new("Rust DoS")
        .about("DNS DoS cli")
        .version("1.0")
        .author("Raaven")
        .disable_help_flag(true)
        .arg(Arg::new("vicitm-ip")
            .short('V')
            .long("victim-ip")
            .help("Set the IP that will be flooded with packets.")
            .required(true))
        .arg(Arg::new("host-ip")
            .short('H')
            .long("host-ip")
            .help("Set the host IP to send packets."))
        .arg(Arg::new("help")
            .short('h')
            .long("help")
            .global(true)
            .action(ArgAction::Help)).get_matches();

    {
        println!("\nVictim-IP: {:?}\nHost-IP: {:?}", matches.get_one::<String>("attack-ip"), matches.get_one::<String>("host-ip"));

        // create socket on self
        let socket = UdpSocket::bind("127.0.0.1:4444").expect("Couldn't bind to address");

        //connect to victim server
        socket.connect("127.0.0.1:8080").expect("connect function failed");

        //send packets to server
        socket.send(&[0, 1, 2]).expect("couldn't send message");

        // let args = Cli::parse();

        //println!("{:?}", &args.pattern);
        // if &args.pattern == "ip"
        // {
        //     println!("The IP({:?}) will be flooded with port 53", &args.pattern);
        // }

        // let socket 
    } // socket closes here

    Ok(())
}
