use clap::Parser;
use std::net::UdpSocket;

#[derive(Parser)] 
struct Cli // for args
{
    // The pattern to look for
    pattern: String,
    // The path to the file to read
    // path: std::path::PathBuf,
}

fn main() -> std::io::Result<()>
{
    {
        // create socket on self
        let socket = UdpSocket::bind("127.0.0.1:4444").expect("Couldn't bind to address");

        //connect to victim server
        socket.connect("127.0.0.1:8080").expect("connect function failed");

        //send packets to server
        socket.send(&[0, 1, 2]).expect("couldn't send message");

        let args = Cli::parse();

        //println!("{:?}", &args.pattern);
        if &args.pattern == "poggers"
        {
            println!("Epic gamer moment");
        }
    } // socket closes here

    Ok(())
}
