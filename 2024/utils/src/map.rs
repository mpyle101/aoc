#[macro_export]
macro_rules! map {
    () => {
        ::std::collections::HashMap::new()
    };

    ($($key:expr => $value:expr),+ $(,)?) => {
        ::std::collections::HashMap::from([ $(($key, $value)),* ])
    };
}

#[cfg(test)]
mod test {
    use std::collections::HashMap;

    #[test]
    fn empty() {
        let m: HashMap<&str, u32> = map![];
        assert!(m.is_empty());
    }

    #[test]
    fn one_pair() {
        let m: HashMap<&str, u32> = map! {
            "a" => 1
        };
        assert!(!m.is_empty());
        assert_eq!(*m.get("a").unwrap(), 1);
    }

    #[test]
    fn two_pairs() {
        let m: HashMap<&str, u32> = map! {
            "a" => 1,
            "b" => 2,
        };
        assert!(!m.is_empty());
        assert_eq!(*m.get("a").unwrap(), 1);
        assert_eq!(*m.get("b").unwrap(), 2);
    }
}