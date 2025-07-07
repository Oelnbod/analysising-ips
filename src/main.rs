mod ip_manipulation;


use std::fs;
use ip_manipulation::*;

fn main() {
    let contents = fs::read_to_string("collected.txt").expect("Error reading file");
    let recorded_ips: Vec<&str> = contents.lines().collect();
    let domains_found = reverse_dns(recorded_ips);
    let deduped: Vec<String> = remove_duplicates(domains_found);
    print_list(deduped);
}
