fn main() {
    let mut v = vec![1,2,3,4,5];
    for item in &mut v {
        *item += 10;
    };

    for item in v {
        println!("{}", item)
    }
}