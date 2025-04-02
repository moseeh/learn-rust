pub fn is_empty(v: &str) -> bool {
    v.len() == 0
}

pub fn is_ascii(v: &str) -> bool {
    v.chars().all(|c| c.is_ascii())
}

pub fn contains(v: &str, pat: &str) -> bool {
    if pat.is_empty() {
        return true;
    }
    
    if v.len() < pat.len() {
        return false;
    }
    
    for i in 0..=v.len() - pat.len() {
        if &v[i..i + pat.len()] == pat {
            return true;
        }
    }
    
    false
}

pub fn split_at(v: &str, index: usize) -> (&str, &str) {
    (&v[..index], &v[index..])
}

pub fn find(v: &str, pat: char) -> usize {
    for (i, c) in v.chars().enumerate() {
        if c == pat {
            return i;
        }
    }
    
    v.len()
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
