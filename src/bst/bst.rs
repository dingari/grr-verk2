extern crate rand;
extern crate std;


use self::rand::Rng;
use vec2d::Vec2d;

use super::node::Node;


#[derive(Debug, Default)]
pub struct Bst<T> {
    pub root: Option<Box<Node<T>>>
}

#[allow(dead_code)]
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

    #[allow(unused_variables)]
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

    #[allow(dead_code, unused_variables)]
    pub fn is_empty(&self) -> bool {
        match &self.root {
            &Some(ref node) => false,
            &None => true
        }
    }
}


////////////////////////
// "Static" functions //
////////////////////////

fn optimal_bst(p: &Vec<f32>,n: usize) -> (Vec2d<f32>, Vec2d<usize>) {
    let mut e: Vec2d<f32> = Vec2d::new(n+1, n+2);
    let mut w: Vec2d<f32> = Vec2d::new(n+1, n+2);
    let mut root: Vec2d<usize> = Vec2d::new(n, n);

    for l in 1..n+1 {
        for i in 1..n-l+2 {
            let j = i + l - 1;
            e.set(i, j, std::f64::INFINITY as f32);
            let tmp = w.get(i, j-1) + p[j];
            w.set(i, j, tmp);
            for r in i..j+1 {
                let t = e.get(i, r-1) + e.get(r+1, j) + w.get(i, j);
                if t < e.get(i, j) {
                    e.set(i, j, t);
                    root.set(i-1, j-1, r);
                }
            }
        }
    } 

    return (e, root);
}

fn construct_optimal_bst_rec<T: PartialOrd + Clone>(k: &Vec<T>, root: &Vec2d<usize>, tree: &mut Bst<T>, i: usize, j: usize) {
    if j == i-1 {
        return
    }

    let r = root.get(i-1, j-1);
    let new_val = k[r-1].clone();
    tree.insert(new_val);

    // Construct subtrees recursively
    construct_optimal_bst_rec(k, root, tree, i, r-1);
    construct_optimal_bst_rec(k, root, tree, r+1, j);
}

pub fn construct_optimal_bst<T: PartialOrd + Clone>(k: &Vec<T>, p: &Vec<f32>, tree: &mut Bst<T>, i: usize, j: usize) {
    let root = optimal_bst(p, j).1;

    construct_optimal_bst_rec(k, &root, tree, i, j);
}

pub fn construct_greedy_bst<T: PartialOrd + Clone>(k: &Vec<T>, p: &Vec<f32>, tree: &mut Bst<T>, i: usize, j: usize) {
    if k.len() == 0 || j == i-1 {
        return
    }

    assert!(k.len() == p.len()-1);

    let mut max = (0, std::f32::NEG_INFINITY);
    for l in i..j+1 {
        if p[l] > max.1 {
            max = (l, p[l]);
        }
    }

    let new_val = k[max.0 - 1].clone();
    tree.insert(new_val);

    // Construct subtrees recursively
    construct_greedy_bst(k, p, tree, i, max.0 -1);
    construct_greedy_bst(k, p, tree, max.0 +1, j)
}

pub fn construct_equal_bst<T: PartialOrd + Clone>(k: &Vec<T>, p: &Vec<f32>, tree: &mut Bst<T>, i: usize, j: usize) {
    if j == i-1 {
        return
    }

    let mut min: (usize, f32) = (0, std::f32::INFINITY);

    for r in i..j+1 {
        let mut r = r;

        if (r-i)%2 != 0 {
            r = j - r/2 + (i+1)/2;
        } else {
            r = r/2 + (i+1)/2;
        }
        
        // Sum left side
        let mut l_sum: f32 = 0.0;
        for s in i..r {
            l_sum += p[s];
        }

        // Sum right side
        let mut r_sum: f32 = 0.0;
        for t in r+1..j+1 {
            r_sum += p[t];
        }

        let diff = (l_sum - r_sum).abs();
        if diff < min.1 {
            min = (r, diff)
        }
    }
    
    let r = min.0;
    let new_val = k[r-1].clone();
    tree.insert(new_val);

    // Construct subtrees recursively in the same manner
    construct_equal_bst(k, p, tree, i, r-1);
    construct_equal_bst(k, p, tree, r+1, j);
}

// Variables p, i and j are not used, but are there to maintain
// the same function signature as the other bst constructing functions
#[allow(unused_variables)]
pub fn construct_random_bst<T: PartialOrd + Clone>(k: &Vec<T>, p: &Vec<f32>, tree: &mut Bst<T>, i: usize, j: usize) {
    let n = k.len();
    let mut used_ind: Vec<bool> = vec![false; n];
    let mut rng = rand::thread_rng();

    let mut num_inserted = 0;
    while num_inserted < n {
        let i = rng.gen_range(0, n);
        if !used_ind[i] {
            tree.insert(k[i].clone());
            used_ind[i] = true;
            num_inserted += 1;
        }
    }
}