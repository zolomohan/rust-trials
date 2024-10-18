fn main() {
    let mut sum: i32 = 0;

    for num in 1..100 {
        if num % 2 == 0 {
            sum += num * num;
        }
    }

    println!("{}", (sum as f64).powf(1f64/5f64));
}
