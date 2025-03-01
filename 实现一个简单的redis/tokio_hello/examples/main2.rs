use tokio::time::sleep;

#[tokio::main]
async fn main() {
    let data = vec![1, 2, 3];
    let data_ref = &data; // Borrow the data

    tokio::task::spawn(async move {
        let mut interval = tokio::time::interval(std::time::Duration::from_millis(200));
        loop {
            interval.tick().await;
            println!("tick");
        }
    });

    tokio::task::unconstrained(async {
        sleep(std::time::Duration::from_secs(2)).await;
        // Access the borrowed data here
        println!("Data inside task: {:?}", data_ref);
    })
    .await;

    println!("Final data: {:?}", data); // Still accessible here
}
