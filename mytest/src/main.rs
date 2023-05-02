use std::thread::{self, JoinHandle};

static mut DATA: u64 = 0;
static mut READY: bool = false;

fn reset() {
    unsafe {
        DATA = 0;
        READY = false;
    }
}

fn producer() -> JoinHandle<()> {
    thread::spawn(move || {
        unsafe {
            DATA = 100; // A
            READY = true; // B
        }
    })
}

fn consumer() -> JoinHandle<()> {
    thread::spawn(move || {
        unsafe {
            while !READY {} // C

            assert_eq!(100, DATA); // D
        }
    })
}

fn main() {
    loop {
        reset();

        let t_producer = producer();
        let t_consumer = consumer();

        t_producer.join().unwrap();
        t_consumer.join().unwrap();
    }
}
