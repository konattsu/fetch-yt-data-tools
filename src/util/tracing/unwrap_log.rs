pub trait UnwrapLog<T> {
    fn unwrap_log(self) -> T;
}

impl<T, E> UnwrapLog<T> for Result<T, E>
where
    E: core::fmt::Debug,
{
    #[inline]
    fn unwrap_log(self) -> T {
        match self {
            Ok(val) => val,
            Err(e) => {
                let msg =
                    format!("called `Result::unwrap_log()` on an `Err` value, {e:?}");
                tracing::error!(msg);
                unwrap_failed(&msg);
            }
        }
    }
}

impl<T> UnwrapLog<T> for Option<T> {
    #[inline]
    fn unwrap_log(self) -> T {
        match self {
            Some(val) => val,
            None => {
                let msg = "called `Option::unwrap_log()` on a `None` value";
                tracing::error!(msg);
                unwrap_failed(msg);
            }
        }
    }
}

#[inline]
fn unwrap_failed(msg: &str) -> ! {
    panic!("{msg}");
}
