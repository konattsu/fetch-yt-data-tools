use std::collections::VecDeque;

pub fn separate_vec_deque_result<T, U>(
    vec_deque_result: VecDeque<Result<T, U>>,
) -> (VecDeque<T>, VecDeque<U>)
where
    T: Clone,
    U: Clone,
{
    let result_ok =
        vec_deque_result.iter().filter_map(|a| a.as_ref().ok().cloned()).collect();
    let result_err =
        vec_deque_result.iter().filter_map(|a| a.as_ref().err().cloned()).collect();
    (result_ok, result_err)
}
