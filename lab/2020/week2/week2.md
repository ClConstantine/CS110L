# week2

## part1

```rust
fn main() {
    let mut s = String::from("hello");
    let ref1 = &s;
    let ref2 = &ref1;
    let ref3 = &ref2;
    s = String::from("goodbye");
    println!("{}", ref3.to_uppercase());
}
```

注意到`s`已经被引用了,应该处于只读状态,不能修改.

```rust
fn drip_drop() -> &String {
    let s = String::from("hello world!");
    return &s;
}
```

注意到这里想要返回一个引用,这是不被允许的

```rust
fn main() {
    let s1 = String::from("hello");
    let mut v = Vec::new();
    v.push(s1);
    let s2: String = v[0];
    println!("{}", s2);
}
```

注意到这里的`v`不能交出所有权,所以要引用,修改如下:

```rust
    let s2: &String = &v[0];
```

## part2

通过计算两个文件的`LCS`来来计算信息的差异.

### 打开文件

```rust
let file = File::open(filename).unwrap();
```

当然除非完全不会`panic`,否则避免使用`unwrap()`

可以使用以下命令

```rust
let file = File::open(filename)?;
```

`?`运算符在 Rust 中通常用于传播错误,如果发生错误自动返回.

本质展开如下

```rust
let file = match File::open(filename) {
    Ok(file) => file,
    Err(err) => return Err(err),
};
```

### 读取行

```rust
for line in io::BufReader::new(file).lines() {
    let line_str = line?;
    // do something with line_str
}
```

在此代码中,`line`是`Result<String, io::Error>`.我们可以使用`?`安全地解开其`Ok`值,就像打开文件时所做的那样.然后,您可以将字符串添加到向量中.

完整代码

```rust
fn read_file_lines(filename: &String) -> Result<Vec<String>, io::Error> {
    let file = File::open(filename)?;
    let mut v = Vec::new();
    for line in io::BufReader::new(file).lines() {
        let line_str = line?;
        v.push(line_str);
    }
    Ok(v)
}
```

测试

```shell
cargo test test_read_file_lines
```

### 实现grid

实现`get`和`set`函数.如果提供的位置在边界内,则`get`应返回`Some(value)`;如果超出边界,则应返回`None`.`set`成功时应返回`Ok(())`.

```rust
    pub fn get(&self, row: usize, col: usize) -> Option<usize> {
        if row >= self.num_rows || col >= self.num_cols {
            None
        }
        else {
            Some(self.elems[row * self.num_cols + col])
        }
    }

    pub fn set(&mut self, row: usize, col: usize, val: usize) -> Result<(), &'static str> {
        if row >= self.num_rows || col >= self.num_cols {
            Err("Out of bound!")
        }
        else{
            self.elems[row * self.num_cols + col] = val;
            Ok(())
        }
    }
```

测试

```shell
cargo test test_grid -- --nocapture
```

### 实现LCS

```rust
fn lcs(seq1: &Vec<String>, seq2: &Vec<String>) -> Grid {
    
    let mut dp = Grid::new(seq1.len() + 1, seq2.len() + 1);

    for i in 0..seq1.len() {
        for j in 0..seq2.len() {
            if seq1[i] == seq2[j] {
                dp.set(i + 1, j + 1, dp.get(i, j).unwrap() + 1).unwrap();
            } else {
                let max_value = max(dp.get(i + 1, j).unwrap(), dp.get(i, j + 1).unwrap());
                dp.set(i + 1, j + 1, max_value).unwrap();
            }
        }
    }
    dp
}
```

这里`set`返回的是`Result`,所以我们用`unwrap()`来处理错误信息,代码保证了不会有问题.

### 实现diff

调用`read_file_lines`函数来读取两个文件的内容.需要在此处调用`expect`并提供适当的错误消息,以处理所提供的文件名之一无效的情况.

```rust
fn print_diff(lcs_table: &Grid, lines1: &Vec<String>, lines2: &Vec<String>, i: usize, j: usize) {
    if i > 0 && j > 0 && lines1[i - 1] == lines2[j - 1]{
        print_diff(lcs_table, lines1, lines2, i - 1, j - 1);
        println!(" {}" ,lines1[i - 1]);
    }
    else if j > 0 && (i == 0 || lcs_table.get(i, j - 1).unwrap() >= lcs_table.get(i - 1,j).unwrap()){
        print_diff(lcs_table, lines1, lines2, i, j - 1);
        println!(">{}" ,lines2[j - 1]);
    }
    else if i > 0 && (j == 0 || lcs_table.get(i, j - 1).unwrap() < lcs_table.get(i - 1,j).unwrap()){
        print_diff(lcs_table, lines1, lines2, i - 1, j);
        println!("<{}" ,lines1[i - 1]);
    }
    else {
        println!()
    }
}
```

### 完成测试

主函注意写点expect,控制文件读写

```rust
fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 3 {
        println!("Too few arguments.");
        process::exit(1);
    }
    let filename1 = &args[1];
    let filename2 = &args[2];

    let lines1 = read_file_lines(filename1).expect("File read failed");
    let lines2 = read_file_lines(filename2).expect("File read failed");

    let lcs_table = lcs(&lines1,&lines2);

    print_diff(&lcs_table, &lines1, &lines2, lines1.len(), lines2.len());
}
```
## part3

```rust
use std::env;
use std::io;
use std::io::BufRead;
use std::process;
use std::fs::File;

fn main()-> io::Result<()>{
    let args: Vec<String> = env::args().collect();
    
    if args.len() < 2 {
        println!("Too few arguments.");
        process::exit(1);
    }
    let filename = &args[1];
    // Your code here :)

    let file = File::open(filename).expect("Open file failed,please check the path!\n");

    let mut line_num = 0;
    let mut word_num = 0;
    let mut char_num = 0;

    for line in io::BufReader::new(file).lines(){
        let line_str = line?;
        line_num += 1;
        char_num += line_str.len();
        word_num += line_str.split_whitespace().count();
    }

    println!("Word count: {}", word_num);
    println!("Line count: {}", line_num);
    println!("Character count: {}", char_num);

    Ok(())

}
```

## part4

