fn foo() {
    println!("Function creates vector and references an element");
    let v : Vec<i32> = Vec::new();
    let v2 = vec![1,2,3,];
    println!("  Vector 1: {:?}", v);
    println!("  Vector 2: {:?}", v2);

    let ref_v : &i32 = &v2[0];
    println!("  Referencing the first element ({}) of {:?}", ref_v, v2);
}

fn bar(v: &mut Vec<i32>){
    let e = 1;
    v.push(e);
    println!("Function pushes a new element {} to the vector {:?}", e, v);
}

fn this_works(){
    println!("Function demonstrates that an immutable reference to an element in a vector can be created along with mutable references of the vector AS LONG AS the immutable reference is NOT USED at all.");
    let ref_elem_v = foo();
    let mut v : Vec<i32> = Vec::new();
    bar(&mut v);
    let some_immut_ref = &v[0];
    let idx = 100;
    println!("  {}th element of Vector {:?} is {:?}", idx, v, v.get(idx));
}

fn this_works_also(){
    println!("Function demonstrates that an immutable reference to an element in a vector can be created along with mutable references of the vector AS LONG AS the immutable reference is NOT USED at all.");

    let mut v : Vec<i32> = Vec::new();
    bar(&mut v);
    let some_immut_ref = &v[0];
    v.push(4);
    
    // can i add more elements now?
    bar(&mut v);
    let idx = 3;
    println!("  {}th element of Vector {:?} is {:?}", idx, v, v.get(idx));
}

fn this_works_too(){
    println!("Function demonstrates that an immutable reference to an element in a vector can be created along with mutable references of the vector AS LONG AS the immutable reference is NOT USED at all.");

    let mut v2 = vec![1,2,3,];
    let another_immut_ref = &v2[1];
    v2.push(2);
}

// uncommenting the following fn
// causes compiler error
// fn this_no_works(){
//     let mut v2 = vec![1,2,3,];
//     let another_immut_ref = &v2[1];
//     v2.push(2);
//     println!("{}", *another_immut_ref);
// }

fn vector_iter(){
    println!("Function demonstrates iterating over a vector");
    let mut v : Vec<i32> = Vec::new();
    v.push(2);
    v.push(3);
    for i in &v {
        println!("  only printing {}", i);
    }

    for i in &mut v {
        let e = *i;
        *i += 2;
        println!("  adding two to {} ==> {}", e, *i);
    } 
    
}

fn clone_of_vector () {
    println!("Function demonstrates cloning a vector to another new vector");
    let mut v : Vec<i32> = vec![9, 2, 3,];
    let cloned_v = v.clone();
    println!("  Vec: {:?}", v);
    println!("  Clone of vec {:?}", cloned_v);
    println!("  Changing elements in main vector now");
    v[0] = v[0] * 2;
    println!("  Vec: {:?}", v);
    println!("  Clone of vec {:?}", cloned_v);
}

// uncommenting the following fn
// will cause compiler error
// fn ref_of_vector () {
//     println!("Function demonstrates equating a vector to another new vector");
//     let v : Vec<i32> = vec![9, 2, 3,];
//     let ref_v = v;
//     println!("  Vec: {:?}", v);
//     println!("  Ref of vec {:?}", ref_v);
//     println!("  Changing elements in main vector now");
//     v[0] = v[0] * 2;
//     println!("  Copy of vec {:?}", ref_v);
// }
    
fn main() {
    this_works();
    this_works_also();
    this_works_too();
    vector_iter();
//    ref_of_vector();
    clone_of_vector();
}
