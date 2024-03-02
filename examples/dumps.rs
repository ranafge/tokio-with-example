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
        let handle = tokio::runtime::Handle::current()
    }

    Ok(())
}