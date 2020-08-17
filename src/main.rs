use std::error::Error;
use std::net::SocketAddr;
use structopt::StructOpt;
use tokio::net::UdpSocket;

#[derive(Debug, StructOpt)]
#[structopt(
    name = "badnet",
    about = "A small proxy tool for testing UDP in bad conditions"
)]
struct Opt {
    /// Address to listen on
    listen_address: SocketAddr,

    /// Address to connect to
    target_address: SocketAddr,

    /// How many percent of packets should be dropped
    #[structopt(long = "loss", default_value = "0.0")]
    packet_loss_rate: f32,

    /// Print a debug line for each packet
    #[structopt(long)]
    debug: bool,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let opt = Opt::from_args();

    let mut socket = UdpSocket::bind(opt.listen_address).await?;

    let mut buf = [0u8; 65536];
    let mut client = None;
    loop {
        let (len, addr) = socket.recv_from(&mut buf).await?;
        if client.is_none() && addr != opt.target_address {
            client = Some(addr);
        }

        if addr == opt.target_address {
            if rand::random::<f32>() > opt.packet_loss_rate {
                if opt.debug {
                    println!("server -> client");
                }
                if let Some(client) = client {
                    socket.send_to(&mut buf[..len], client).await?;
                } else {
                    println!("no client yet, skipping");
                }
            } else {
                println!("server -> client (dropped)");
            }
        } else {
            if rand::random::<f32>() > opt.packet_loss_rate {
                if opt.debug {
                    println!("client -> server");
                }
                socket.send_to(&mut buf[..len], opt.target_address).await?;
            } else {
                println!("client -> server (dropped)")
            }
        }
    }
}
