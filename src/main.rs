use clap::Parser;
use net::IP;
use std::env;
use std::process::ExitCode;

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

fn main() -> ExitCode {
    let args = Args::parse();
    let argc = env::args().count();

    let ip: IP;

    match IP::try_from(&args.ip as &str) {
        Ok(ip_result) => {
            ip = ip_result;
        }
        Err(e) => {
            println!("Error: {}", e);
            return ExitCode::FAILURE;
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
            println!("Valor de subneteo incorrecto");
            return ExitCode::FAILURE;
        }
        let subnets: Vec<IP>;
        match ip.subnet(*prefix) {
            Ok(result) => subnets = result,
            Err(e) => {
                println!("Error: {}", e);
                return ExitCode::FAILURE;
            }
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
    ExitCode::SUCCESS
}
