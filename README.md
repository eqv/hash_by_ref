# HashByRef

This is a small helper crate to use `Rc<T>` as hash keys where equality is supposed to be determined by the underlaying
reference identity (i.e. by the value of the pointer). It provides a type `HashByRef<T>` that can be used as key in the hashmap.


## Quick Start

```
hash_by_ref = "0.1.0"
```


```rust
    use std::collections::HashMap;
    use std::rc::Rc;
    use hash_by_ref::HashByRef;

    let r1 = Rc::new(1);
    let r2 = Rc::new(1);
    let r3 = r1.clone();
    let mut h = HashMap::new();
    h.insert(HashByRef::new(r1.clone()),1);
    h.insert(HashByRef::new(r2.clone()),2);
    assert_eq!(h[&HashByRef::new(r1.clone())], 1);
    assert_eq!(h[&HashByRef::new(r2.clone())], 2);
    assert_eq!(h[&HashByRef::new(r3.clone())], 1);

```
