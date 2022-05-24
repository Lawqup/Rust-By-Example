// This function borrows a slice
fn analyze_slice(slice: &[i32]) {
    println!("first element of the slice: {}", slice[0]);
    println!("the slice has {} elements", slice.len());
}

fn main() {
    // Fixed-size array (type signature is superfluous)
    let xs: [i32; 3] = [1, 2, 3];

    // All elements can be initialized to the same value
    let _ys: [i32; 500] = [0; 500];

	for i in 0..xs.len() {
		for j in i+1..xs.len()+1 {
			analyze_slice(&xs[i..j]);
		}
	}


}
