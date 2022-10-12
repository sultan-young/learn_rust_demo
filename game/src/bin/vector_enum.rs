fn main() {
    enum Type {
        Number(usize),
        String(String),
    }
    let mut vec: Vec<Type> = Vec::new();
    vec.push(Type::Number(123));
    vec.push(Type::String(String::new()));

}