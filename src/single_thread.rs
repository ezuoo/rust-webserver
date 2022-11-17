use std::fs::File;
use std::io::prelude::*;
use std::net::TcpListener;
use std::net::TcpStream;

pub fn single_thread_web_server() {
  let listener = TcpListener::bind("127.0.0.1:7878").unwrap();

  for stream in listener.incoming() {
    let stream = stream.unwrap();
    handle_connection(stream);
  }
}

fn handle_connection(mut stream: TcpStream) {
  let mut buffer = [0; 512];
  let get = b"GET / HTTP/1.1\r\n";
  stream.read(&mut buffer).unwrap();

  // println!("Request: {}", String::from_utf8_lossy(&buffer[..]));
  println!("GET : {}", String::from_utf8_lossy(&get[..]));
  println!("GET : {:?}", get);
  println!("GET : {:?}", b"\r\n");

  let (status_line, filename) = if buffer.starts_with(get) {
    ("HTTP/1.1 200 OK", "hello.html")
  } else {
    ("HTTP/1.1 404 NOT FOUND", "404.html")
  };

  let mut file = File::open(filename).unwrap();

  let mut contents = String::new();
  file.read_to_string(&mut contents).unwrap();

  let response = format!(
    "{}\r\nContent-Length: {}\r\n\r\n{}",
    status_line,
    contents.len(),
    contents
  );
  stream.write(response.as_bytes()).unwrap();
  stream.flush().unwrap();
}
