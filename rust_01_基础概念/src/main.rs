// 摄氏度和华氏度互转
// enum Temperature {
//     C, // 摄氏度
//     F, // 华氏度
// }

// fn main() {
//     let temperature: f32 = cover_temperature(56.0, Temperature::C);
//     println!("温度为：{}", temperature)
// }

// fn cover_temperature(temperature: f32, temperature_type: Temperature) -> f32{
//     let result:f32;
//     match temperature_type {
//         Temperature::C => {
//             result = 1.8 * temperature + 32.0;
//         }
//         Temperature::F => {
//             result = (temperature - 32.0) / 1.8 ;
//         }
//     }
//     return  result;
// }


// 生成n阶斐波那契数列
// fn main() {
//     let order = 10;
//     println!("{} 的阶数为 {}", order, fibonacci(order))
// }

// fn fibonacci(order: u32) -> u32 {
//     if order <= 1 {
//         return order;
//     }
//     return fibonacci(order - 1) + fibonacci(order - 2);
// }

// 打印圣诞颂歌 “The Twelve Days of Christmas” 的歌词，利用歌曲中的重复部分（编写循环）
// fn main() {
//     let mouth_count = [
//         "first", "second", "third", "fourth", "fifth", "sixth", "seventh", "eighth", "ninth",
//         "tenth", "eveth", "Twelve",
//     ];
//     println!("The Twelve Days Of Christmas - Songtime Kids");
//     for mouth_index in 0..mouth_count.len() {
//         println!("On the {} day of Christmas", mouth_count[mouth_index]);
//         println!("My true love sent to me");
//         if mouth_index != 0 {
//             println!("{} turtle doves", mouth_count[mouth_index])
//         }
//         println!("{} partridge in a pear tree", if mouth_index != 0 {"And a"} else {"A"})
//     }
// }