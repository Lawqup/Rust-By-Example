#[derive(Debug, Copy, Clone)]
struct Point {
	x: f32,
	y: f32
}

#[derive(Debug)]
struct Rect {
	top_left: Point,
	bottom_right: Point
}

impl Rect {
	fn rect_area(&self) -> f32 {
		let width = &self.top_left.x - &self.bottom_right.x;
		let height = &self.top_left.y - &self.bottom_right.y;
		
		width*height
	}
}

fn square(top_left: Point, width: f32) -> Rect {
	Rect {
		top_left,
		bottom_right: Point {x: top_left.x + width, y: top_left.y + width}
	}
}
fn main() {
	let rect = Rect {
		top_left: Point{x: 2.0, y: 3.0},
		bottom_right: Point{x: 5.0, y: 5.0}
	};

	println!("{:#?}", rect);
	println!("Area: {}", rect.rect_area());

	let sq = square(Point{x: 1.0, y: 1.5}, 5.0);

	println!("{:#?}", sq);
	println!("Area: {}", sq.rect_area());
}
