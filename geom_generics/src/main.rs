use std::ops::Add;

struct Point<T> {
    x : T,
    y : T
}

// impl<T : Display + Numeric> Point<T> {
//     fn add(a : Point<T>, b : Point<T>) -> Point<T> {
//         Point {x : a.x + b.x, y : a.y + b.y }
//     }
// 
//     fn string(&self) -> String {
//         let s = format!("x : {}, y : {}", self.x, self.y);
//         s 
//     }
// }

impl Point<f32> {
    fn add(a : Point<f32>, b : Point<f32>) -> Point<f32> {
        Point {x : a.x + b.x , y : a.y + b.y }
    }

    fn string(&self) -> String {
        let s = format!("x : {}, y : {}", self.x, self.y);
        s 
    }
}
        
fn main() {
    let p = Point{ x : 1.1, y : 2.3};
    let q = Point{ x : 0.0, y : 1.0};
    println!("Points added to result {:#?}.", Point::add(p,q).string());
}
