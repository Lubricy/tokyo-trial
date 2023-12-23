use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use tokio::sync::mpsc;
use tokio::task;
use tokio::time::{self, Duration};
async fn producer(tx: mpsc::Sender<u64>, i: u64) {
    loop {
        tx.send(i).await.unwrap_or_else(|e| println!("error! {} failed: {}", i, e));
        time::sleep(Duration::from_millis(10)).await;
    }
}

fn consumer(mut rx: mpsc::Receiver<u64>) {
    let mut counter = 0;
    while let Some(i) = rx.blocking_recv() {
        counter += i;
    }
}

#[tokio::main]
async fn main() {
    let map: Arc<HashMap<u64, u64>> = Arc::new(HashMap::new());
    let (tx, rx) = mpsc::channel(10);

    for i in 0..100 {
        let tx = tx.clone();
        task::spawn(producer(tx, i));
    }

    std::thread::spawn(move || consumer(rx));

    println!("start");
    time::sleep(Duration::from_millis(1000000)).await;


}
