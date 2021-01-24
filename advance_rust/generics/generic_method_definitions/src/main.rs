#[derive(Debug)]
struct Point<T> {
	x: T,
	y: T,
}

impl <T> Point<T> {
	fn x(&self) -> &T {
		&self.x
	}
}

impl Point<f64> {
	fn distance(&self) -> f64 {
		(&self.x.powi(2) + &self.y.powi(2)).sqrt()
	}
}

#[derive(Debug)]
struct AdvPoint <T, U> {
	x: T,
	y: U
}

impl <T, U> AdvPoint<T, U> {
	fn mixup <V, W> (self, other: AdvPoint<V, W>) -> AdvPoint<T, W> {
		AdvPoint{
			x: self.x,
			y: other.y
		}
	}
}

fn main() {
    let p_one = Point{x:8, y:9};
    let p_two = Point{x:10.0, y:20.0};

    println!("{:?}", p_two.distance());

    // will throw an error because their is no method distance for integer type
    // println!("{:?}", p_one.distance());


    println!("{:#?}", p_one.x());
    println!("{:#?}", p_two.x());

    let adv_point_one = AdvPoint{x: 8, y: 9.0};
    let adv_point_two = AdvPoint{x: "hello", y: 'q'};

    let adv_point_three = adv_point_one.mixup(adv_point_two);

    println!("{:#?}", adv_point_three);
}
