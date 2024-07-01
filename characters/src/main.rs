fn main() {
    println!("Hello, world!");
}

// we go over 14 characters at a time, collect them in a hashset and check its size
fn simple(i: &[u8]) -> usize {
    return i.windows(14)
        .position(|w| {
            return w.iter().collect::<HashSet<_>>().len() == 14;
        })
        .map(|x| x + 14)
        .unwrap();
}

// we insert elements into the hashset one at a time rather than all at once
fn faster(i: &[u8]) -> usize {
    return i.windows(14)
        .position(|w| {
            let mut hash_set = HashSet::new();
            for x in w {
                if !hash_set.insert(x) {
                    return false;
                }
            }
            return true;
        })
    .map(|x| x + 14)
    .unwrap();
}

// we use a vector instead of a hashset because we're dealing with small values
fn faster_vec(i: &[u8]) -> usize {
    return i.windows(14)
        .position(|w| {
            let mut vec = Vec::with_capacity(14);
            for x in w {
                if vec.contains(x) {
                    return false;
                }
                vec.push(*x);
            }
            return true;
        })
        .map(|x| x + 14)
        .unwrap();
}

// we use an array instead of a vector as it's stack allocated
fn faster_arr(i: &[u8]) -> usize {
    return i.windows(14)
        .position(|w| {
            let mut arr = [0u8; 14];
            let mut idx = 0;
            for x in w {
                for i in 0..idx {
                    if arr[i] == *x {
                        return false;
                    }
                }
                arr[idx] = *x;
                idx += 1;
            }
            return true;
        })
        .map(|x| x + 14)
        .unwrap();
}
