extern crate rand;
extern crate time;

mod bst;
mod vec2d;

use rand::Rng;
use std::env;
use std::fs::File;
use std::io::Write;
use time::*;

fn main() {
	// Skip over first argument
	let mut args = env::args().skip(1);
	let arg = args.next().unwrap();

	// Define a vector for the different size values
	let mut n_vec: Vec<usize> = Vec::new();

	// Reserve a file object to write results to
	let mut file: Option<File> = None;

	// Parse the first argument from the command line
	// If a filename is specified the file object is instantiated
	// and the results are written to that file
	// If a number is found, the program assumes that no filename
	// was provided and prints the results directly to the console
	match arg.parse::<usize>() {
		Ok(num) => {
			println!("No filename specified - printing results to console");
			n_vec.push(num);	// Assume that the first arg is the first n value
		}
		Err(ref err) => {;
			match File::create(&arg) {
				Ok(mut f) => {
					file = Some(f);
					println!("file created: {:?}", file);
				}
				Err(ref err) => println!("{:?}", err)
			}
		}
	}

	// Populate the size vector
	loop {
		match args.next() {
			Some(ref s) => {
				let value = s.parse::<usize>().unwrap();
				n_vec.push(value);
			}
			None => break
		}
	}

	match file {
	    Some(ref mut f) => {
	    	f.write(b"n , Optimal, Greedy, Even, Random");
	    }
	    None => {}
	}

	for n in n_vec {
		match file {
			Some(ref mut f) => {
				let mut tmp_str = String::new();
				tmp_str.push_str("\n");
				tmp_str.push_str(&n.to_string());
				f.write(tmp_str.as_bytes());

			}
			None => {}
		}

		println!("-------------------");
		println!("Running with n = {:?}", n);
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
		test_bst(&k, &p, n, "Optimal BST", &bst::construct_optimal_bst, &mut file);
		test_bst(&k, &p, n, "Greedy BST", &bst::construct_greedy_bst, &mut file);
		test_bst(&k, &p, n, "Equal BST", &bst::construct_equal_bst, &mut file);
		test_bst(&k, &p, n, "Random BST", &bst::construct_random_bst, &mut file);
	}

}

fn test_bst<T: PartialOrd + Clone + Default>(k: &Vec<T>, p: &Vec<f32>, n: usize, label: &'static str,
		func: &Fn(&Vec<T>, &Vec<f32>, &mut bst::Bst<T>, usize, usize), file: &mut Option<File>) {
	
	let mut tree = bst::Bst::default();
	let start = time::precise_time_ns();
	func(k, p, &mut tree, 1, n);
	let duration = time::precise_time_ns() - start;
	let duration_ms = (duration as f64) / (1000.0 * 1000.0);

	println!("{:?} ran in {:?} ms", label, duration_ms);

	let wpl = tree.weighted_path_length(&p);

	match file {
		&mut Some(ref mut f) => {
			let mut tmp_str = String::new();
			tmp_str.push_str(", ");
			tmp_str.push_str(&wpl.to_string());
			f.write(tmp_str.as_bytes());
		}
		&mut None => {
			// TODO: check if possible to print function name here
			// println!("{:?}", func.to_string());
			println!("Height: {:?}", tree.height());
			println!("Weighted path length: {:?} \n", wpl);
		}
	}
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