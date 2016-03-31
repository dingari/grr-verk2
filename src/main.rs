mod bst;
mod vec2d;

use bst::Bst;
use vec2d::Vec2d;

fn main() {
	let n = 5;
	// let p: Vec<f32> = vec![0.0; n];
	// let q: Vec<f32> = vec![0.0; n];

	let p: Vec<f32> = vec![0.0, 0.15, 0.1, 0.05, 0.1, 0.2];
	let q: Vec<f32> = vec![0.05, 0.1, 0.05, 0.05, 0.05, 0.1];

	// let k: Vec<&str> = vec!["asdf", "bar", "baz", "foo", "oof"];
	let k: Vec<i32> = vec![1, 2, 3, 4, 5];

	println!("{:?}", k);

	let res = optimal_bst(&p, &q, n);
	let e = res.0;
	let root = res.1;

	let mut bst = Bst::default();
	println!("BST before: {:?}", bst);
	construct_optimal_bst(&k, &root, &mut bst, 1, n);
	println!("BST after: {:?}", bst);
}

fn optimal_bst(p: &Vec<f32>, q: &Vec<f32>, n: usize) -> (Vec2d<f32>, Vec2d<usize>) {
	let mut e: Vec2d<f32> = Vec2d::new(n+1, n+2);
	let mut w: Vec2d<f32> = Vec2d::new(n+1, n+2);
	let mut root: Vec2d<usize> = Vec2d::new(n, n);

	
	for i in 1..n+2 {
		e.set(i, i-1, q[i-1]);
		w.set(i, i-1, q[i-1]);
	}

	println!("{:?}", e);

	for l in 1..n+1 {
		println!("loop 1: l={:?}", l);
		for i in 1..n-l+2 {
			let j = i + l - 1;
			println!("loop 2: i={:?}, j={:?}", i, j);
			e.set(i, j, std::f64::INFINITY as f32);
			let tmp = w.get(i, j-1) + p[j] + q[j];
			w.set(i, j, tmp);
			for r in i..j+1 {
				let t = e.get(i, r-1) + e.get(r+1, j) + w.get(i, j);
				println!("loop 3: r={:?}, t={:?}", r, t);
				if t < e.get(i, j) {
					e.set(i, j, t);
					root.set(i-1, j-1, r);
				}
			}
		}
	} 

	// TODO: just for testing, remove later
	println!("{:?}", e);
	println!("{:?}", w);
	println!("{:?}", root);

	return (e, root);
}

fn construct_optimal_bst<T: Default + PartialOrd + Clone>(k: &Vec<T>, root: &Vec2d<usize>, bst: &mut Bst<T>, i: usize, j: usize) {
	if j == i-1 {
		return
	}

	let r = root.get(i-1, j-1);
	let new_val = k[r-1].clone();
	bst.insert(new_val);

	construct_optimal_bst(k, root, bst, i, r-1);
	construct_optimal_bst(k, root, bst, r+1, j);
}