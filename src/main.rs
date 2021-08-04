fn main() {
    let a = 10;
    // a = 15; 不行
    // 变量默认不可变

    // 可变变量
    let mut b = 1;
    b = 2;

    // 隐藏变量(重新声明)
    let c = 5;
    let c = c * 3;

    // 强类型
    // let number: u32 = 'foo';
    let number: u32 = 100;
    // 32位电脑就32位
    let number: isize = 888;

    // 判断和计算
    let isBigger: bool = 2 > 1;
    let calcResult: u32 = 1 + 2 - 3 * 2 / 2;

    // 字符和字符串
    // 字符得是单引号
    let emoji: char = '😊';
    let str: &str = "我都想笑了";

    // 元组
    let tup = (emoji, str, isBigger);

    println!("Hello world");
    println!("a,b,c: {}, {}, {}", a, b, c);
    println!("number: {}", number);
    println!("isBigger: {}", isBigger);
    println!("calcResult: {}", calcResult);
    println!("Emoji: {}", emoji);
    println!("str: {}", str);
    // 需要index才能在print里access
    println!("tup: {}, {}, {}", tup.0, tup.1, tup.2);

    // if
    let formal = true;
    let greeting = if formal {
        "Good day to you."
    } else {
        "Hey!"
    };
    println!("{}", greeting);

    // struct
    // 经典版
    struct Classic {name: String, age: usize};
    // 元组版(index访问)
    struct Tuple(String, usize);
    // 单元结构(做标记)
    struct Unit;
    let person0 = Classic{name: "Foo".parse().unwrap(), age: 18};
    let person1 = Tuple("Bar".parse().unwrap(), 10);
    println!("Classic: {}; Tuple: {}", person0.name, person1.0);

    // enum
    enum Transmission {
        Manual,
        SemiAuto,
        Automatic,
    }
    // 实例化, 用'::', <enum>::<variant>, eg: Transmission::Manual.

    // 函数
    goodbye(0);

    // 数组
    let arr = ['1', '2'];
    // 第一个元素为0, 长度为5
    let declare_arr = [0; 5];
    println!("arr: {}, {}", arr[0], arr[1]);

    // 向量(动态数组)
    let mut three_nums = vec![15, 3, 46];
    println!("Initial vector: {:?}", three_nums);
    three_nums.push(20);
    println!("Later vector: {:?}", three_nums);
    three_nums.pop();
    println!("Final vector: {:?}", three_nums);

    // 哈希表
    use std::collections::HashMap;
    let mut reviews: HashMap<String, String> = HashMap::new();

    reviews.insert("Ancient Roman History".to_string(), "Very accurate.".to_string());
    reviews.insert("Cooking with Rhubarb".to_string(), "Sweet recipes.".to_string());
    reviews.insert("Programming in Rust".to_string(), "Great examples.".to_string());
    reviews.remove("Ancient Roman History");
    println!("Map: {:?}; find stuff: {:?}", reviews, reviews.get("0"));

    // 循环
    // loop
    let mut counter = 1;
    let stop_loop = loop {
        counter *= 2;
        if counter > 100 {
            break counter;
        }
    };
    println!("Break the loop at counter = {}.", stop_loop);
    // while
    let mut love = true;
    while love {
        println!("love {}", love);
        love = false;
    }
    // for
    for i in arr.iter() {
        println!("{}", i);
    }
}

fn goodbye(input: u32) -> bool {
    println!("return {}", input);
    return true;
}
