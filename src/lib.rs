use std::hash::{Hash, Hasher};
use std::rc::Rc;

#[derive(Debug)]
pub struct HashByRef<T>{
    val: Rc<T>,
}

impl <T> HashByRef<T>{
    pub fn new(val: Rc<T>) -> Self{
        return HashByRef{val}
    }
}

impl <T> Hash for HashByRef<T> {
  fn hash<H>(&self, state: &mut H) where H: Hasher {
      let ptr = Rc::into_raw(self.val.clone());
      ptr.hash(state);
      let _ = unsafe{ Rc::from_raw(ptr) };
  }
}

impl <T> PartialEq for HashByRef<T> {
    fn eq(&self, other: &Self) -> bool {
        return Rc::ptr_eq(&self.val, &other.val);
    }
}
impl <T> Eq for HashByRef<T> {}


#[cfg(test)]
mod tests {
    use ::HashByRef;
    use std::collections::HashMap;
    use std::rc::Rc;

    #[test]
    fn test_hash_by_ref() {
        let r1 = Rc::new(1);
        let r2 = Rc::new(1);
        let r3 = r1.clone();
        let mut h = HashMap::new();
        h.insert(HashByRef::new(r1.clone()),1);
        h.insert(HashByRef::new(r2.clone()),2);
        assert_eq!(h[&HashByRef::new(r1.clone())], 1);
        assert_eq!(h[&HashByRef::new(r2.clone())], 2);
        assert_eq!(h[&HashByRef::new(r3.clone())], 1);
    }

    #[test]
    fn test_mem_leak(){ //tests that the unsafe block that hashes the underlaying pointer doesn't leak references
        let r1 = Rc::new(1);
        {
            let mut h = HashMap::new();
            h.insert(HashByRef::new(r1.clone()),1);
            assert_eq!(h[&HashByRef::new(r1.clone())], 1);
        }
        assert_eq!(Rc::strong_count(&r1), 1);
        assert_eq!(Rc::weak_count(&r1), 0);
    }


    use std::hash::{Hash, Hasher};
    use std::collections::hash_map::DefaultHasher;
    fn calculate_hash<T: Hash>(t: &T) -> u64 {
        let mut s = DefaultHasher::new();
        t.hash(&mut s);
        s.finish()
    }

    #[test]
    fn test_hash(){
        let r1 = Rc::new(1);
        let r2 = Rc::new(1);
        //the assertion below might seldomly fail since two hashes might just turn out to be the same
        assert_ne!(calculate_hash(&HashByRef::new(r1.clone()) ), calculate_hash(&HashByRef::new(r2.clone()))); 
        //however this assertion must not fail
        assert_eq!(calculate_hash(&HashByRef::new(r1.clone()) ), calculate_hash(&HashByRef::new(r1.clone())));
    }

    #[test]
    fn test_eq(){
        let r1 = Rc::new(1);
        let r2 = Rc::new(1);
        assert_ne!(HashByRef::new(r1.clone()), HashByRef::new(r2.clone()));
        assert_eq!(HashByRef::new(r1.clone()), HashByRef::new(r1.clone()));
    }
}
