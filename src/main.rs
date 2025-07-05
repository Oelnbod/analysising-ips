use dns_lookup::lookup_addr;
use std::fs;


fn main() {
    let contents = fs::read_to_string("collected.txt").expect("Error reading file");
    let recorded_ips: Vec<&str> = contents.lines().collect();
    reverse_dns(recorded_ips);
    let domain= "143-42-1-123.ip.linodeusercontent.com";
    println!("{}", shrink_domain(domain))

}
fn reverse_dns(recorded_ips: Vec<&str>) {
    for items in recorded_ips {
        match items.parse::<std::net::IpAddr>() {
            Ok(ip) => match lookup_addr(&ip) {
                Ok(host) => println!("{}", host),
                Err(_) => print!(""),
            },
            Err(_) => print!(""),
        }
    }
}

fn shrink_domain(domain: &str) -> String {
    let parts: Vec<&str> = domain.split(".").collect();

    if parts.len() >= 3 {
        let output =  (&parts[parts.len()- 3..]).join(".");
        return output
    } else if parts.len() == 2 {
        let output =  (&parts[parts.len()- 2..]).join(".");
        return output
    } else {
        let output =  (&parts[parts.len()- 2..]).join(".");
        return output
    }
}