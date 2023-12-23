use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use tokio::sync::mpsc;
use tokio::task;
use tokio::time::{self, Duration};

struct Large {
    id: u64,
    blob: Vec<String>
}

async fn producer(tx: mpsc::Sender<Large>, i: u64) {
    loop {
        let signal = Large {
            id: i,
            blob: std::iter::repeat("Hello".to_owned()).take(100).collect()
        };
        tx.send(signal).await.unwrap_or_else(|e| println!("error! {} failed: {}", i, e));
        time::sleep(Duration::from_millis(10)).await;
    }
}

fn consumer(mut rx: mpsc::Receiver<Large>) {
    let mut counter = 0;
    while let Some(i) = rx.blocking_recv() {
        counter += i.id;
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
