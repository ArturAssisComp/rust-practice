use std::{
    fs,
    io::{self, BufRead, BufReader, Write},
    net::{TcpListener, TcpStream},
    sync::{Arc, Mutex, mpsc},
    thread::{self, JoinHandle},
    time::Duration,
};

struct Worker {
    id: u8,
    thread: Option<JoinHandle<()>>,
}

impl Worker {
    fn new(id: u8, receiver: Arc<Mutex<mpsc::Receiver<Job>>>) -> Self {
        Self {
            id,
            thread: Some(thread::spawn(move || {
                loop {
                    let message = receiver.lock().unwrap().recv();
                    match message {
                        Ok(job) => {
                            println!("Worker {id} got a job; executing.");
                            job();
                        }
                        Err(_) => {
                            println!("Worker {id} disconnected; shutting down.");
                            break;
                        }
                    }
                }
            })),
        }
    }
}

type Job = Box<dyn FnOnce() + Send + 'static>;

struct ThreadPool {
    workers: Vec<Worker>,
    sender: Option<mpsc::Sender<Job>>,
}

impl ThreadPool {
    /// Create a new thread pool
    ///
    /// # Panics
    /// Panics if `total_threads` is 0.
    pub fn new(total_workers: u8) -> Self {
        assert!(
            total_workers > 0,
            "The total number of threads must be positive (> 0)."
        );

        let (sender, receiver) = mpsc::channel();
        let receiver = Arc::new(Mutex::new(receiver));
        let mut workers = Vec::with_capacity(total_workers.into());
        for i in 0..total_workers {
            workers.push(Worker::new(i, Arc::clone(&receiver)));
        }
        Self {
            workers,
            sender: Some(sender),
        }
    }

    pub fn execute<F>(&self, f: F)
    where
        F: FnOnce() + Send + 'static,
    {
        let job = Box::new(f);

        self.sender.as_ref().unwrap().send(job).unwrap();
    }
}

impl Drop for ThreadPool {
    fn drop(&mut self) {
        drop(self.sender.take());
        for worker in &mut self.workers {
            if let Some(thread) = worker.thread.take() {
                println!("Shutting down worker {}", worker.id);
                thread.join().unwrap();
            }
        }
    }
}

fn main() {
    let listener =
        TcpListener::bind("127.0.0.1:7878").expect("expected connection to the localhost:7878");

    let thread_pool = ThreadPool::new(4);
    for stream in listener.incoming().take(2) {
        match stream {
            Ok(stream) => {
                thread_pool.execute(move || handle_connection(stream));
            }
            Err(_) => {
                println!("connection failed");
            }
        }
    }
}

enum Status {
    Ok,
    NotFound,
}

impl Status {
    fn get_status_line(&self) -> &'static str {
        match self {
            Status::Ok => "HTTP/1.1 200 OK",
            Status::NotFound => "HTTP/1.1 404 NOT FOUND",
        }
    }
}
fn build_response(status: Status, body_file_path: &str) -> Result<String, io::Error> {
    let body = fs::read_to_string(body_file_path)?;

    Ok(format!(
        "{}\r\nContent-Length: {}\r\n\r\n{}",
        status.get_status_line(),
        body.len(),
        body
    ))
}
fn handle_connection(mut stream: TcpStream) {
    let buf_reader = BufReader::new(&mut stream);

    let request_line = buf_reader.lines().next().unwrap().unwrap();
    let response: String = match request_line.as_str() {
        "GET / HTTP/1.1" => build_response(Status::Ok, "projects/web_server/hello.html").unwrap(),
        "GET /sleep HTTP/1.1" => {
            thread::sleep(Duration::from_secs(3));
            build_response(Status::Ok, "projects/web_server/hello_async.html").unwrap()
        }

        _ => build_response(Status::NotFound, "projects/web_server/not_found.html").unwrap(),
    };

    match stream.write_all(response.as_bytes()) {
        Ok(_) => println!("Response was sent SUCCESSFULLY!"),
        Err(_) => println!("Response FAILED!"),
    }
}
