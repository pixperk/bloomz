use core::hash::{BuildHasher, Hash, Hasher};

pub fn hash2<T : Hash, S : BuildHasher>(state : &S, item : &T) -> (u64, u64){
    
    
    let v1 = state.hash_one(item);

    let mut h2 = state.build_hasher();
    v1.hash(&mut h2);
    item.hash(&mut h2);
    let mut v2 = h2.finish();

    // ensure v2 is odd to avoid pathological cycles in some corner cases
    if v2 & 1 == 0{
        v2 |= 1;
    }

    (v2, v2)
}