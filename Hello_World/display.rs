use std::fmt;


#[derive(Debug)]
struct Complex {
	re: f64,
	im: f64, 
}

impl fmt::Display for Complex { 
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result { 
		write!(f, "{} + {}i", self.re, self.im)
	} 
}

fn main() {


	let bronum = Complex{ re: 3.0, im: 2.0 };
	println!("Display: {}", bronum);
	println!("Debug: {:?}", bronum);



}
