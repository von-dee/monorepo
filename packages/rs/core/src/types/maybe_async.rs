use futures::future::OptionFuture;

pub type MaybeAsync<T> = OptionFuture<T>;

pub fn is_promise<T>(_test: MaybeAsync<T>) -> bool {
    todo!()
}

pub async fn execute_maybe_async_function<T>(
    _func: fn(args: [T]) -> MaybeAsync<T>,
    _args: &[T],
) -> T {
    todo!()
}
