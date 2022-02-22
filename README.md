# lan

A very simple program written in Rust to find the current IPv4 address of the local machine.

More or less equivalent to launch, from a Unix-like console, `ifconfig | grep 192.168`.

I wrote this to find the local IP of my machine (which changes often based on new devices connected to the local network) but most of all to do some simple experiments with Rust.

Internally it uses the `nix` crate available at https://docs.rs/nix/0.19.1/nix/index.html

## Installation
Clone this repo, then launch `cargo run` to see it in action on your machine.

## Contributing
Pull requests are welcome. For major changes, please open an issue first to discuss what you would like to change.

## License
[MIT](https://choosealicense.com/licenses/mit/)
