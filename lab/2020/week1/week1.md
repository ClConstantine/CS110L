# week1

## part1

```
cargo run
```

![](..\..\..\pic\2020\week1\1.jpg)

`cargo`是包管理器，会注意到文件的修改并编译

## part2

`rust`中的数字类型包括`i8,i16,i32,i64`(有符号数） ，以及`u8,u16,u32,u64`(存储无符号数字)。

例如:

```rust
let n: i32 = 10;
```

当然可以自动推断

```rust
let n = 10;
```

### 字符串

`rust`两种字符串

* `&str`:指向不可变字符,数据在只读数据段
* `String`:指向堆分配字符

```rust
let s: &str = "123456";
let t: String = "111111";
```

### vector

```rust
let mut v: Vec<i32> = Vec::new();
v.push(2);
v.push(3);
```

需要声明类型

`rust`还支持固定大小的数组。与`c`不同，数组的长度作为数组类型的一部分存储。

```rust
let mut arr: [i32; 4] = [0, 2, 4, 8];
arr[0] = -2;
println!("{}", arr[0] + arr[1]);
```

### 循环

元素遍历

```rust
for i in v.iter() { // v is the vector from above
    println!("{}", i);
}
```

```rust
while i < 20 {
    i += 1;
}
```

`while true`:用`loop`

```rust
let mut i = 0;
loop {
    i += 1;
    if i == 10 { break; }
}
```



### 函数声明

```rust
fn sum(a: i32, b: i32) -> i32 {
    a + b
}

fn main() {
    // do stuff...
}
```

与变量不同,需要为返回值的函数指定返回类型.

`sum` 函数中没有 `return` 关键字,而且少了一个分号.

在 Rust 中，一切都是表达式,所以下面的代码是成立的

```rust
let x = if someBool { 2 } else { 4 };
```

同时下面的代码是错误的

```rust
fn sum(a: i32, b: i32) -> i32 {
    a + b;
}
```

函数是用分号分隔的表达式。因此，该函数实际上包含两个表达式： `a + b` （分号之前）和一个空表达式（分号之后）。由于最后一个表达式为 void，因此该函数最终不会返回任何内容。

同时由于是表达式,下面的代码是成立的

```rust
fn fib(n: i32) -> i32 {
    if n <= 1 { n } else { fib(n-1) + fib(n-2) }
}
```

### for循环详解

首先有一个数组

```rust
    let arr :[i32;5] = [1,2,3,4,5];
    
    for i in arr{
        println!("{}",i);
    }

    for i in arr{
        println!("{}",i);
    }
```

这里由于是`copy`,所以可以成立

然后修改为`vector`

```rust
    let arr:Vec<i32> = vec![1,2,3,4,5];

    for i in arr{
        println!("{}",i);
    }
    // for i in arr{
    //     println!("{}",i);
    // }
```

#### into_iter()

获取所有权,转化为迭代器

第二次遍历会失效,因为`for`循环的本质为语法糖,这里会拿走所有权,编译器自动展开为

```rust
    let iter = IntoIterator::into_iter(arr);

    for item in iter{
        println!("{}", item);
    }
```

#### iter()

可以实现只读循环

```rust
    let arr:Vec<i32> = vec![1,2,3,4,5];

    for i in &arr{
        println!("{}",i);
    }
```

自动展开得到

```rust
    let arr:Vec<i32> = vec![1,2,3,4,5];

    for i in arr.iter(){
        println!("{}",i);
    }
```

#### iter_mut()

实现读写循环

```rust
    let mut arr:Vec<i32> = vec![1,2,3,4,5];

    for i in &mut arr{
        println!("{}",i);
    }
```

展开得到

```rust
    let mut arr:Vec<i32> = vec![1,2,3,4,5];

    for i in arr.iter_mut(){
        println!("{}",i);
    }
```



### Part2实现

实现 `add_n` ，它采用数字向量和一些数字 `n` 。该函数应返回一个新向量，其元素是原始向量 `v` 中的数字，并在每个数字上添加 `n` 。

```rust
fn add_n(v: Vec<i32>, n: i32) -> Vec<i32> {
    let mut new_vec = Vec::new();
    for i in v {
        new_vec.push(i + n);
    }
    new_vec
}
```



实现 `add_n_inplace` ，它与 `add_n` 执行相同的操作，但直接（就地）修改 `v` 并且不返回任何内容。

```rust
fn add_n_inplace(v: &mut Vec<i32>, n: i32) {
    for i in v.iter_mut() {
        *i += n;
    }
}
```



实现`dedup`在向量中就地删除重复元素。如果某个元素在向量中的任何位置重复，则应保留最先出现的元素。需要使用`HashSet`

```rust
fn dedup(v: &mut Vec<i32>) {
    let mut set = HashSet::new();
    let mut i = 0;
    while i < v.len(){
        if set.contains(&v[i]){
            v.remove(i);
        }
        else{
            set.insert(v[i]);
            i += 1;
        }
    }
}
```

![](..\..\..\pic\2020\week1\2.jpg)

![](..\..\..\pic\2020\week1\3.jpg)

## part3

### 读入

```rust
use std::io;

io::stdin().read_line(&mut guess).expect("Failed to read line");
```

从标准输入流中获取一行数据,`expect`代表`Result`中处理潜在错误

### 输出

缓冲区刷新

```rust
io::stdout().flush().unwrap();
```



### collect

将`vector`和`String`相互转化

```rust
let secret_word_chars: Vec<char> = secret_word.chars().collect();

println!("The word so far is {}",
    guessed_word.clone().into_iter().collect::<String>());

println!("You have guessed the following letters: {}",
    guessed_letter.iter().collect::<String>());

```

需要注明类型,否则需要声明`collect`转化的对象.

注意`iter()`和`into_iter()`的区别.

### 闭包

```rust
if guessed_word.iter().all(|&c| c != '-'){
	println!("Congratulations! You guessed the word: {}", secret_word);
	break;
}
```

闭包的定义以一对竖线`||`开始，在竖线中指定闭包的参数。如果有多于一个参数，可以使用逗号分隔，比如`|param1, param2|`。

参数之后是存放闭包体的大括号 —— 如果闭包体只有一行则大括号是可以省略的。

形成匿名函数

```rust
let expensive_closure = |num| {
    println!("calculating slowly...");
    thread::sleep(Duration::from_secs(2));
    num
};
```

严格写法

```rust
let expensive_closure = |num: u32| -> u32 {
    println!("calculating slowly...");
    thread::sleep(Duration::from_secs(2));
    num
};
```

```rust
fn  add_one_v1   (x: u32) -> u32 { x + 1 }
let add_one_v2 = |x: u32| -> u32 { x + 1 };
let add_one_v3 = |x|             { x + 1 };
let add_one_v4 = |x|               x + 1  ;
```

闭包定义会为每个参数和返回值推断一个具体类型。

如果有

```rust
let example_closure = |x| x;

let s = example_closure(String::from("hello"));
let n = example_closure(5);

```

得到

```rust
error[E0308]: mismatched types
 --> src/main.rs
  |
  | let n = example_closure(5);
  |                         ^ expected struct `std::string::String`, found
  integer
  |
  = note: expected type `std::string::String`
             found type `{integer}`

```

第一次使用 `String` 值调用 `example_closure` 时，编译器推断 `x` 和此闭包返回值的类型为 `String`。接着这些类型被锁定进闭包 `example_closure` 中,如果尝试对同一闭包使用不同类型则会得到类型错误。

### 实现

```rust
// Simple Hangman Program
// User gets five incorrect guesses
// Word chosen randomly from words.txt
// Inspiration from: https://doc.rust-lang.org/book/ch02-00-guessing-game-tutorial.html
// This assignment will introduce you to some fundamental syntax in Rust:
// - variable declaration
// - string manipulation
// - conditional statements
// - loops
// - vectors
// - files
// - user input
// We've tried to limit/hide Rust's quirks since we'll discuss those details
// more in depth in the coming lectures.
extern crate rand;
use rand::Rng;
use std::fs;
use std::io;
use std::io::Write;

const NUM_INCORRECT_GUESSES: u32 = 5;
const WORDS_PATH: &str = "words.txt";

fn pick_a_random_word() -> String {
    let file_string = fs::read_to_string(WORDS_PATH).expect("Unable to read file.");
    let words: Vec<&str> = file_string.split('\n').collect();
    String::from(words[rand::thread_rng().gen_range(0, words.len())].trim())
}

fn print_info(counter: &u32, guessed_word: &Vec<char>,guessed_letter: &Vec<char>){
    println!("The word so far is {}",guessed_word.clone().into_iter().collect::<String>());
    println!("You have guessed the following letters: {}",guessed_letter.iter().collect::<String>());
    println!("You have {} guesses left",counter);
    print!("Please enter your guess: ");
}

fn main() {
    let secret_word = pick_a_random_word();
    // Note: given what you know about Rust so far, it's easier to pull characters out of a
    // vector than it is to pull them out of a string. You can get the ith character of
    // secret_word by doing secret_word_chars[i].
    let secret_word_chars: Vec<char> = secret_word.chars().collect();
    // Uncomment for debugging:
    // println!("random word: {}", secret_word);
    
    println!("Welcome to CS110L Hangman!");
    let mut counter = NUM_INCORRECT_GUESSES;
    let mut guessed_word: Vec<char> = vec!['-'; secret_word.len()];
    let mut guessed_letter: Vec<char> = Vec::new();
    let found_flag = false;

    // for c in &secret_word_chars{
    //     print!("{}",c);
    // }
    // println!();

    while found_flag == false && counter > 0 {
        print_info(&counter,&guessed_word,&guessed_letter);
        io::stdout().flush().unwrap();

        let mut guess = String::new();
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");
        let guess = guess.trim().chars().next().unwrap();
        
        guessed_letter.push(guess);

        let mut found = false;
        for i in 0..secret_word.len(){
            if secret_word_chars[i] == guess{
                guessed_word[i] = guess;
                found = true;
            }
        }
        if found == false{
            counter -= 1;
            if counter == 0{
                break;
            }
            println!("Incorrect! You have {} guesses left", counter);
        }

        println!();
        if guessed_word.iter().all(|&c| c != '-'){
            println!("Congratulations! You guessed the word: {}", secret_word);
            break;
        }
    }

    if counter == 0{
        println!("\nSorry, you ran out of guesses! The word was: {}", secret_word);
    }

}
```

