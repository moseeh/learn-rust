use std::collections::HashMap;
use std::hash::Hash;

pub fn slices_to_map<'a, T, U>(keys: &'a [T], values: &'a [U]) -> HashMap<&'a T, &'a U>
where
    T: Eq + Hash,
{
    // zip stops at the shortest slice automatically
    keys.iter().zip(values.iter()).collect()
}
