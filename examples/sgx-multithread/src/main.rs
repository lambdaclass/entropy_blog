use std::{thread};
use std::sync::{mpsc};

fn expensive_function(f: f64,sender: mpsc::Sender<(f64,f64)>) {
    let mut result = f;
    for _i in 1..100000000 {
        result = result.sin();
    }
    sender.send((f,result)).unwrap();
}

fn main() {

    let (sender, receiver) = mpsc::channel();
    
    for i in 0..4{
        let sender_n = sender.clone();
        thread::spawn(move || {
            println!("Thread #{i} started!");
            expensive_function(0.3*(i as f64),sender_n);
        });
    }

    for _i in 0..4 {
        match receiver.recv() {
            Ok((f1,f2)) => {
                println!("\nMain thread received a result");
                println!("Expensive function for: {} yields: {}", f1, f2);
            },
            Err(_) => panic!("Worker threads disconnected!"),
        }
    }
}
