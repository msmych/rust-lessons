use hello::ThreadPool;
use rand::Rng;
use rayon::prelude::*;
use std::{
    env, fs,
    io::{prelude::*, BufReader},
    net::{TcpListener, TcpStream},
    sync::{Arc, Mutex},
    thread,
    time::Duration,
};

fn main() {
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();
    if env::var("USE_RAYON").is_ok() {
        start_with_rayon(listener);
    } else {
        start_with_custom_thread_pool(listener);
    }
}

fn start_with_custom_thread_pool(listener: TcpListener) {
    let pool = Arc::new(Mutex::new(ThreadPool::new(4)));

    let tick_pool = Arc::clone(&pool);

    thread::spawn(move || loop {
        tick_pool.lock().unwrap().execute(|| {
            tick();
        });
        thread::sleep(Duration::from_secs(1));
    });

    for stream in listener.incoming() {
        let stream = stream.unwrap();

        pool.lock().unwrap().execute(|| {
            handle_connection(stream);
        });
    }

    println!("Shutting down");
}

fn start_with_rayon(listener: TcpListener) {
    let pool = rayon::ThreadPoolBuilder::new()
        .num_threads(4)
        .build()
        .unwrap();

    pool.scope(|scope| {
        scope.spawn(|_| loop {
            tick();
            thread::sleep(Duration::from_secs(1));
        });

        scope.spawn(|_| {
            listener
                .incoming()
                .into_iter()
                .par_bridge()
                .for_each(|stream| handle_connection(stream.unwrap()))
        });
    });
}

fn handle_connection(mut stream: TcpStream) {
    let buf_reader = BufReader::new(&mut stream);
    let request_line = buf_reader.lines().next().unwrap().unwrap();

    let mut request_parts = request_line.split_whitespace();

    let method = request_parts.next().unwrap();
    let path = request_parts.next().unwrap();

    let (status_line, filename) = if method != "GET" && method != "HEAD" {
        ("HTTP/1.1 404 NOT FOUND", "404.html")
    } else {
        match path {
            "/" => ("HTTP/1.1 200 OK", "hello.html"),
            "/sleep" => {
                thread::sleep(Duration::from_secs(5));
                ("HTTP/1.1 200 OK", "hello.html")
            }
            _ => ("HTTP/1.1 404 NOT FOUND", "404.html"),
        }
    };

    let contents = fs::read_to_string(filename).unwrap();
    let length = contents.len();

    let response = format!("{status_line}\r\nContent-Length: {length}\r\n\r\n{contents}");

    stream.write_all(response.as_bytes()).unwrap();
}

fn tick() {
    println!("Tick {}", rand::thread_rng().gen_range(0..100));
}
