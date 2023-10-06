use core::fmt::Debug;

pub struct Network {
    pub start_ip: [u8; 4],
    pub end_ip: [u8; 4],
    pub broadcast: [u8; 4],
    pub netmask: [u8; 4],
    pub prefix: u32,
}

impl Network {
    pub fn new(initial_ip: &[u8; 4], amount_of_hosts: u32) -> Self {

        let netmask = hosts_to_netmask(amount_of_hosts);
        let prefix = netmask_to_prefix(&netmask);

        if initial_ip.iter().enumerate().any(|(i, v)| v & !netmask[i] != 0) {
            panic!("ERROR: Invalid initial IP given.");
        }
        
        let last_ip = last_ip_from_netmask(initial_ip, &netmask);
        let broadcast = next_ip(&last_ip);

        Self {
            start_ip: initial_ip.to_owned(),
            end_ip: last_ip,
            broadcast,
            netmask,
            prefix,
        }
    }
}

impl Debug for Network {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Address Space:\n\tFrom: {}/{}\n\tTo: {}/{}\n\tBroadcast: {}\n\tNetmask: {}",
            slice_to_string(&self.start_ip),
            self.prefix,
            slice_to_string(&self.end_ip),
            self.prefix,
            slice_to_string(&self.broadcast),
            slice_to_string(&self.netmask)
        )
    }
}

// Taken in a netmask and returns the network prefix
fn netmask_to_prefix(n: &[u8; 4]) -> u32 {
    n.iter().map(|d| d.count_ones()).sum()
}

// Takes in a network prefix and returns a netmask
fn prefix_to_netmask(n: u32) -> [u8; 4] {
    let mut mixed: u32 = 0;

    for _ in 0..n {
        mixed = (mixed >> 1) | 0x80000000;
    }

    mixed.to_be_bytes()
}

// Takes in an amount of hosts you want to allocate and returns the netmask that makes it possible
fn hosts_to_netmask(n: u32) -> [u8; 4] {
    let mut power: u32 = 0;

    for i in 0..32 {
        if 2_u32.pow(i) >= n {
            power = i;
            break;
        }
    }

    prefix_to_netmask(32 - power)
}

// Takes in the first IP of the network and netmask and returns the broadcast address
fn ip_netmask_to_broadcast(ip: &[u8; 4], netmask: &[u8; 4]) -> [u8; 4] {
    let mut res = [0; 4];

    for i in 0..4 {
        res[i] = ip[i] | !netmask[i]
    }

    res
}

// Takes in the first IP of the network and netmask and returns the last IP that can be allocated
fn last_ip_from_netmask(ip: &[u8; 4], netmask: &[u8; 4]) -> [u8; 4] {
    let mut last_ip = ip_netmask_to_broadcast(ip, netmask);
    last_ip[3] -= 1;

    last_ip
}

// Returns the next IP address
pub fn next_ip(ip: &[u8; 4]) -> [u8; 4] {
    (u32::from_be_bytes(ip.to_owned()) + 1).to_be_bytes()
}

// IP/Netmask to String
fn slice_to_string(sl: &[u8; 4]) -> String {
    sl.map(|d| d.to_string()).join(".")
}
