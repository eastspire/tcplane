use crate::*;

impl<T> ErrorHandle for T where T: Fn(String) {}

impl<F> Func for F where
    F: Fn(Context) -> Pin<Box<dyn Future<Output = ()> + Send + 'static>> + Send + Sync + 'static
{
}

impl<F, Fut> AsyncFuncWithoutPin<Fut> for F
where
    F: Fn(Context) -> Fut + Send + Sync + 'static,
    Fut: Future<Output = ()> + Send + 'static,
{
}
