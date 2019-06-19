use std::cmp::PartialOrd;
use std::fmt::Display;

struct Sequence<T> {
    arr : Vec<T>
}

impl<T : PartialOrd + Copy> Sequence<T> {
    pub fn max(&self) -> T {
        let mut max_elem = self.arr[0];
        for &i in self.arr.iter() {
            if i > max_elem {
                max_elem = i;
            }
        }
        
        max_elem
    }
}

impl<T : Display + Copy> Sequence<T> {
    fn offset(oset : u32) -> String {
        let mut offset = String::new();
        for _i in 0..oset {
            offset = offset + "\t";
        }
        offset
    }

    pub fn print(&self, oset : u32) -> String {
        let mut s = String::from("\n") + &Sequence::<T>::offset(oset) + &String::from("{\n");
        for &i in self.arr.iter() {
            s = format!("{}{}{}\n", s, Sequence::<T>::offset(2 * oset), i);
        }
        format!("{}{}{}\n", s, Sequence::<T>::offset(oset), String::from("}\n")) 
    }
}

fn main() {
    println!("Program demonstrates the PartialOrd trait required for implementing functions for a generic sequence of numbers.");

    let s = Sequence {
        arr : vec![0, 1, 2, 3, 5, 10, 3]
    };
    println!("  Maximum of {} found: {}", s.print(1), s.max());
}
