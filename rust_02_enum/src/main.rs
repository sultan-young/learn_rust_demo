// #[derive(Debug)] // 这样可以立刻看到州的名称
// enum PigType {
//     ZhuBaJie(String),
//     PeiQi(String),
//     // --snip--
// }

// enum Monster {
//     Cat(String),
//     Dog(String),
//     Pig(PigType),
//     Monkey(String),
// }

// fn main() {
//    let monkey = Monster::Monkey(String::from("峨眉山猴子"));
//    let pig = Monster::Pig(PigType::PeiQi(String::from("小猪佩奇")));
//    let pig_zhubajie = Monster::Pig(PigType::ZhuBaJie(String::from("小猪佩奇")));
//    find_monster(monkey);
//    find_monster(pig);
//    find_monster(pig_zhubajie);
// }   
// fn find_monster(monster: Monster) {
//     match monster {
//         Monster::Pig(pig_type) => {
//             if let PigType::PeiQi(state) = pig_type {
//                 println!("获得了一直小猪佩奇类型的猪: {}", state)
//             } else {
//                 println!("并不是我想要的猪");
//             }
//         },
//         Monster::Monkey(name) => {
//             println!("得到了一直猴子，名为：{}", name);
//         },
//         _ => {},
//     }
// }

fn  main() {
    let x: Option<&str> = Some("asdf");
    match x {
        None => println!("得到了一个空值"),
        Some(number) => println!("获得的值为=> {}", number)
    }

    if let Option::Some(number) = x {
        println!("获得的值为=> {}", number)
    } else {
        println!("得到了一个空值")
    }
}