# datrie

Double array trie implementation of rust.

```rs
use datrie;

fn main() {
    let mut dat = datrie::Dat::new();
    dat.append("我", 1);
    dat.append("我喜欢", 2);
    dat.load(vec![("我喜欢你", 3), ("你不配", 4)]);

    assert_eq!(
        dat.search("我喜欢你"),
        vec![
            ("我".to_string(), 1),
            ("我喜欢".to_string(), 2),
            ("我喜欢你".to_string(), 3)
        ]
    );
    assert_eq!(dat.search("配！"), vec![]);
    assert_eq!(dat.lookup("我喜欢你"), Some(3));
    assert_eq!(dat.lookup("我喜欢她"), None);
    assert!(dat.contain("你不配"));
    assert!(!dat.contain("你配"));
}
```
