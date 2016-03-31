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
}