// fn main() {
//     // rust的shadow概念，讲一个变量声明多次，之前声明的变量就会被隐藏
//     let x = 1;
//     let x = x * 2;
//     let x = x * 2;
//     println!("is {}", x);

//     let spaces = "      ";
//     let spaces = spaces.len();
//     println!("is {}", spaces);

//     // ![](https://cdn.jsdelivr.net/gh/sultan-young/picture-bed/assets/20220921155618.png)
//     // let i8Number: i8 = 2;
//     // let i32Number: i32 = 2;
//     // let i64Number: i64 = 2;
//     // let i128Number: i128 = 2;

//     // 元祖, 元祖中可以存放多种类型，元祖的长度是固定的
//     let tup: (i32, f64, u128) = (1, 2.4, 3);
//     let (x, y, z) = tup;
//     println!("{} {} {}", x, y, z);

//     // 数组，数组只能存放单种类型，数组长度固定。
//     // let arr: [i32; 5] = [1,2,3,4,5];
//     let _arr = [3; 5]; // 相当于 [3, 3, 3, 3, 3];
//     println!("{}", _arr[4]);

//     let num = 10;

//     if num % 3 == 0 {
//         println!("可以被3整除")
//     } else if num % 3 == 0{
//         println!("可以被4整除")
//     } else if num % 5 == 0{
//         println!("可以被5整除")
//     }

//     let arr = [1,2,3,4,5];
//     for element in arr.iter() {
//         println!("{}", element)
//     }

//     for number in (1..4).rev() {
//         println!("{}", number)
//     }

//     let mut str = String::from("Hello, world!");
//     let word_index = first_word(&str);
//     println!("wordIndex {}", word_index  )

// }

// fn first_word(str: &String) -> usize {
//     let bytes = str.as_bytes();

//     for (i, &item) in bytes.iter().enumerate() {
//         if item == b' ' {
//             return  i;
//         }
//     }

//     return str.len();
// }




/* 
    字符串切片练习题
   要求实现一个函数，函数接收一个句子，返回句子的第三个单词。如果没有，则返回整个句子. 
 */
fn main() {
    let str1 = String::from("I am the best boy");
    let third_word = find_same_word(&str1[..]);
    println!("{}", third_word);
}

fn find_same_word(str1: &str) -> &str{
    let mut count = 0;
    let mut start_index = 0;

    for (i, &item) in str1.as_bytes().iter().enumerate() {
        if item == b' ' {
            count += 1;
        }
        if count == 3 && start_index == 0{
            start_index = i;
        }
        if count == 4 {
            println!("{}, {}", start_index, i);
            return  &str1[start_index .. i].trim();
        }
    }
    return  &str1[ .. ];
}