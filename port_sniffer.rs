use std::io::{self, Write};
use std::net::{IpAddr, TcpStream};
use std::process;
use std::str::FromStr;
use std::sync::mpsc::{channel, Sender};
use std::thread;
use std::{env, u16};

const MAX: u16 = 65535;

struct Arguments {
    flag: String,
    ipaddress: IpAddr,
    threads: u16,
}

impl Arguments {
    fn new(args: &[String]) -> Result<Arguments, &'static str> {
        if args.len() < 2 {
            return Err("Not enough arguments recieved");
        } else if args.len() > 4 {
            return Err("Too many arguments");
        }

        let ip = args[1].clone();
        if let Ok(ipaddress) = IpAddr::from_str(&ip) {
            return Ok(Arguments {
                flag: String::from(" "),
                ipaddress,
                threads: 4,
            });
        } else {
            let flag = args[1].clone();
            if flag.contains("-h") || flag.contains("-help") && args.len() == 2 {
                println!("Usage: -j to select how many threads you want -h or -help to show this help message");
                return Err("Help");
            } else if flag.contains("-h") || flag.contains("-help") {
                return Err("Too many arguments");
            } else if flag.contains("-j") {
                let ipaddress = match IpAddr::from_str(&args[3]) {
                    Ok(s) => s,
                    Err(_) => return Err("Not a valid Ipaddr must be Ipv4 or Ipv6"),
                };

                let threads = match args[2].parse::<u16>() {
                    Ok(s) => s,
                    Err(_) => return Err("Failed to parse thread number"),
                };
                return Ok(Arguments {
                    threads,
                    flag,
                    ipaddress,
                });
            } else {
                return Err("Invalid Syntax");
            }
        }
    }
}

fn scan(tx: Sender<u16>, start_port: u16, addr: IpAddr, num_threads: u16) {
    let mut port = start_port + 1;
    loop {
        match TcpStream::connect((addr, port)) {
            Ok(_) => {
                print!(".");
                io::stdout().flush().unwrap();
                tx.send(port).unwrap();
            }
            Err(_) => {}
        }

        if (MAX - port) <= num_threads {
            break;
        }

        port += num_threads;
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let program = args[0].clone();
    let arguments = Arguments::new(&args).unwrap_or_else(|err| {
        if err.contains("help") {
            process::exit(0);
        } else {
            eprintln!("{} problem passing arguments: {}", program, err);
            process::exit(0);
        }
    });

    let num_threads = arguments.threads;
    let addr = arguments.ipaddress;
    let (tx, rx) = channel();
    for i in 0..num_threads {
        let tx = tx.clone();
        thread::spawn(move || {
            scan(tx, i, addr, num_threads);
        });
    }

    let mut out = vec![];
    drop(tx);
    for p in rx {
        out.push(p);
    }

    println!("");
    out.sort();

    for add in out {
        println!("{} is open", add);
    }
}
