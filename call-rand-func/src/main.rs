use rand::{thread_rng, Rng};

fn first_function() {
    println!("This is the first function");
}

fn second_function() {
    println!("This is the second function");
}

fn third_function() {
    println!("This is the third function");
}

fn main() {
    let mut rng = thread_rng();
    let num: i32 = rng.gen_range(1..4);

    match num {
        1 => first_function(),
        2 => second_function(),
        3 => third_function(),
        _ => panic!(),
    }
}
