use tokio::runtime::Handle;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    use std::sync::Arc;
    use tokio::sync::Barrier;
    #[inline(never)]
    async fn a(barrier:Arc<Barrier>) {
        b(barrier).await
    }
    #[inline(never)]
    async fn b(barrier: Arc<Barrier>){
        c(barrier).await
    }

    #[inline(never)]
    async fn c(barrier: Arc<Barrier>){
        barrier.wait().await;
    }
    async fn dump_or_quit() {
        use tokio::time::{timeout, Duration, Instant};
        let handle = tokio::runtime::Handle::current();
        let mut last_signal: Option<Instant> = None;
        while let Ok(_) = tokio::signal::ctrl_c().await {
            // exit if a CTRL_C is inputed twice within 1 second
            if let Some(time_since_last_singla) = last_signal.map(|i| i.elapsed()) {
                if time_since_last_singla < Duration::from_secs(1) {
                    return ;
                }
                last_signal = Some(Instant::now());
                // capture a dump, and print each trac
                println!("{:<80}", "");
                if let Ok(dump) = timeout(Duration::from_secs(2),handle.dump()).await {
                    for task in dump.task().iter() {
                        let id = task.id();
                        let trace = task.trace();
                        println!("TASK {id}:");
                        println!("{trace}\n");
                    }
                }else {
                    println!("dumping timed out")
                }
            }
            println!("{:-<80}", "");
            println!("Input CTRL+C twice within 1 second to exit.");
        }
    }
    println!("This program has a deadlock.");
    println!("Input CTRL+C to print a task dump.");
    println!("Input CTRL+C twice within 1 second to exit.");

    // oops! this barrier waits for one more task than will ever come.
    let barrier = Arc::new(Barrier::new(3));

    let task_1 = tokio::spawn(a(barrier.clone()));
    let task_2 = tokio::spawn(a(barrier));

    tokio::select!(
        _ = dump_or_quit() => {},
        _ = task_1 => {},
        _ = task_2 => {},
    );

    Ok(())
}

// #[cfg(not(all(
//     tokio_unstable,
//     tokio_taskdump,
//     target_os = "linux",
//     any(target_arch = "aarch64", target_arch = "x86", target_arch = "x86_64")
// )))]
// fn main() {
//     println!("task dumps are not available")
// }