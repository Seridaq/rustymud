mod protocol;
use protocol::Telnet;

fn main() {
    print!("Hello from rust! This is SE: {}", Telnet::SE);
}
