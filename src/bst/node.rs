use std::cmp;
use std::fmt;

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

////////////////////////
// "Static" functions //
////////////////////////

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