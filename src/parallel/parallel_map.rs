use rayon::prelude::*;

pub fn parallel_map<T, U, F>(input: &[T], func: F) -> Vec<U>
    where
        T: Sync,
        U: Send,
        F: Fn(&T) -> U + Sync,
{
    input.par_iter().map(func).collect()
}

pub fn parallel_map_mut<T, F>(input: &mut [T], func: F)
    where
        T: Send + Sync,
        F: Fn(&mut T) + Send + Sync,
{
    input.par_iter_mut().for_each(func);
}

pub fn parallel_filter<T, F>(input: &[T], predicate: F) -> Vec<T>
    where
        T: Sync + Clone,
        F: Fn(&T) -> bool + Sync,
{
    input.par_iter().filter(|&item| predicate(item)).cloned().collect()
}

pub fn parallel_filter_map<T, U, F>(input: &[T], func: F) -> Vec<U>
    where
        T: Sync,
        U: Send,
        F: Fn(&T) -> Option<U> + Sync,
{
    input.par_iter().filter_map(func).collect()
}

pub fn parallel_flat_map<T, U, F>(input: &[T], func: F) -> Vec<U>
    where
        T: Sync,
        U: Send,
        F: Fn(&T) -> Vec<U> + Sync,
{
    input.par_iter().flat_map(func).collect()
}

pub fn parallel_fold<T, U, F>(input: &[T], init: U, func: F) -> U
    where
        T: Sync,
        U: Send + Sync,
        F: Fn(U, &T) -> U + Sync,
{
    input.par_iter().fold(init, func)
}

pub fn parallel_reduce<T, F>(input: &[T], func: F) -> Option<T>
    where
        T: Send + Sync,
        F: Fn(T, T) -> T + Sync,
{
    input.par_iter().reduce(func)
}