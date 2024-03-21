# lecture2

## Memory Errors

 find errors by reasoning and testing(推理于测试)

### memory Leak

![](D:/class/CS110L/pic/2021/lecture3/1.jpg)

### double free

![](D:/class/CS110L/pic/2021/lecture3/2.jpg)

### dangling pointers

![](D:/class/CS110L/pic/2021/lecture3/3.jpg)

### Iterator Invalidation

![](D:/class/CS110L/pic/2021/lecture3/4.jpg)

## Language and Compiler

`Rust`的编译器实行了严格限制,导致编写某些程序是不可能的.但是`rust`的很多安全保证来源于编译器的检查.编译器也为程序做出了很多优化.

在之后我们会讨论不安全的`Rust`

## Meet Rust

### Ownership

```rust
let a = Bear::get();
let b = a; // now b is responsible for the object,a can not do anthing to the object
```

此时b拥有了所有权

当我们将bear交给函数

```rust
let a = Bear::get();
func(a); //now func is responsible for the object
```

函数将获得bear的所有权,需要释放.

![](D:/class/CS110L/pic/2021/lecture3/9.jpg)

但是`rust`有**借用**的特性

```rust
let a = Bear::get();
func(&a); //now after finish func, the ownership will return to a
```

这里的函数为借用变量,结束后归还.

同时`for`循环的表现也有所不同

```rust
    let v = vec!["1", "2", "3"];
    for x in v {
        println!("{}", x);
    }
    println!("{:?}",v);
```

此时会产生报错,编译器认为`v`的所有权交给了循环,应写成如下形式

```rust
    let v = vec!["1", "2", "3"];
    for x in &v {
        println!("{}", x);
    }
    println!("{:?}",v);
```

所以`rust`有以下特性:

- 每个值会存在一个变量作为`owner`
- 同时只有一个`owner`
- 当`owner`超出了作用域了以后,值会被自动回收(`compiler`插入释放代码)

这样的设计能够防止以下问题:

- memory leaks
- double free
- use after free
- many others... 

但这些特性不是`Rust`独有的.

## Copy

对于存在栈空间上的内容(数值类型、布尔型等等)我们进行`Copy`.

```rust
	let a = 123;
	let b = a;
	println!("a: {}, b: {}", a, b);
```

## Reference

同时：

* 多个不可变引用
* 一个可变引用

类似读者写者问题

`Rust`比较聪明,能够在编译器计算变量的生存周期(从创建到最后一次**使用**)

生命周期结束后会调用特殊的`Drop`函数

```rust
fn change_it_up(s: &mut String) { 
	*s = "goodbye".to_string();
}
fn make_it_plural(word: &mut String) { 
	word.push('s');
}
fn let_me_see(s: &String) { 
	println!("{}", s);
} 
fn main() {
	let mut s = "hello".to_string(); 
	change_it_up(&mut s); 
	let_me_see(&s);
	make_it_plural(&mut s); 
	let_me_see(&s);
}
```

有以下关于`vector`的例子

```rust
fn main() {
	let v = vec![1, 2, 3, 4, 5];
	for i in v.iter_mut(){
		*i = 5;
	}
	println!("{:?}", v);
}
```

这里的`v`是不可变的,但是我们使用了`iter_mut()`,默认需要可变参数,所以会出现错误

这里体现了`rust`的哲学,我们严格强调可变和不可变

## Compiler cost

`rust`的众多检查基于编译时期,所以会增加编译的开销

runtime check -> compilertime check