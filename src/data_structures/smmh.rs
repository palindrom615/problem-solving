// Symmetric Min-Max Heap
// S. Sahni, Double-ended priority queues, Data Structures and Applications, Chapman-Hall/CRC Press, 2005. Ed. D. Mehta and S. Sahni.

use std::ops::Index;

pub struct SMMH<T: Ord> {
    tree: Vec<T>,
    size: usize,
}

impl<T: Ord> Index<usize> for SMMH<T> {
    type Output = T;
    fn index(&self, i: usize) -> &Self::Output {
        &self.tree[i - 1]
    }
}

impl<T: Ord> SMMH<T> {
    pub fn new() -> Self {
        SMMH {
            tree: Vec::new(),
            size: 0,
        }
    }
    pub fn with_capacity(cap: usize) -> Self {
        SMMH {
            tree: Vec::with_capacity(cap),
            size: 0,
        }
    }
    pub fn len(&self) -> usize {
        self.size
    }
    pub fn peek_max(&self) -> Option<&T> {
        match self.size {
            0 => None,
            1 => Some(&self[1]),
            _ => Some(&self[2]),
        }
    }
    pub fn peek_min(&self) -> Option<&T> {
        match self.size {
            0 => None,
            _ => Some(&self[1]),
        }
    }
    pub fn pop_max(&mut self) -> Option<T> {
        match self.size {
            0 => None,
            1 | 2 => {
                self.size -= 1;
                self.tree.pop()
            }
            _ => {
                self.size -= 1;

                let max = self.swap_remove(2);
                let mut pos = 2;
                loop {
                    let new_pos = self.p3_from_uncle(pos);
                    if pos == new_pos {
                        break;
                    }
                    pos = new_pos;
                }
                self.p1(pos);
                Some(max)
            }
        }
    }
    pub fn pop_min(&mut self) -> Option<T> {
        match self.size {
            0 => None,
            1 => {
                self.size -= 1;
                self.tree.pop()
            }
            _ => {
                self.size -= 1;

                let max = self.swap_remove(1);
                let mut pos = 1;
                loop {
                    let new_pos = self.p2_from_uncle(pos);
                    if pos == new_pos {
                        break;
                    }
                    pos = new_pos;
                }
                self.p1(pos);

                Some(max)
            }
        }
    }
    pub fn push(&mut self, elem: T) {
        self.tree.push(elem);
        self.size += 1;
        let mut pos = self.size;
        loop {
            let mut new_pos = self.p1(pos);
            new_pos = self.p2(new_pos);
            new_pos = self.p3(new_pos);
            if new_pos == pos {
                break;
            }
            pos = new_pos;
        }
    }
    fn swap(&mut self, idx1: usize, idx2: usize) {
        self.tree.swap(idx1 - 1, idx2 - 1)
    }
    fn swap_remove(&mut self, idx1: usize) -> T {
        self.tree.swap_remove(idx1 - 1)
    }
    fn get(&self, idx: usize) -> Option<&T> {
        if self.size >= idx {
            Some(&self[idx])
        } else {
            None
        }
    }
    fn p1(&mut self, idx: usize) -> usize {
        if idx % 2 == 0 {
            if self[idx - 1] < self[idx] {
                return idx;
            }
            self.swap(idx, idx - 1);
            return idx - 1;
        } else {
            if self.get(idx + 1) == None || self[idx + 1] > self[idx] {
                return idx;
            }
            self.swap(idx, idx + 1);
            return idx + 1;
        }
    }
    fn p2(&mut self, idx: usize) -> usize {
        if idx <= 2 || self[left_child(grand_parent(idx))] <= self[idx] {
            return idx;
        }
        self.swap(left_child(grand_parent(idx)), idx);
        left_child(grand_parent(idx))
    }
    fn p2_from_uncle(&mut self, idx: usize) -> usize {
        let child = left_child(idx);
        let niece = left_child(idx + 1);
        if self.get(child) == None
            || self[idx] < self[child] && (self.get(niece) == None || self[idx] < self[niece])
        {
            return idx;
        }
        if self.get(niece) == None || self[child] <= self[niece] {
            self.swap(child, idx);
            return child;
        } else {
            self.swap(niece, idx);
            return niece;
        }
    }
    fn p3(&mut self, idx: usize) -> usize {
        if idx <= 2 || self[right_child(grand_parent(idx))] >= self[idx] {
            return idx;
        }
        self.swap(right_child(grand_parent(idx)), idx);
        right_child(grand_parent(idx))
    }
    fn p3_from_uncle(&mut self, idx: usize) -> usize {
        let child = right_child(idx);
        let niece = right_child(idx - 1);
        if self.get(niece) == None
            || self[idx] > self[niece] && (self.get(child) == None || self[idx] > self[child])
        {
            return idx;
        }
        if self.get(child) == None || self[child] < self[niece] {
            self.swap(niece, idx);
            return niece;
        } else {
            self.swap(child, idx);
            return child;
        }
    }
}

fn left_child(idx: usize) -> usize {
    idx * 2 + 1
}
fn right_child(idx: usize) -> usize {
    idx * 2 + 2
}
fn grand_parent(idx: usize) -> usize {
    ((idx - 1) / 2 - 1) / 2
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_pop_empty() {
        let mut s = SMMH::<i32>::new();
        for i in 0..100 {
            s.push(i);
        }
        for _ in 0..100 {
            s.pop_max();
        }
        assert_eq!(s.pop_max(), None);
        assert_eq!(s.pop_min(), None);
    }
    #[test]
    fn test_pop_min() {
        let mut s = SMMH::new();
        s.push(2);
        s.push(3);
        s.push(5);
        s.push(7);
        assert_eq!(s.pop_min(), Some(2));
        s.push(4);
        s.push(6);
        s.push(10);
        s.push(14);
        assert_eq!(s.pop_min(), Some(3));
        s.push(9);
        s.push(15);
        s.push(21);
        assert_eq!(s.pop_min(), Some(4));
        s.push(8);
        s.push(12);
        s.push(20);
        s.push(28);
        assert_eq!(s.pop_min(), Some(5));
        s.push(25);
        s.push(35);
        assert_eq!(s.pop_min(), Some(6));
        s.push(18);
        s.push(30);
        assert_eq!(s.pop_min(), Some(7));
        assert_eq!(s.pop_min(), Some(8));
        s.push(16);
        s.push(24);
        assert_eq!(s.pop_min(), Some(9));
        assert_eq!(s.pop_min(), Some(10));
    }
    #[test]
    fn test_pop_max() {
        let mut s = SMMH::new();
        s.push(2);
        s.push(3);
        s.push(5);
        s.push(7);
        assert_eq!(s.pop_max(), Some(7));
        s.push(4);
        s.push(6);
        s.push(10);
        s.push(14);
        assert_eq!(s.pop_max(), Some(14));
        s.push(9);
        s.push(15);
        s.push(21);
        assert_eq!(s.pop_max(), Some(21));
        s.push(8);
        s.push(12);
        s.push(20);
        s.push(28);
        assert_eq!(s.pop_max(), Some(28));
        s.push(25);
        s.push(35);
        assert_eq!(s.pop_max(), Some(35));
        s.push(18);
        s.push(30);
        assert_eq!(s.pop_max(), Some(30));
        assert_eq!(s.pop_max(), Some(25));
        s.push(16);
        s.push(24);
        assert_eq!(s.pop_max(), Some(24));
        assert_eq!(s.pop_max(), Some(20));
    }
}
