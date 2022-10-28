use std::{io::Write, net::TcpListener, thread::sleep, time::Duration};

use threadpool::ThreadPool;

fn main() {
    let listener = TcpListener::bind("127.0.0.1:1234").unwrap();
    let mut pool = ThreadPool::new(3);

    for stream in listener.incoming() {
        let mut stream = stream.unwrap();
        println!("Received stream!");
        pool.execute(move || {
            sleep(Duration::from_millis(5000));
            stream
                .write_all("HTTP/1.1 200 OK\r\n\r\nHello world!".as_bytes())
                .unwrap();
        });
    }
}
