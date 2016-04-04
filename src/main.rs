extern crate rand;
extern crate time;

mod bst;
mod vec2d;

use rand::Rng;
use std::env;
use std::fs::File;
use time::*;

fn main() {
	// let n: usize = 100;

	// Skip over first argument
	let mut args = env::args().skip(1);

	// Consume print argument
	let print = args.next().unwrap();

	// Consume values of n to test for
	let mut n_vec: Vec<usize> = Vec::new();

	loop {
		match args.next() {
			Some(ref s) => {
				let value = s.parse::<usize>().unwrap();
				n_vec.push(value);
			}
			None => break
		}
	}

	let file: Option<File> = None;

	// let mut tree_opt = bst::Bst::default();
	// bst::construct_optimal_bst(&k, &p, &mut tree_opt, 1, n);
	// // println!("Optimal tree: {:?}", tree_opt);
	// println!("Optimal tree");
	// println!("Height: {:?}", tree_opt.height());
	// println!("Weighted path length: {:?} \n", tree_opt.weighted_path_length(&p));

	// let mut tree_gr = bst::Bst::default();
	// bst::construct_greedy_bst(&k, &p, &mut tree_gr, 1, n);
	// // println!("Greedy tree: {:?}", tree_gr);
	// println!("Greedy tree");
	// println!("Height: {:?}", tree_gr.height());
	// println!("Weighted path length: {:?} \n", tree_gr.weighted_path_length(&p));

	// let mut tree_eq = bst::Bst::default();
	// bst::construct_equal_bst(&k, &p, &mut tree_eq, 1, n);
	// // println!("Equal tree: {:?}", tree_eq);
	// println!("Equal tree");
	// println!("Height: {:?}", tree_eq.height());
	// println!("Weighted path length: {:?} \n", tree_eq.weighted_path_length(&p));

	// let mut tree_rand = bst::Bst::default();
	// bst::construct_random_bst(&k, &mut tree_rand);
	// // println!("Random tree: {:?}", tree_rand);
	// println!("Random tree");
	// println!("Height: {:?}", tree_rand.height());
	// println!("Weighted path length: {:?} \n", tree_rand.weighted_path_length(&p));

	for n in n_vec {
		println!("-------------------");
		println!("Running with n={:?}", n);
		println!("-------------------");
		let k: Vec<usize> = (1..n+1).collect();

		// We need to have an extra zero at the front of the p vector
		// This is a consequence of implementing the optimal_bst function
		// to expect another frequency vector q that represents
		// frequency of values that are not in the tree
		// I will probably change this later
		let mut p: Vec<f32> = Vec::new();
		p.push(0.0);
		let mut shuffled_vec = zipf_vec_shuffled(n);
		p.append(&mut shuffled_vec);

		// Pass the constructing functions in as closures
		// That way we are able to use just one function 
		// to test the four cases!
		test_bst(&k, &p, n, &bst::construct_optimal_bst, &file);
		test_bst(&k, &p, n, &bst::construct_greedy_bst, &file);
		test_bst(&k, &p, n, &bst::construct_equal_bst, &file);
		test_bst(&k, &p, n, &bst::construct_random_bst, &file);
	}

}

fn test_bst<T: PartialOrd + Clone + Default>(k: &Vec<T>, p: &Vec<f32>, n: usize, 
		func: &Fn(&Vec<T>, &Vec<f32>, &mut bst::Bst<T>, usize, usize), file: &Option<File>) {
	
	let mut tree = bst::Bst::default();
	func(k, p, &mut tree, 1, n);

	// TODO: check if possible to print function name here
	// println!("{:?}", func);
	println!("Height: {:?}", tree.height());
	println!("Weighted path length: {:?} \n", tree.weighted_path_length(&p));
}


fn zipf_vec(n: usize) -> Vec<f32> {
	let mut c_inv = 0.0;
	for i in 1..n {
		c_inv += 1.0 / ((i+1) as f32);
	}

	let mut p = vec![0.0; n];

	for i in 0..p.len() {
		p[i] = (1.0 / c_inv) / ((i+1) as f32);
	}

	return p
}

fn zipf_vec_shuffled(n: usize) -> Vec<f32> {
	let mut rng = rand::thread_rng();

	let mut zipf_vec = zipf_vec(n);
	let mut zipf_slice = zipf_vec.as_mut_slice();
	rng.shuffle(&mut zipf_slice);

	let mut p = Vec::new();
	p.extend_from_slice(zipf_slice);
	return p;
}

#[cfg(test)]
mod tests {
	#[test]
	fn zipf_sum() {
		let sum: f32 = super::zipf_vec(100).iter().fold(0.0, |sum, x| sum + x);

		assert_eq!(((sum * 1000.0) / 1000.0) as usize, 1);
	}

	#[test]
	fn zipf_shuffled_sum() {
		let sum: f32 = super::zipf_vec(100).iter().fold(0.0, |sum, x| sum + x);

		assert_eq!(((sum * 1000.0) / 1000.0) as usize, 1);
	}

}