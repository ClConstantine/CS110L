# lecture3

## ownership in c++

在`C/C++`中自然的出现了内存所有权的概念

![](D:/class/CS110L/pic/2021/lecture3/5.jpg)

![](D:/class/CS110L/pic/2021/lecture3/6.jpg)

但有时候问题会变得非常复杂,需要自定义函数

![](D:/class/CS110L/pic/2021/lecture3/7.jpg)

在`C/C++`中,这些控制均由程序员完成,编译器并不知道你的前后控制条件.

## ownership & reference

同样传递指针，但是在`ownership`的传递生命周期末期会插入特殊的`free`

类似强制声明何处释放变量.

### 代码例子

```rust
let s = String::from("hello");
let s1 = &mut s; // s is unmutalbe
let s2 = &s // can't have another reference after mutable reference
```

修改如下,仍不能通过编译

```rust
let mut s = String::from("hello");
let s1 = &mut s;
println!("{} {}",s1,s);
```

在`rust`中，对于同一个数据，不能同时拥有一个可变引用和一个不可变引用.

修改如下,仍不能通过编译

```rust
fn main() {
    let mut v = String::from("111");
    let s1 = &mut v;
    println!("{} {}",&mut v,s1);
}
```

在`rust`中，为了防止数据竞争,对于同一个数据，不能同时拥有两个可变引用

修改如下,仍不能通过编译

```rust
fn main() {
    let mut v = String::from("111");
    let s1: &mut String = &mut v;
    println!("{}",v);
    println!("{}",s1);
}
```

在 Rust 中，一旦对一个变量创建了可变引用，就不能在**该可变引用**的生命周期内再对同一个变量进行不可变引用，即使这个不可变引用出现在可变引用之前。

修改如下,通过编译

```rust
fn main() {
    let mut v = String::from("111");
    let s1: &mut String = &mut v;
    println!("{}",s1);
    println!("{}",v);
}

```

`Rust`的编译器非常智能,能检测一个变量的最后一次使用.这里认为`v`不在`s1`的生命周期内,故可以通过

本质为可以有多个人读,但是当有人写的时候就不能有人仍在读(在不可变引用的生命周期中)

注意`println!()`和`for`不是一个函数,本质为宏

## Error handling

有如下`cpp`

```cpp
size_t len = packet.length;
void *buf = malloc(len);
memcpy(buf,packet.data,len);
//...
free(buf);
```

注意到如果`malloc`失败,得到`null`,并没有相应的处理.

`deny service attack`

### Handling null

#### Option

`rust`引入了`option`,包含可能为`null`的问题

```rust
fn printyes(x :i32) -> Option<String>{
    if x > 10 {
        return Some("yes".to_string());
    }
    else{
        return None;
    }
}
```

可以如下作为检验

```rust
if feeling_lucky().is_none() { 
    println!("no :(");
}
```

或者`unwrap_or`

```rust
let message = feeling_lucky().unwrap_or(String::from("No :("));
```

#### match

`match`类似`switch`

```rust
fn printyes(x :i32) -> Option<String>{
    if x > 10 {
        return Some("yes".to_string());
    }
    else{
        return None;
    }
}

fn main() {
    match printyes(5) {
        Some(x) => println!("{}", x),
        None => println!("no"),
    }
}
```

当然完整应该这么写

```rust
fn main() {
    match printyes(5) {
        Some(x) => {
            println!("{}", x);
        }
        None => {
            println!("no");
        }
    }
}
```

`Some` 是`rust`中的一个枚举类型（`enum`），用于表示一个可能存在的值。它是 `Option`枚举类型的一部分，`Option`可以用于处理可能为空`None`或包含某个值`Some(value)`的情况。

```rust
pub enum Option<T> {
    None,
    Some(T),
}
```

Option 枚举类型有两个变体：`Some` 和 `None`。`Some` 变体用于包装一个具体的值，表示存在一个值；`None` 变体表示不存在值。

#### 函数写法

在 Rust 中，函数的返回值可以通过函数体中最后一个表达式的结果来确定的

所以这样写是对的

```rust
fn printyes(x :i32) -> Option<String>{
    if x > 10 {
        Some("yes".to_string())
    }
    else{
        None
    }
}
```

这样是不对的

```rust
fn printyes(x :i32) -> Option<String>{
    if x > 10 {
        if x > 1000 {
            Some("123".to_string())
        }
        return Some("yes".to_string())
    }
    else{
        None;
    }
}
```

### Handling errors

在`C`中通过返回值来控制是否出错.

出错返回`-1` ,全局变量`errno`标识出现了什么错误.

但是某个`CVE`就是利用这个特性实现了任意代码执行,这说明了这种控制方式不适合:

- 任意函数随时抛出异常
- 代码不断增加的情况
- 手动管理内存的情况

#### result

这里rust就引入了`Result<T,E>`,包含了可能有错误的信息

```rust
Ok(T)
Err(E)
```



```rust
fn divide(a: i32, b: i32) -> Result<i32, String> {
    if b != 0 {
        Ok(a / b)
    } 
    else {
        Err("Division by zero".to_string())
    }
}

fn main() {
    let result = divide(10, 2);
    match result {
        Ok(value) => println!("Result: {}", value),
        Err(error) => println!("Error: {}", error),
    }
}

```

#### unwrap

- 如果 `Result` 值是 `Ok`，则 `unwrap` 方法会返回包装的结果值。
- 如果 `Result` 值是 `Err`，则 `unwrap` 方法会触发一个 panic，并终止程序的执行。

```rust
fn divide(a: i32, b: i32) -> Result<i32, String> {
    if b != 0 {
        Ok(a / b)
    } else {
        Err("Division by zero".to_string())
    }
}

fn main() {
    let result = divide(10, 2).unwrap();
    println!("Result: {}", result);
}
```

#### expect

类似于 `unwrap` 方法，用于从 `Result` 值中获取包装的实际结果。与 `unwrap` 方法不同的是，`expect` 方法允许我们提供一个自定义的错误消息，以便更好地指示发生错误的原因。

```rust
fn divide(a: i32, b: i32) -> Result<i32, String> {
    if b != 0 {
        Ok(a / b)
    } else {
        Err("Division by zero".to_string())
    }
}

fn main() {
    let result = divide(10, 0);
    let value = result.expect("Failed to divide");
    println!("Result: {}", value);
}
```

