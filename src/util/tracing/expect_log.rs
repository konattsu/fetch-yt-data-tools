pub trait ExpectLog<T> {
    fn expect_log(self, msg: &str) -> T;
}

impl<T, E> ExpectLog<T> for Result<T, E>
where
    E: core::fmt::Debug,
{
    #[inline]
    fn expect_log(self, msg: &str) -> T {
        match self {
            Ok(val) => val,
            Err(e) => {
                tracing::error!("{msg}: {e:?}");
                unwrap_failed(msg, &e);
            }
        }
    }
}

impl<T> ExpectLog<T> for Option<T> {
    #[inline]
    fn expect_log(self, msg: &str) -> T {
        match self {
            Some(val) => val,
            None => {
                tracing::error!("{msg}");
                unwrap_failed(msg, &"None");
            }
        }
    }
}

#[inline]
fn unwrap_failed(msg: &str, error: &dyn core::fmt::Debug) -> ! {
    panic!("{msg}, {error:?}")
}
