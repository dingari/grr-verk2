extern crate rand;

mod bst;
mod vec2d;

use rand::Rng;
use vec2d::Vec2d;

fn main() {
	let n = 5;

	let p: Vec<f32> = vec![0.0, 0.3, 0.2, 0.05, 0.15, 0.3];
	let k: Vec<i32> = vec![1, 2, 3, 4, 5];

	let res = optimal_bst(&p, n);
	let e = res.0;
	let root = res.1;

	let mut tree_opt = bst::Bst::default();
	construct_optimal_bst(&k, &root, &mut tree_opt, 1, n);
	println!("Optimal tree: {:?}", tree_opt);
	println!("Height: {:?}", tree_opt.height());
	println!("Weighted path length: {:?} \n", tree_opt.weighted_path_length(&p));

	let mut tree_gr = bst::Bst::default();
	construct_greedy_bst(&k, &p, &mut tree_gr, 1, n);
	println!("Greedy tree: {:?}", tree_gr);
	println!("Height: {:?}", tree_gr.height());
	println!("Weighted path length: {:?} \n", tree_gr.weighted_path_length(&p));

	let mut tree_eq = bst::Bst::default();
	construct_equal_bst(&k, &p, &mut tree_eq, 1, n);
	println!("Equal tree: {:?}", tree_eq);
	println!("Height: {:?}", tree_eq.height());
	println!("Weighted path length: {:?} \n", tree_eq.weighted_path_length(&p));

	let mut tree_rand = bst::Bst::default();
	construct_random_bst(&k, &mut tree_rand);
	println!("Random tree: {:?}", tree_rand);
	println!("Height: {:?}", tree_rand.height());
	println!("Weighted path length: {:?} \n", tree_rand.weighted_path_length(&p));
}

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

fn construct_optimal_bst<T: Default + PartialOrd + Clone>(k: &Vec<T>, root: &Vec2d<usize>, tree: &mut bst::Bst<T>, i: usize, j: usize) {
	if j == i-1 {
		return
	}

	let r = root.get(i-1, j-1);
	let new_val = k[r-1].clone();
	tree.insert(new_val);

	// Construct subtrees recursively
	construct_optimal_bst(k, root, tree, i, r-1);
	construct_optimal_bst(k, root, tree, r+1, j);
}

fn construct_greedy_bst<T: PartialOrd + Clone>(k: &Vec<T>, p: &Vec<f32>, tree: &mut bst::Bst<T>, i: usize, j: usize) {
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

fn construct_equal_bst<T: PartialOrd + Clone>(k: &Vec<T>, p: &Vec<f32>, tree: &mut bst::Bst<T>, i: usize, j: usize) {
	if j == i-1 {
		return
	}

	let mut r = (j - i)/2 + i;

	let mut l_sum = 0.0;
	for l in i..r {
		l_sum += p[l];
	}

	let mut r_sum = 0.0;
	for l in r+1..j+1 {
		r_sum += p[l];
	}

	let mut min_diff = (l_sum - r_sum).abs();

	// Loop until the best root is found
	loop {
		let l_sum_old = l_sum;
		let r_sum_old = r_sum;
		let r_old = r;

		if l_sum < r_sum {
			// Move root to the right
			r = r+1;
			l_sum = l_sum + p[r-1];
			r_sum = r_sum - p[r];
		} else if l_sum > r_sum {
			// Move root to the left
			r = r-1;
			l_sum = l_sum - p[r];
			r_sum = r_sum + p[r+1];
		} 

		let diff = (l_sum - r_sum).abs();

		if diff < min_diff {
			min_diff = diff;
		} else {
			l_sum = l_sum_old;
			r_sum = r_sum_old;
			r = r_old;

			break;
		}
	}
	
	let new_val = k[r-1].clone();
	tree.insert(new_val);

	// Construct subtrees recursively in the same manner
	construct_equal_bst(k, p, tree, i, r-1);
	construct_equal_bst(k, p, tree, r+1, j);
}

fn construct_random_bst<T: PartialOrd + Clone>(k: &Vec<T>, tree: &mut bst::Bst<T>) {
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