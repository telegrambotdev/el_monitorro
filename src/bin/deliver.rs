use dotenv::dotenv;
use el_monitorro;
use el_monitorro::bot::deliver_job;
use std::thread;

#[tokio::main]
async fn main() {
    dotenv().ok();
    env_logger::init();

    thread::spawn(|| deliver_job::deliver_updates())
        .join()
        .expect("Thread panicked")
}
