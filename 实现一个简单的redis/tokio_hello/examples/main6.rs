use std::time::Instant;
use tokio::task;
use tokio::time::{Duration, sleep};
#[tokio::main(flavor = "current_thread")]
async fn main() {
    let now = Instant::now();

    let regular_task = async move {
        println!("Regular task: Started,{}", now.elapsed().as_millis());
        sleep(Duration::from_millis(500)).await;
        println!("Regular task: Resumed,{}", now.elapsed().as_millis());
    };

    tokio::spawn(regular_task);

    let unconstrained_task = task::unconstrained(async {
        println!(
            "Unconstrained task: Before sleep,{}",
            now.elapsed().as_millis()
        );
        // ???
        sleep(Duration::from_secs(1)).await;
        println!(
            "Unconstrained task: After sleep,{}",
            now.elapsed().as_millis()
        );

        println!("Unconstrained task: Before yield_now");
        task::yield_now().await;
        println!("Unconstrained task: After yield_now");
    });

    unconstrained_task.await;
    println!("Main task complete");
}
