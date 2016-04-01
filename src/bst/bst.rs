use std::fmt;
use std::cmp;
use super::node::Node;

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

    pub fn weighted_path_length(&self, p: &Vec<f32>) -> f32 {
        let n = p.len();
        let depth = self.gather_heights();

        if n != depth.len() {
            panic!("Vectors must be same length!");
        }

        let mut sum: f32 = 1.0;

        for i in 1..n {
            sum += (depth[i] as f32) * p[i];
        }

        return sum
    }

    fn gather_heights(&self) -> Vec<usize> {
        let mut depth: Vec<usize> = Vec::new();

        // Push a dummy value at the front to make indexing easier later
        depth.push(0);

        match &self.root {
            &Some(ref node) => self.gather_heights_rec(&self.root, &mut depth),
            &None => {}
        }

        return depth
    }

    fn gather_heights_rec(&self, boxed_node: &Option<Box<Node<T>>>, depth: &mut Vec<usize>) {
        match boxed_node {
            &Some(ref node) => {
                self.gather_heights_rec(&node.left, depth);

                let dist = self.root.as_ref().unwrap().distance_to(node.val.clone());
                depth.push(dist as usize);

                self.gather_heights_rec(&node.right, depth);
            }
            &None => {}
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