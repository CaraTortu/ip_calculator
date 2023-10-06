mod network;

use network::{next_ip, Network};
use std::io::{stdin, stdout, Write};
use std::str::FromStr;
use std::fmt::Debug;

// Python-like input function
fn input<T: FromStr>(str: &str) -> T where <T as FromStr>::Err: Debug {
    // Print statement
    print!("{}", str);
    stdout().flush().unwrap();

    // Return and parse input
    let mut s = String::new();
    stdin().read_line(&mut s).unwrap();

    s.trim_end().parse::<T>().unwrap()
}

// Get the initial IP from the user
fn get_initial_ip() -> [u8; 4] {
    let ip_string: String = input("Initial IP: "); 
    let initial_ip_vector: Vec<u8> = ip_string.split(".").map(|d| d.parse::<u8>().unwrap()).collect::<Vec<_>>();

    // Tranform to allow us to know total values
    // TODO: Verify if an IP is valid
    [initial_ip_vector[0], initial_ip_vector[1], initial_ip_vector[2], initial_ip_vector[3]]
}

fn get_names_and_hosts() -> Vec<(String, u32)> {
    let amount: u32 = input("How many cities do you need?: ");
    
    let mut names_and_hosts: Vec<(String, u32)> = Vec::new();

    for i in 0..amount {
        let name: String = input(&format!("What is the name of city number {}?: ", i));
        let hosts: u32 = input(&format!("How many hosts in {} do you need?: ", name));

        names_and_hosts.push((name, hosts));
    }

    names_and_hosts
}

fn main() {

    let mut initial_ip = get_initial_ip();
    let names_and_hosts = get_names_and_hosts();

    println!("\nLANs:\n");
    for i in 0..names_and_hosts.len() {
       let network = Network::new(&initial_ip, names_and_hosts[i].1);
       initial_ip = next_ip(&network.broadcast);
       println!(" {} {:?}\n", names_and_hosts[i].0, network);
    }

    println!("\nWANs\n");
    for i in 0..names_and_hosts.len() {
        let network = Network::new(&initial_ip, 4);
        initial_ip = next_ip(&network.broadcast);
        println!(" {}->{} WAN {:?}\n", names_and_hosts[i].0, names_and_hosts[(i+1) % names_and_hosts.len()].0, network)
    }

    // HALT UNTIL ENTER
    input::<String>("Press enter to quit: ");
}
