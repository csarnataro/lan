/// Extract the IP address from a SockAddr, which is usually
/// in the form 0.0.0.0:0 (with a colon after the proper address)
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
      _ => {
        // do nothing in case of invalid address
      }
    }
  }
}
