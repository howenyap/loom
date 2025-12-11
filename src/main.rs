use loom::server::Server;

fn main() {
    let server = Server::new();

    server.run();
}
