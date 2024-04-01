# lecture5

traits & generics

特性和泛型

`cpp`利用`class`和`namespace`将组件连接起来,利用继承和虚函数来完成多态这件事.

在`rust`中我们使用`traits`来完成这件事

## traits

能做到以下内容

- 展示、`clone&copy`、迭代器、判断等价
- 重载函数
- 默认`implementations`
- 重载运算符

回到链表...

```rust
impl fmt::Display for LinkedList {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut current: &Option<Box<Node>> = &self.head;
        let mut result = String::new();
        loop {
            match current {
                Some(node) => {
                    result = format!("{} {}", result, node.value);
                    current = &node.next;
                },
                None => break,
            }
        }
        write!(f, "{}", result)
    }
}
```

