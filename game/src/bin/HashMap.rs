// 对于实现了copy trait的类型，值会被复制到hash map中。
// 对于拥有所有权的值（例如String），值会被移动，所有权会转移给HashMap。
// 如果将值的引用插入HashMap中，值本身不会移动

use std::collections::HashMap;

fn main() {
    // let mut scores = HashMap::new();
    // scores.insert(1, 2);

    let teams = vec![String::from("Blue"), String::from("red")];
    let initial_scores = vec![10, 50];

    let mut scores: HashMap<_, _> = teams.iter().zip(initial_scores.iter()).collect();

    // hashMap 取值
    let key = "Blue".to_string();
    let options = scores.get(&key);
    match options {
        Some(value) => println!("value => {}", value),
        None => println!("未找到对应key")
    }

    // 遍历hashMap 
    for (key, value) in &scores {
        println!("{} - {}", key, value);
    }

    // 输出hashMap
    println!("{:?}", scores);

    // 修改hashMap
    // scores.insert(String::from("Blue"), 20);
    let mut store = HashMap::new();
    store.insert("A", 1);
    // 重新insert会更新key相同的项
    store.insert("A", 2);
    println!("{:?}", store);
    // entry 只在key不存在情况下插入
    store.entry("A").or_insert(50);
    store.entry("B").or_insert(30);
    println!("{:?}", store);


    // 练习题:
    // 给定一段句子，记录其出现的单词数量。
    const text: &str = "hello world the funk world";
    let mut count_hashmap = HashMap::new();
    for word in text.split_whitespace() {
        let count  = count_hashmap.entry(word).or_insert(0);
        *count += 1;
    }
    println!("{:#?}", count_hashmap);

     
}