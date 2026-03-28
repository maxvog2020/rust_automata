pub(crate) fn clone_reduce<T: Clone>(arr: &[T], f: impl Fn(T, &T) -> T) -> Option<T> {
    let mut iter = arr.iter();
    let item = iter.next()?;
    Some(iter.fold(item.clone(), f))
}
