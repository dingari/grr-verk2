
use std::fmt;

pub struct Vec2d<T> {
    v: Vec<Vec<T>>
}

impl<T: Clone + Default + fmt::Debug> Vec2d<T> {
    pub fn new(n: usize, m: usize) -> Vec2d<T> {
    	let tmp_v: Vec<Vec<T>> = vec![vec![T::default(); n]; m];
    	Vec2d {
    		v: tmp_v
    	}
    }

    pub fn get(&self, i: usize, j: usize) -> T {
        // TODO: might want to return a reference instead of clone
        // But for now, this works better
    	self.v[i][j].clone()
    }

    pub fn set(&mut self, i: usize, j: usize, val: T) {
    	self.v[i][j] = val;
    }

    #[allow(dead_code)]
    pub fn len(&self) -> (usize, usize) {
    	(self.v.len(), self.v[0].len())
    }
}

impl<T: fmt::Debug> fmt::Debug for Vec2d<T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    	for i in 0..self.v.len() {
    		for j in 0..self.v[i].len() {
    			write!(f, "{:?}, ", self.v[i][j]);
    		}
    		write!(f, "\n");
    	}

    	write!(f, "")
    }
}