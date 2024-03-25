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