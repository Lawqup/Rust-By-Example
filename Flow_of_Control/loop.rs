fn main() {
	
	'outer: for j in 0..10{
		println!("********outer********");
		'inner: for i in 0..10{
			println!("inner");
			if i == 3 {
				break 'outer;
			}
		}
	}

}
