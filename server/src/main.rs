mod server;
fn main() {
    server::Server::new().start(7878);
}