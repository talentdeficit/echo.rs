use std::io::{TcpListener, TcpStream};
use std::io::{Acceptor, Listener};

fn main() {
  let mut acceptor = TcpListener::bind("127.0.0.1", 8000).listen();
  
  for stream in acceptor.incoming() {
    match stream {
      Err(_) => { break; }
      Ok(stream) => 
        spawn(proc() {
          handle_client(stream)
        })
    }
  }
}

fn handle_client(mut stream: TcpStream) {
  let read = stream.read_to_end().unwrap();
  let _ = stream.write(read.as_slice());
}

