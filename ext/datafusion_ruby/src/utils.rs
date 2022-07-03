use std::future::Future;
use tokio::runtime::Runtime;

pub fn wait_for_future<F: Future>(f: F) -> F::Output
where
    F: Send,
    F::Output: Send,
{
    let rt = Runtime::new().unwrap();
    rt.block_on(f)
}
