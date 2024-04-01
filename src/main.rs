use clap::Parser;
use core::panic;
use net::IP;
use std::env;

mod net;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    ip: String,

    #[arg(short, long)]
    prefix: Option<u8>,

    #[arg(short, long)]
    limit: Option<usize>,
}

fn main() {
    let args = Args::parse();
    let argc = env::args().count();

    let ip: IP;

    match IP::try_from(&args.ip as &str) {
        Ok(ip_result) => {
            ip = ip_result;
        }
        Err(e) => {
            panic!("Error: {}", e);
        }
    }

    if argc <= 2 {
        println!("--- {} ---", ip);
        println!("Subnet Mask: {}", ip.mask());
        println!("Network Address: {}", ip.net_addr());
        println!("First Host: {}", ip.first_host());
        println!("Last Host: {}", ip.last_host());
        println!("Broadcast: {}", ip.broadcast());
        println!("Wildcard: {}", ip.wildcard());
    }

    if let Some(prefix) = &args.prefix {
        if prefix <= ip.prefix() {
            panic!("Valor de subneteo incorrecto");
        }
        let subnets: Vec<IP>;
        match ip.subnet(*prefix) {
            Ok(result) => subnets = result,
            Err(e) => panic!("Error: {}", e),
        }

        let mut max_subnets_show = subnets.len();
        if let Some(limit) = &args.limit {
            max_subnets_show = *limit;
        }

        println!("--- {} ---", ip);
        for subnet in 0..max_subnets_show {
            println!("{}", subnets[subnet]);
        }
        println!("--- {} ---", ip);
    }
}
