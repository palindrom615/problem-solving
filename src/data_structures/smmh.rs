// Symmetric Min-Max Heap
// S. Sahni, Double-ended priority queues, Data Structures and Applications, Chapman-Hall/CRC Press, 2005. Ed. D. Mehta and S. Sahni.

pub struct SMMH<T: Ord> {
    tree: Vec<Option<T>>,
}

impl<T: Ord> SMMH<T> {
    pub fn new() -> Self {
        SMMH { tree: vec![None] }
    }
    pub fn with_capacity(cap: usize) -> Self {
        let mut vec = Vec::with_capacity(cap);
        vec.push(None);
        SMMH { tree: vec }
    }
    pub fn len(&self) -> usize {
        self.tree.len() - 1
    }
    pub fn peek_max(&self) -> Option<&T> {
        match self.len() {
            0 => None,
            1 => self.tree[1].as_ref(),
            _ => self.tree[2].as_ref(),
        }
    }
    pub fn peek_min(&self) -> Option<&T> {
        match self.len() {
            0 => None,
            _ => self.tree[1].as_ref(),
        }
    }
    pub fn pop_max(&mut self) -> Option<T> {
        match self.len() {
            0 => None,
            1 | 2 => self.tree.pop().unwrap(),
            _ => {
                let min = self.tree.swap_remove(2);
                let mut pos = 2;
                loop {
                    let new_pos = p3_from_uncle(pos, &mut self.tree);
                    if pos == new_pos {
                        break;
                    }
                    pos = new_pos;
                }
                p1(pos, &mut self.tree);

                min
            }
        }
    }
    pub fn pop_min(&mut self) -> Option<T> {
        match self.len() {
            0 => None,
            1 => self.tree.pop().unwrap(),
            _ => {
                let min = self.tree.swap_remove(1);
                let mut pos = 1;
                loop {
                    let new_pos = p2_from_uncle(pos, &mut self.tree);
                    if pos == new_pos {
                        break;
                    }
                    pos = new_pos;
                }
                p1(pos, &mut self.tree);

                min
            }
        }
    }
    pub fn push(&mut self, elem: T) {
        let mut pos = self.tree.len();
        self.tree.push(Some(elem));
        loop {
            let mut new_pos = p1(pos, &mut self.tree);
            new_pos = p2(new_pos, &mut self.tree);
            new_pos = p3(new_pos, &mut self.tree);
            if new_pos == pos {
                break;
            }
            pos = new_pos;
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

fn p1<T: Ord>(idx: usize, tree: &mut Vec<Option<T>>) -> usize {
    if idx % 2 == 0 {
        if tree[idx - 1] < tree[idx] {
            return idx;
        }
        tree.swap(idx, idx - 1);
        return idx - 1;
    } else {
        if tree.get(idx + 1) == None || tree[idx + 1] > tree[idx] {
            return idx;
        }
        tree.swap(idx, idx + 1);
        return idx + 1;
    }
}
fn p2<T: Ord>(idx: usize, tree: &mut Vec<Option<T>>) -> usize {
    if idx <= 2 || tree[left_child(grand_parent(idx))] <= tree[idx] {
        return idx;
    }
    tree.swap(left_child(grand_parent(idx)), idx);
    left_child(grand_parent(idx))
}
fn p2_from_uncle<T: Ord>(idx: usize, tree: &mut Vec<Option<T>>) -> usize {
    let child = left_child(idx);
    let niece = left_child(idx + 1);
    if tree.get(child) == None
        || tree[idx] < tree[child] && (tree.get(niece) == None || tree[idx] < tree[niece])
    {
        return idx;
    }
    if tree.get(niece) == None || tree[child] <= tree[niece] {
        tree.swap(child, idx);
        return child;
    } else {
        tree.swap(niece, idx);
        return niece;
    }
}
fn p3<T: Ord>(idx: usize, tree: &mut Vec<Option<T>>) -> usize {
    if idx <= 2 || tree[right_child(grand_parent(idx))] >= tree[idx] {
        return idx;
    }
    tree.swap(right_child(grand_parent(idx)), idx);
    right_child(grand_parent(idx))
}
fn p3_from_uncle<T: Ord>(idx: usize, tree: &mut Vec<Option<T>>) -> usize {
    let child = right_child(idx);
    let niece = right_child(idx - 1);
    if tree.get(niece) == None
        || tree[idx] > tree[niece] && (tree.get(child) == None || tree[idx] > tree[child])
    {
        return idx;
    }
    if tree.get(child) == None || tree[child] < tree[niece] {
        tree.swap(niece, idx);
        return niece;
    } else {
        tree.swap(child, idx);
        return child;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
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
        dbg!(&s.tree);
        assert_eq!(s.pop_min(), Some(8));
        s.push(16);
        s.push(24);
        assert_eq!(s.pop_min(), Some(9));
        assert_eq!(s.pop_min(), Some(10));
    }
}
