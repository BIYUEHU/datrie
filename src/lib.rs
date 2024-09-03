use std::collections::HashMap;

pub struct Dat<T: Copy> {
    base: Vec<i32>,
    check: Vec<i32>,
    tail: HashMap<i32, T>,
    code_map: HashMap<char, usize>,
}

impl<T: Copy> Default for Dat<T> {
    fn default() -> Self {
        Self {
            base: vec![1],
            check: vec![0],
            tail: HashMap::new(),
            code_map: HashMap::new(),
        }
    }
}

impl<T: Copy> Dat<T> {
    pub fn new() -> Self {
        Self::default()
    }

    fn get_code(&mut self, ch: char) -> usize {
        let len = self.code_map.len();
        *self.code_map.entry(ch).or_insert_with(|| {
            let code = len + 1;
            code
        })
    }

    fn resize(&mut self, size: usize) {
        while self.base.len() < size {
            self.base.push(0);
            self.check.push(0);
        }
    }

    fn can_use_base(&mut self, base: usize, suffix: &str) -> bool {
        for ch in suffix.chars() {
            let ch_code = self.get_code(ch);
            let new_state = base + ch_code;

            if new_state < self.base.len()
                && (self.base[new_state] != 0 || self.check[new_state] != 0)
            {
                return false;
            }
        }
        true
    }

    fn relocate(&mut self, state: usize, old_base: usize, new_base: usize) {
        for (_, &code) in &self.code_map {
            let old_state = old_base + code;
            let new_state = new_base + code;

            if old_state < self.base.len() && self.check[old_state] == state as i32 {
                while self.base.len() < (new_state + 1) {
                    self.base.push(0);
                    self.check.push(0);
                }
                self.base[new_state] = self.base[old_state];
                self.check[new_state] = state as i32;
                for j in 0..self.base.len() {
                    if self.check[j] == old_state as i32 {
                        self.check[j] = new_state as i32;
                    }
                }
                self.base[old_state] = 0;
                self.check[old_state] = 0;
            }
        }
        self.base[state] = new_base as i32;
    }

    pub fn append(&mut self, key: &str, value: T) {
        let mut state = 0;

        for (i, ch) in key.chars().enumerate() {
            let ch_code = self.get_code(ch);
            let new_state = (self.base[state].abs() as usize) + ch_code;
            self.resize(new_state + 1);

            if self.base[new_state] == 0 && self.check[new_state] == 0 {
                self.base[new_state] = self.base.len() as i32;
                self.check[new_state] = state as i32;
            } else if self.check[new_state] != state as i32 {
                let old_base = self.base[state].abs() as usize;
                let mut new_base = 1;

                while !self.can_use_base(new_base, &key[i..]) {
                    new_base += 1;
                }

                self.relocate(state, old_base, new_base);
                let new_state = new_base + ch_code;
                self.resize(new_state + 1);
                self.base[new_state] = self.base.len() as i32;
                self.check[new_state] = state as i32;
            }
            state = new_state;
        }

        let tail_key = -(self.tail.len() as i32) - 1;
        self.tail.insert(tail_key, value);
        self.base[state] = tail_key;
    }

    pub fn load(&mut self, list: Vec<(&str, T)>) {
        for (key, value) in list {
            self.append(key, value);
        }
    }

    pub fn search<'a>(&self, key: &str) -> Vec<(String, T)> {
        let mut state = 0;
        let mut results = Vec::new();
        let mut current_key = String::new();

        for ch in key.chars() {
            current_key.push(ch);

            if let Some(&ch_code) = self.code_map.get(&ch) {
                let new_state = (self.base[state].abs() as usize) + ch_code;
                if new_state >= self.base.len() || self.check[new_state] != state as i32 {
                    break;
                }
                state = new_state;
            } else {
                break;
            }

            if self.base[state] < 0 {
                if let Some(value) = self.tail.get(&self.base[state]) {
                    results.push((current_key.clone(), *value));
                }
            }
        }

        results
    }

    pub fn lookup(&self, key: &str) -> Option<T> {
        let mut state = 0;

        for ch in key.chars() {
            if let Some(&ch_code) = self.code_map.get(&ch) {
                let new_state = (self.base[state].abs() as usize) + ch_code;
                if new_state >= self.base.len() || self.check[new_state] != state as i32 {
                    return None;
                }
                state = new_state;
            } else {
                return None;
            }

            if self.base[state] < 0 && ch == key.chars().last().unwrap() {
                if let Some(value) = self.tail.get(&self.base[state]) {
                    return Some(*value);
                }
            }
        }

        None
    }

    pub fn contain(&self, key: &str) -> bool {
        self.lookup(key).is_some()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_dat() {
        let dat: Dat<i32> = Dat::new();
        assert_eq!(dat.base.len(), 1);
        assert_eq!(dat.check.len(), 1);
        assert_eq!(dat.tail.len(), 0);
        assert_eq!(dat.code_map.len(), 0);
    }

    #[test]
    fn test_append_and_lookup() {
        let mut dat: Dat<i32> = Dat::new();
        dat.append("key1", 1);
        dat.append("key2", 2);

        assert_eq!(dat.lookup("key1"), Some(1));
        assert_eq!(dat.lookup("key2"), Some(2));
        assert_eq!(dat.lookup("key3"), None);
    }

    #[test]
    fn test_search() {
        let mut dat: Dat<i32> = Dat::new();
        dat.append("key", 1);
        dat.append("key", 1);
        dat.append("key1", 2);

        let results = dat.search("key1");
        assert!(results.contains(&("key".to_string(), 1)));
        assert!(results.contains(&("key1".to_string(), 2)));
        assert_eq!(results.len(), 2);
    }

    #[test]
    fn test_contain() {
        let mut dat: Dat<i32> = Dat::new();
        dat.append("key1", 1);

        assert!(dat.contain("key1"));
        assert!(!dat.contain("key2"));
    }

    #[test]
    fn test_load() {
        let mut dat: Dat<i32> = Dat::new();
        let list = vec![("key1", 1), ("key2", 2)];
        dat.load(list);

        assert_eq!(dat.lookup("key1"), Some(1));
        assert_eq!(dat.lookup("key2"), Some(2));
    }
}
