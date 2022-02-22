fn format_ipv4_address(address: &str) -> &str {
  let index = address.chars().position(|c| c == ':').unwrap();
  return &address[..index];
}

fn main() {
  let addrs = nix::ifaddrs::getifaddrs().unwrap();
  for ifaddr in addrs {
    match ifaddr.address {
      Some(address) => {
        if ifaddr.interface_name == "en0" && address.to_string().contains("192.168") {
          println!("{}", format_ipv4_address(&address.to_string()));
        }
      }
      None => {
        // println!("interface {} with unsupported address family", ifaddr.interface_name);
      }
    }
  }
}
