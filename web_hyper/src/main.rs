extern crate hyper;

use hyper::Server;
use hyper::server::Request;
use hyper::server::Response;
use hyper::net::Fresh;

fn hello(req: Request, res: Response<Fresh>) {
    // println!("Got request");
    let msg = format!("Hello, I don't know you, but you are asking for {:?}, right?", req.uri).into_bytes();
    res.send(&msg).unwrap();
}

fn main() {
    let port = 3000;
    println!("Listening on port {}", port);
    let addr = ("127.0.0.1", port);
    Server::http(addr).unwrap().handle(hello).unwrap();
}
