use min_heap::MinHeap;
use std::collections::BinaryHeap;
use std::collections::HashMap;


struct RevOrd<T> (pub T);

pub struct Sieve {
    i: u32,
    heap: MinHeap<u32>,
    map: HashMap<u32, Vec<u32>>,
}

impl Sieve {
    fn new() -> Sieve {
        Sieve {
            i: 1,
            heap: MinHeap::new(),
            map: HashMap::new(),
        }
    }

    fn cross_out_next_multiple(&mut self, p: u32) {
        let next_cross = self.i + p;
        let already_crossed = self.map.contains_key(&next_cross);
        if already_crossed {
            self.map.get_mut(&next_cross).unwrap().push(p);
        } else {
            self.heap.push(next_cross);
            self.map.insert(next_cross, vec![p]);
        }
    }

    fn loop_f(&mut self, n: u32) {
        let x = self.heap.pop();
        let v = self.map.remove(&n).unwrap();

        for p in v {
            self.cross_out_next_multiple(p);
        }
    }
}


impl Iterator for Sieve {
    type Item = u32;
    
    fn next(&mut self) -> Option<u32> {
        self.i += 1;
        loop {
            let n;
            if let Some(next) = self.heap.peek() {
                n = next.clone();
            } else {
                break;
            }
            
            if n > self.i {
                break;
            }

            self.loop_f(n);

            if n == self.i {
                self.i += 1;
            }
        }
        let result = self.i; self.cross_out_next_multiple(result); 
        Some(result)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn Sieve_smalltest() {
        let mut s = Sieve::new();
        assert_eq!(s.next().unwrap(), 2);
        assert_eq!(s.next().unwrap(), 3);
        assert_eq!(s.next().unwrap(), 5);
        assert_eq!(s.next().unwrap(), 7);
        assert_eq!(s.next().unwrap(), 11);
    }
    
    #[test]
    fn Sieve_100test() {
        let mut s = Sieve::new();

        let result: Vec<u32> = s.take(100).collect();
        let answer = vec![
            2, 3, 5, 7, 11, 13, 17, 19, 23, 29,
            31, 37, 41, 43, 47, 53, 59, 61, 67, 71,
            73, 79, 83, 89, 97, 101, 103, 107, 109, 113,
            127, 131, 137, 139, 149, 151, 157, 163, 167, 173,
            179, 181, 191, 193, 197, 199, 211, 223, 227, 229,
            233, 239, 241, 251, 257, 263, 269, 271, 277, 281,
            283, 293, 307, 311, 313, 317, 331, 337, 347, 349,
            353, 359, 367, 373, 379, 383, 389, 397, 401, 409,
            419, 421, 431, 433, 439, 443, 449, 457, 461, 463,
            467, 479, 487, 491, 499, 503, 509, 521, 523, 541,
        ];

        assert_eq!(result, answer);
    }
}
