mod network;

use network::{next_ip, Network};

fn main() {
    let initial_ip: [u8; 4] = [135, 30, 0, 0];

    let belfast = Network::new(&initial_ip, 3000);
    let galway = Network::new(&next_ip(&belfast.broadcast), 2000);
    let dublin = Network::new(&next_ip(&galway.broadcast), 1800);
    let cork = Network::new(&next_ip(&dublin.broadcast), 800);
    let letterkenny = Network::new(&next_ip(&cork.broadcast), 120);

    println!("Belfast: {:?}\n", belfast);
    println!("Galway: {:?}\n", galway);
    println!("Dublin: {:?}\n", dublin);
    println!("Cork: {:?}\n", cork);
    println!("Letterkenny: {:?}\n", letterkenny);
}
