# lecture4

## ownership in memory

由于所有权模型的存在,禁止`shallow copy`(浅拷贝)

![](..\..\pic\2021\lecture4\1.jpg)

当超出生命周期(`{}`决定),会调用特殊的释放函数,类似于`free`和`destructor`的特性,称为`Drop function`

所有权模型中的浅拷贝就是获取所有权,深拷贝则是`clone`

有以下代码

```rust
let julio = 10;
let ryan = julio;
println!("{} {}",julio,ryan);
```

这里没有用到堆,一些数值类型、布尔值类型等等直接存在栈空间上.

仅需栈空间的内容一般进行`copy`操作,直接创造副本,并且没有`drop`特性

## Reference

所有变量默认都是不能修改的.

`mut`关键字能让变量可以修改

![](..\..\pic\2021\lecture4\2.jpg)

所以可以存在引用变量(reference == Borrowing Type),代表就是借的

```rust
let julio = Bear::get(); 
let julio_reference = &julio;
my_cool_bear_function(julio_reference);
```

有如下例子.不可编译

```rust
fn append_to_vector(lst: &Vec<u32>) { 
    lst.push(3); 
} 
fn main() {
    let mut lst = vec![1,2,3];
    append_to_vector(&lst);
}
```

函数借用的时候是不可变变量,修正

```rust
fn append_to_vector(lst: &mut Vec<u32>) { 
    lst.push(3); 
} 
fn main() {
    let mut lst = vec![1,2,3];
    append_to_vector(&mut lst);
}
```

### 规则

在某个生命周期中,只能存在任意多不可变引用或一个可变引用,不能同时存在可变和不可变引用.

目的是累了防止数据争夺,[例子](https://play.rust-lang.org/?version=stable&mode=debug&edition=2018&gist=64d8e83cbbd5429621e345157e857ea1)

```rust
fn main() {
    let mut v = vec![1, 2, 3];
    /* This for loop borrows the vector above to do its work */    
    for i in &v { 
        println!("{}", i);
        v.push(34);
    }
}
```

同时存在可变和不可变,这里`for`是宏

![](..\..\pic\2021\lecture4\3.jpg)

通过所有权模型和引用规则,我们可以避免很多内存问题

### 细节

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

在 Rust 中，一旦对一个变量创建了可变引用，就不能在该可变引用的生命周期内再对同一个变量进行不可变引用，即使这个不可变引用出现在可变引用之前。

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

注意`println!()`不是一个函数,本质为宏