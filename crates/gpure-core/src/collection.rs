use std::collections::{HashMap, HashSet};
use std::hash::Hash;

pub fn chunk<T>(slice: &[T], size: usize) -> Vec<Vec<&T>> {
    if size == 0 {
        return vec![];
    }
    slice.chunks(size).map(|c| c.to_vec()).collect()
}

pub fn uniq<T: Eq + Hash + Clone>(slice: &[T]) -> Vec<T> {
    let mut seen = HashSet::new();
    let mut result = Vec::new();
    for item in slice {
        if seen.insert(item.clone()) {
            result.push(item.clone());
        }
    }
    result
}

pub fn group_by<T, K, F>(slice: &[T], key_fn: F) -> HashMap<K, Vec<&T>>
where
    K: Eq + Hash,
    F: Fn(&T) -> K,
{
    let mut map = HashMap::new();
    for item in slice {
        let key = key_fn(item);
        map.entry(key).or_insert_with(Vec::new).push(item);
    }
    map
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_chunk_normal() {
        let data = vec![1, 2, 3, 4, 5];
        let result = chunk(&data, 2);
        assert_eq!(result.len(), 3);
        assert_eq!(result[0], vec![&1, &2]);
        assert_eq!(result[1], vec![&3, &4]);
        assert_eq!(result[2], vec![&5]);
    }

    #[test]
    fn test_chunk_zero_size() {
        let data = vec![1, 2, 3];
        let result = chunk(&data, 0);
        assert!(result.is_empty());
    }

    #[test]
    fn test_chunk_empty_slice() {
        let data: Vec<i32> = vec![];
        let result = chunk(&data, 2);
        assert!(result.is_empty());
    }

    #[test]
    fn test_uniq_preserves_order() {
        let data = vec![1, 2, 2, 3, 1, 4];
        let result = uniq(&data);
        assert_eq!(result, vec![1, 2, 3, 4]);
    }

    #[test]
    fn test_uniq_empty() {
        let data: Vec<i32> = vec![];
        let result = uniq(&data);
        assert!(result.is_empty());
    }

    #[test]
    fn test_group_by_string_key() {
        let data = vec!["apple", "banana", "avocado", "blueberry"];
        let result = group_by(&data, |s| s.chars().next().unwrap());
        assert_eq!(result.get(&'a').unwrap().len(), 2);
        assert_eq!(result.get(&'b').unwrap().len(), 2);
    }

    #[test]
    fn test_group_by_empty() {
        let data: Vec<&str> = vec![];
        let result = group_by(&data, |s| *s);
        assert!(result.is_empty());
    }
}
