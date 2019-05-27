fn give_ref() -> &String {
    let s = String::from("returned");
    &s
}

fn main(){
    let something = give_ref();
}
