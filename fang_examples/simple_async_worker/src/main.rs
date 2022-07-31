use fang::asynk::async_queue::AsyncQueue;
use fang::asynk::async_queue::AsyncQueueable;
use fang::asynk::async_worker_pool::AsyncWorkerPool;
use fang::AsyncRunnable;
use simple_async_worker::MyTask;
use std::time::Duration;
use tokio_postgres::NoTls;

#[tokio::main]
async fn main() {
    env_logger::init();

    log::info!("Starting...");

    let mut queue = AsyncQueue::connect("postgres://postgres:postgres@localhost/fang", NoTls, true)
        .await
        .unwrap();

    log::info!("Queue connected...");

    let mut pool = AsyncWorkerPool::builder()
        .queue(queue.clone())
        .number_of_workers(2 as u16)
        .build();

    log::info!("Pool created ...");

    pool.start().await;
    log::info!("Workers started ...");

    let task1 = MyTask::new(0);
    let task2 = MyTask::new(20_000);

    let metadata1 = serde_json::to_value(&task1 as &dyn AsyncRunnable).unwrap();
    let metadata2 = serde_json::to_value(&task2 as &dyn AsyncRunnable).unwrap();

    queue.insert_task(metadata1, "common").await.unwrap();
    queue.insert_task(metadata2, "common").await.unwrap();

    tokio::time::sleep(Duration::from_secs(100)).await;
}
