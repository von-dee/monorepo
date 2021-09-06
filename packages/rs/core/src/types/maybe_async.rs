use futures::future::{Future, OptionFuture};

// MaybeAsync may not be necessary in Rust
pub type MaybeAsync<T> = OptionFuture<T>;

pub fn is_promise<T>(_test: T) -> bool {
    todo!()
}

pub async fn execute_maybe_async_function<T>(func: fn(args: &[T]) -> T, args: &[T]) -> T
where
    T: Future + Future<Output = T> + Copy + Clone,
{
    func(args)
}
