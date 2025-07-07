use std::collections::HashSet;
use dns_lookup::lookup_addr;
use std::fs;


fn main() {
    let contents = fs::read_to_string("collected.txt").expect("Error reading file");
    let recorded_ips: Vec<&str> = contents.lines().collect();
    let domains_found = reverse_dns(recorded_ips);
    let deduped: Vec<String> = remove_duplicates(domains_found);
    println!("{:?}", deduped);
}
fn reverse_dns(recorded_ips: Vec<&str>) -> Vec<String> {
    let mut domains: Vec<String> = Vec::new();
    for items in recorded_ips {
        match items.parse::<std::net::IpAddr>() {
            Ok(ip) => match lookup_addr(&ip) {
                Ok(host) => {
                    let shrunk_host = shrink_domain(host.as_str());
                    domains.push(shrunk_host.clone());


                },
                Err(_) => (),
            },
            Err(_) => (),
        }
    };
    domains
}

fn shrink_domain(domain: &str) -> String{
    let parts: Vec<&str> = domain.split(".").collect();

    if parts.len() >= 3 {
        let output = (&parts[parts.len() - 3..]).join(".");
        output
    } else if parts.len() == 2 {
        let output = (&parts[parts.len() - 2..]).join(".");
        output
    } else {
        let output = (&parts[parts.len() - 2..]).join(".");
        output
    }
}
fn remove_duplicates(with_duplicates: Vec<String>) -> Vec<String> {
    let mut unique_elements = HashSet::new();
    let mut result: Vec<String> = Vec::new();
    for item in with_duplicates {
        if unique_elements.insert(item.clone()) {
            result.push(item);
        }
    }
    result
}