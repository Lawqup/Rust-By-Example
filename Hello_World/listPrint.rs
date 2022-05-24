use std::fmt;

struct List(Vec<i32>);

impl fmt::Display for List {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {

		let vec = &self.0;
		
		write!(f,"{{")?;

		for (i, elem) in vec.iter().enumerate() {
			write!(f, "{}: {}", i, elem)?;
			if i != vec.len() - 1 { write!(f, ", ")?; }
		}
		
		return write!(f, "}}");

	}
}

fn main() {
	let v = List(vec![1,2,4]);
	println!("{}", v);
}
