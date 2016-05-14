#[derive(Debug,PartialEq)]
pub struct BinaryTree<T : Ord> {
    data: T,
    left: Option<Box<BinaryTree<T>>>,
    right: Option<Box<BinaryTree<T>>>,
}

impl <T : Ord> BinaryTree<T> {
    pub fn leaf(data: T) -> BinaryTree<T> {
        return BinaryTree{ data: data, left: None, right: None };
    }
    pub fn left(data: T, left: BinaryTree<T>) -> BinaryTree<T> {
        return BinaryTree{ data: data, left: Some(Box::new(left)), right: None };
    }
    pub fn right(data: T, right: BinaryTree<T>) -> BinaryTree<T> {
        return BinaryTree{ data: data, right: Some(Box::new(right)), left: None };
    }
    pub fn branch(data: T, left: BinaryTree<T>, right: BinaryTree<T>) -> BinaryTree<T> {
        return BinaryTree{ data: data, left: Some(Box::new(left)), right: Some(Box::new(right))};
    }

    pub fn contains(&self, value: T) -> bool {
        if value == self.data {
            return true;
        } else if value > self.data {
            return match self.right {
                Some(ref right) => (*right).contains(value),
                _ => false
            };
        } else if value < self.data {
            return match self.left {
                Some(ref left) => (*left).contains(value),
                _ => false
            };
        }

        false
    }

    pub fn insert(&mut self, value: T) {
        if value == self.data {
            return; // already in the set, no need to add it again. Or panic?
        } else if value > self.data {
            match self.right {
                Some(ref mut right) => right.insert(value),
                _ => self.right = Some(Box::new(BinaryTree::leaf(value)))
            };
        } else if value < self.data {
            match self.left {
                Some(ref mut left) => left.insert(value),
                _ => self.left = Some(Box::new(BinaryTree::leaf(value)))
            };
        }
    }

}

impl <T : Ord + Clone> BinaryTree<T> {
    pub fn from(data: &mut [T]) -> BinaryTree<T> {
        data.sort();
        return BinaryTree::from_sorted(data);
    }

    pub fn from_sorted(data: &[T]) -> BinaryTree<T> {
        let len = data.len();
        if len == 0 {
            panic!("cannot make a binary tree out of no items");
        }

        // integer division by 2
        let pivot = len >> 1;
        let mut tree = BinaryTree::leaf(data[pivot].clone());

        if pivot > 0 {
            tree.left = Some(Box::new(BinaryTree::from_sorted(&data[0..pivot])));
        }

        if pivot + 1 < data.len() {
            tree.right = Some(Box::new(BinaryTree::from_sorted(&data[(pivot + 1)..data.len()])));
        }

        return tree;
    }
}

#[cfg(test)]
mod tests {

    use BinaryTree;

    #[test]
    fn bt_leaf() {
        let bt = BinaryTree::leaf(5);
        assert_eq!(5, bt.data);
        assert_eq!(None as Option<Box<BinaryTree<_>>>, bt.left);
        assert_eq!(None as Option<Box<BinaryTree<_>>>, bt.right);
    }

    #[test]
    fn bt_leftonly() {
        let l = BinaryTree::leaf(1);
        let bt = BinaryTree::left(5, l);
        assert_eq!(5, bt.data);
        match bt.left {
            Some(btl) => assert_eq!(BinaryTree::leaf(1), *btl),
            _ => assert!(false)
        }
        assert_eq!(None as Option<Box<BinaryTree<_>>>, bt.right);
    }

    #[test]
    fn bt_rightonly() {
        let r = BinaryTree::leaf(10);
        let bt = BinaryTree::right(5, r);
        assert_eq!(5, bt.data);
        match bt.right {
            Some(btr) => assert_eq!(BinaryTree::leaf(10), *btr),
            _ => assert!(false)
        }
        assert_eq!(None as Option<Box<BinaryTree<_>>>, bt.left);
    }

    #[test]
    fn bt_branch() {
        let l = BinaryTree::leaf(1);
        let r = BinaryTree::leaf(10);
        let bt = BinaryTree::branch(5, l, r);
        assert_eq!(5, bt.data);
        match bt.left {
            Some(btl) => assert_eq!(BinaryTree::leaf(1), *btl),
            _ => assert!(false)
        }
        match bt.right {
            Some(btr) => assert_eq!(BinaryTree::leaf(10), *btr),
            _ => assert!(false)
        }
    }

    #[test]
    fn leaf_contains_true() {
        let bt = BinaryTree::leaf(5);
        assert!(bt.contains(5));
    }

    #[test]
    fn leaf_contains_false_going_right() {
        let bt = BinaryTree::leaf(5);
        assert!(!bt.contains(6));
    }

    #[test]
    fn leaf_contains_false_going_left() {
        let bt = BinaryTree::leaf(5);
        assert!(!bt.contains(4));
    }

    #[test]
    fn branch_contains_goes_left() {
        let l = BinaryTree::leaf(1);
        let bt = BinaryTree::left(5, l);
        assert!(bt.contains(1));
    }

    #[test]
    fn branch_contains_goes_right() {
        let l = BinaryTree::leaf(10);
        let bt = BinaryTree::right(5, l);
        assert!(bt.contains(10));
    }

    #[test]
    fn leaf_inserts_left() {
        let mut bt = BinaryTree::leaf(5);
        bt.insert(1);
        match bt.left {
            Some(btl) => assert_eq!(BinaryTree::leaf(1), *btl),
            _ => assert!(false)
        }
    }

    #[test]
    fn leaf_inserts_right() {
        let mut bt = BinaryTree::leaf(5);
        bt.insert(10);
        match bt.right {
            Some(btr) => assert_eq!(BinaryTree::leaf(10), *btr),
            _ => assert!(false)
        }
    }

    #[test]
    fn test_from_slice_is_searchable() {
        let mut arr = vec![];
        let mut arr_for_bt = [0; 80000];

        for i in 0..80000 {
            arr.push(i * 2);
            arr_for_bt[i] = i * 2;
        }

        let tree = BinaryTree::from_sorted(&arr_for_bt[..]);

        println!("running binary search");
        for i in 0..80000 {
            assert!(tree.contains(i * 2));
            assert!(!tree.contains(i * 2 + 1));
        }
        println!("done");

        println!("running naive search");
        for i in 0..160000 {
            let should_contain = i % 2 == 0;
            let mut contains = false;
            for i2 in &arr {
                if *i2 == i {
                    contains = true;
                    break;
                }
            }

            assert_eq!(should_contain, contains);
        }
        println!("done");
    }
}
