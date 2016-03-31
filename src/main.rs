mod vec2d;

use vec2d::Vec2d;

fn main() {
	let n = 5;
	// let p: Vec<f32> = vec![0.0; n];
	// let q: Vec<f32> = vec![0.0; n];

	let p: Vec<f32> = vec![0.0, 0.15, 0.1, 0.05, 0.1, 0.2];
	let q: Vec<f32> = vec![0.05, 0.1, 0.05, 0.05, 0.05, 0.1];

	// print!("{:?} \n", p);
	// print!("{:?} \n", p);

	let res = optimal_bst(&p, &q, n);
	let e = res.0;
	let root = res.1;
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