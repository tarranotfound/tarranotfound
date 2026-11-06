#[derive(Debug)]
enum ConnectionState {
    Disconnected,
    Connecting,
    Connected { address: String },
}

fn describe(state: ConnectionState) {
    match state {
        ConnectionState::Disconnected => println!("No connection"),
        ConnectionState::Connecting => println!("Connecting..."),
        ConnectionState::Connected { address } => println!("Connected to {address}"),
    }
}

fn main() {
    describe(ConnectionState::Disconnected);
    describe(ConnectionState::Connecting);
    describe(ConnectionState::Connected {
        address: String::from("127.0.0.1:8080"),
    });
}