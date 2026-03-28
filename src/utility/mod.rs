use std::collections::{HashMap, HashSet};
use std::hash::Hash;

pub(crate) fn clone_reduce<T: Clone>(arr: &[T], f: impl Fn(T, &T) -> T) -> Option<T> {
    let mut iter = arr.iter();
    let item = iter.next()?;
    Some(iter.fold(item.clone(), f))
}

pub(crate) fn flat_vec_hashmap<'a, 'b: 'a, K: Hash + Eq + Clone, V: Clone>(vec: &'b [HashMap<K, V>]) -> impl Iterator<Item = (usize, K, V)> + 'a {
    vec.iter().enumerate().flat_map(|(q, transition)| transition.iter().map(move |(a, p)| (q, a.clone(), p.clone())))
}

pub(crate) fn flat_vec_hashmap_hashset<'a, 'b: 'a, K: Hash + Eq + Clone, V: Clone>(vec: &'b [HashMap<K, HashSet<V>>]) -> impl Iterator<Item = (usize, K, V)> + 'a {
    vec.iter().enumerate().flat_map(|(q, transition)| transition.iter().flat_map(move |(a, p)| p.iter().map(move |p| (q, a.clone(), p.clone()))))
}

pub(crate) fn hashmap_of_unit_to_hashset<K: Hash + Eq + Clone>(hashmap: HashMap<K, ()>) -> HashSet<K> {
    hashmap.into_keys().collect()
}

pub(crate) fn hashset_of_unit_to_hashmap<K: Hash + Eq + Clone>(hashset: HashSet<K>) -> HashMap<K, ()> {
    hashset.into_iter().map(|k| (k, ())).collect()
}
