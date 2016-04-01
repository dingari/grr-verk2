extern crate rand;

mod bst;
mod vec2d;

use rand::Rng;
use vec2d::Vec2d;

fn main() {
	let n = 5;

	let p: Vec<f32> = vec![0.0, 0.3, 0.2, 0.05, 0.15, 0.3];
	let k: Vec<i32> = vec![1, 2, 3, 4, 5];

	let res = bst::optimal_bst(&p, n);
	let e = res.0;
	let root = res.1;

	let mut tree_opt = bst::Bst::default();
	bst::construct_optimal_bst(&k, &root, &mut tree_opt, 1, n);
	println!("Optimal tree: {:?}", tree_opt);
	println!("Height: {:?}", tree_opt.height());
	println!("Weighted path length: {:?} \n", tree_opt.weighted_path_length(&p));

	let mut tree_gr = bst::Bst::default();
	bst::construct_greedy_bst(&k, &p, &mut tree_gr, 1, n);
	println!("Greedy tree: {:?}", tree_gr);
	println!("Height: {:?}", tree_gr.height());
	println!("Weighted path length: {:?} \n", tree_gr.weighted_path_length(&p));

	let mut tree_eq = bst::Bst::default();
	bst::construct_equal_bst(&k, &p, &mut tree_eq, 1, n);
	println!("Equal tree: {:?}", tree_eq);
	println!("Height: {:?}", tree_eq.height());
	println!("Weighted path length: {:?} \n", tree_eq.weighted_path_length(&p));

	let mut tree_rand = bst::Bst::default();
	bst::construct_random_bst(&k, &mut tree_rand);
	println!("Random tree: {:?}", tree_rand);
	println!("Height: {:?}", tree_rand.height());
	println!("Weighted path length: {:?} \n", tree_rand.weighted_path_length(&p));
}

