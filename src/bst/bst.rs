use std::cmp;

#[derive(Debug, Default)]
pub struct Bst<T> {
    root: Option<Box<Node<T>>>
}

impl<T: PartialOrd> Bst<T> {
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

    pub fn weighted_path_length(&self) -> f32 {
        0.0
    }
}

#[derive(Debug, Default, PartialEq)]
struct Node<T> {
    val: T,
    left: Option<Box<Node<T>>>,
    right: Option<Box<Node<T>>>
}

impl<T: PartialOrd> Node<T> {
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