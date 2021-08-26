use futures::future::Future;

pub fn is_promise<T>(_test: T) -> bool {
    todo!()
}

pub async fn execute_maybe_async_function<T: Future + Future<Output = T> + Copy + Clone>(
    func: fn(args: &[T]) -> T,
    args: &[T],
) -> T {
    let mut result = func(args);
    if is_promise(result) {
        result = result.await;
    }
    result
}
