mod bst;
mod vec2d;

// use bst;
use vec2d::Vec2d;

fn main() {
	let n = 5;

	let p: Vec<f32> = vec![0.0, 0.3, 0.15, 0.10, 0.15, 0.3];
	let k: Vec<i32> = vec![1, 2, 3, 4, 5];

	let res = optimal_bst(&p, n);
	let e = res.0;
	let root = res.1;

	let mut tree_opt = bst::Bst::default();
	construct_optimal_bst(&k, &root, &mut tree_opt, 1, n);
	println!("Optimal tree: {:?}", tree_opt);
	println!("Height: {:?}", tree_opt.height());
	println!("Weighted path length: {:?}", tree_opt.weighted_path_length(&p));

	let mut tree_gr = bst::Bst::default();
	construct_greedy_bst(&k, &p, &mut tree_gr, 1, n);
	println!("Greedy tree: {:?}", tree_gr);
	println!("Height: {:?}", tree_gr.height());
	println!("Weighted path length: {:?}", tree_gr.weighted_path_length(&p));
}

fn optimal_bst(p: &Vec<f32>,n: usize) -> (Vec2d<f32>, Vec2d<usize>) {
	let mut e: Vec2d<f32> = Vec2d::new(n+1, n+2);
	let mut w: Vec2d<f32> = Vec2d::new(n+1, n+2);
	let mut root: Vec2d<usize> = Vec2d::new(n, n);

	println!("{:?}", e);

	for l in 1..n+1 {
		println!("loop 1: l={:?}", l);
		for i in 1..n-l+2 {
			let j = i + l - 1;
			println!("loop 2: i={:?}, j={:?}", i, j);
			e.set(i, j, std::f64::INFINITY as f32);
			let tmp = w.get(i, j-1) + p[j];
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

fn construct_optimal_bst<T: Default + PartialOrd + Clone>(k: &Vec<T>, root: &Vec2d<usize>, tree: &mut bst::Bst<T>, i: usize, j: usize) {
	if j == i-1 {
		return
	}

	let r = root.get(i-1, j-1);
	let new_val = k[r-1].clone();
	tree.insert(new_val);

	construct_optimal_bst(k, root, tree, i, r-1);
	construct_optimal_bst(k, root, tree, r+1, j);
}

fn construct_greedy_bst<T: PartialOrd + Clone>(k: &Vec<T>, p: &Vec<f32>, tree: &mut bst::Bst<T>, i: usize, j: usize) {
	if k.len() == 0 || j == i-1 {
		return
	} else if k.len() != p.len()-1 {
		panic!("k.len() must be one less than p.len()");
	}

	let mut max = (0, std::f32::NEG_INFINITY);
	for l in i..j+1 {
		if p[l] > max.1 {
			max = (l, p[l]);
		}
	}

	let new_val = k[max.0 - 1].clone();
	tree.insert(new_val);

	construct_greedy_bst(k, p, tree, i, max.0 -1);
	construct_greedy_bst(k, p, tree, max.0 +1, j)
}

fn construct_equal_bst<T>(k: &Vec<T>, p: &Vec<f32>, tree: &mut bst::Bst<T>, i: usize, j: usize) {

}