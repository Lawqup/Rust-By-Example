fn main() {
	let brovec = vec![0, 1, 2, 0, 0, 1, 0];

	for (i, elem) in brovec.iter().enumerate() {
		
		match elem {
			0 => println!("{}", i),
			_ => {},
		}

	}
}
