use std::fmt;
use std::cmp;

#[derive(Debug, Default)]
pub struct Bst<T> {
    pub root: Option<Box<Node<T>>>
}

impl<T: PartialOrd + Clone> Bst<T> {
	pub fn insert(&mut self, new_val: T) {
		let target_node = &mut self.root;
		match target_node {
			&mut Some(ref mut subnode) => {
				subnode.insert(new_val);
			},
			&mut None => {
				let new_node = Node::new(new_val);
				let boxed_node = Some(Box::new(new_node));
				*target_node = boxed_node;
			}
		}
	}

    pub fn height(&self) -> usize {
        match &self.root {
            &Some(ref node) => node.height(),
            &None => 0
        }
    }

    pub fn distance_to(&self, key: T) -> isize {
        match &self.root {
            &Some(ref node) => node.distance_to(key),
            &None => -1
        }
    }

    pub fn weighted_path_length(beta: &Vec<f32>, alpha: Vec<f32>) -> f32 {
        if beta.len() != alpha.len()+1 {
            panic!("Vectors must have equal length");
        }

        let n = alpha.len();
        let mut sum: f32 = 0.0;

        
        // for i in 1..n+1 {
        //     sum += beta[i] * (b[i] + 1) + alpha[i-1] * a[i-1];
        // }

        return sum
    }

    pub fn gather_heights(&self) -> (Vec<usize>, Vec<usize>) {
        let mut b: Vec<usize> = Vec::new();
        let mut a: Vec<usize> = Vec::new();

        match &self.root {
            &Some(ref node) => self.gather_heights_rec(&self.root, &mut b, &mut a),
            &None => {}
        }

        return (b, a)
    }

    fn gather_heights_rec(&self, boxed_node: &Option<Box<Node<T>>>, b: &mut Vec<usize>, a: &mut Vec<usize>) {
        match boxed_node {
            &Some(ref node) => {
                self.gather_heights_rec(&node.left, b, a);

                let dist = self.root.as_ref().unwrap().distance_to(node.val.clone());
                b.push(dist as usize);

                // Look for a left "dummy" leaf
                match &node.left {
                    &None => a.push((dist+1) as usize),
                    &Some(ref subnode) => {}
                }

                // Look for a right "dummy" leaf
                match &node.right {
                    &None => a.push((dist+1) as usize),
                    &Some(ref subnode) => {}
                }

                self.gather_heights_rec(&node.right, b, a);
            }
            &None => {}
        }
    }
}

#[derive(Debug, Default, PartialEq)]
pub struct Node<T> {
    pub val: T,
    pub left: Option<Box<Node<T>>>,
    pub right: Option<Box<Node<T>>>
}

impl<T: PartialOrd + Clone> Node<T> {
	pub fn new(new_val: T) -> Node<T> {
		Node {
			val: new_val,
			left: None,
			right: None
		}
	}

    pub fn insert(&mut self, new_val: T) {
    	if self.val == new_val {
    		return
    	}

    	let target_node = if new_val < self.val {
    		&mut self.left
    	} else {
    		&mut self.right
    	};

    	match target_node {
    		&mut Some(ref mut subnode) => { subnode.insert(new_val) }
    		&mut None => {
    			let new_node = Node::new(new_val);
    			let boxed_node = Some(Box::new(new_node));
    			*target_node = boxed_node;
    		}
    	}
    }

    pub fn height(&self) -> usize {
        let height_left = match &self.left {
            &Some(ref subnode) => subnode.height() + 1,
            &None => 0
        };

        let height_right = match &self.right {
            &Some(ref subnode) => subnode.height() + 1,
            &None => 0
        };

        return cmp::max(height_left, height_right)
    }

    pub fn distance_to(&self, key: T) -> isize {
        if key < self.val {
            match &self.left {
                &Some(ref subnode) => subnode.distance_to(key) + 1,
                &None => -1
            }
        } else if key > self.val {
            match &self.right {
                &Some(ref subnode) => subnode.distance_to(key) + 1,
                &None => -1
            }
        } else {
            return 0
        }
    }

    pub fn is_leaf(&self) -> bool {
        match (&self.left, &self.right) {
            (&None, &None) => true,
            _ => false
        }
    }
}

pub fn inorder_tree_walk<T: fmt::Debug>(boxed_node: &Option<Box<Node<T>>>) {
    match boxed_node {
        &Some(ref node) => {
            inorder_tree_walk(&node.left);
            println!("{:?}", node.val);
            inorder_tree_walk(&node.right);
        }
        &None => {}
    };
}

#[test]
fn empty_height() {
    let bst: Bst<i32> = Bst::default();
    assert_eq!(bst.height(), 0);
}

#[test]
fn height() {
    let mut bst: Bst<i32> = Bst::default();
    bst.insert(1);
    bst.insert(2);
    bst.insert(3);
    bst.insert(4);

    assert_eq!(bst.height(), 3);

    let mut bst2: Bst<i32> = Bst::default();
    bst2.insert(2);
    bst2.insert(1);
    bst2.insert(3);

    assert_eq!(bst2.height(), 1);
}

#[test]
fn leaf() {
    let mut bst: Bst<i32> = Bst::default();
    bst.insert(5);

    assert!(bst.root.as_ref().unwrap().is_leaf());

    bst.insert(6);

    assert!(!bst.root.as_ref().unwrap().is_leaf());
    assert!(bst.root.as_ref().unwrap().right.as_ref().unwrap().is_leaf());
}

#[test]
fn distance() {
    let mut tree: Bst<i32> = Bst::default();
    tree.insert(2);
    tree.insert(1);
    tree.insert(5);
    tree.insert(4);
    tree.insert(3);

    assert_eq!(tree.distance_to(1), 1);
    assert_eq!(tree.distance_to(2), 0);
    assert_eq!(tree.distance_to(3), 3);
    assert_eq!(tree.distance_to(4), 2);
    assert_eq!(tree.distance_to(5), 1);
}