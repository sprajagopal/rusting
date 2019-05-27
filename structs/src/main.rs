use rand::Rng;

#[derive(Debug)]
struct Dimensions {
    width: u32,
    height: u32,
}

impl Dimensions {
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn can_hold(&self, other: &Dimensions) -> (bool, String) {
        let b = other.width < self.width && other.height < self.height;
        let s = if b { String::from("can") } else {String::from("cannot")};
        (b, s)
    }
}

fn main() {
    let rect = Dimensions {
        width: 23,
        height: 10,
    };

    println!("Rect {:#?} area is {}", rect, rect.area());

    let inner = Dimensions {
        width : rand::thread_rng().gen_range(rect.width - 5, rect.width + 5),
        height : rand::thread_rng().gen_range(rect.height - 5, rect.height + 5)
    };

    let (_can_hold, result) = rect.can_hold(&inner);
    println!("New rect {:#?} {} fit inside {:#?}.", inner, result, rect);
}
