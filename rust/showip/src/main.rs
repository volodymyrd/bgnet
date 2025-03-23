use std::env;
use std::io;
use std::net::ToSocketAddrs;

fn main() -> io::Result<()> {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        eprintln!("usage: showip hostname");
        std::process::exit(1);
    }

    let hostname = &args[1];

    println!("IP addresses for {}:\n", hostname);

    // Use ToSocketAddrs to resolve the hostname to IP addresses.
    let addresses = (hostname.as_str(), 0).to_socket_addrs().map_err(|e| {
        io::Error::new(
            io::ErrorKind::Other,
            format!("Failed to resolve hostname: {}", e),
        )
    })?;

    for address in addresses {
        let ip = address.ip();
        println!("  {}: {}", if ip.is_ipv4() { "IPv4" } else { "IPv6" }, ip);
    }

    Ok(())
}
