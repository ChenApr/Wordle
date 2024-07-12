# Honor Code
* 我在 2024-07-07 参考了 abmfy 在 https://github.com/abmfy/wordle/tree/master 编写的代码，引入到了 game.rs 代码中。

* 关于 多关键词排序，我向 ChatGPT 提问了 “rust 中的hashmap 可以降序输出吗” ，

  ChatGPT 回答了:

​	在 Rust 中，HashMap 本身不保证顺序，但你可以通过将键值对提取到一个可排序的集合中来实现降序输出。下面是一个示例代码，展示如何实现这一点：

```rust
use std::collections::HashMap;

fn main() {
    let mut map = HashMap::new();
    map.insert("apple", 5);
    map.insert("banana", 3);
    map.insert("cherry", 7);

    // 提取键值对并排序
    let mut items: Vec<_> = map.iter().collect();
    items.sort_by(|a, b| b.1.cmp(a.1)); // 根据值降序排序
    
    // 输出结果
    for (key, value) in items {
        println!("{}: {}", key, value);
    }
}
```
​	解释：
​	1. 创建 HashMap：插入键值对。
​	2. 提取键值对：使用 map.iter().collect() 将键值对提取到 Vec 中。
​	3. 排序：使用 sort_by 方法根据值进行降序排序。
​	4. 输出结果：遍历排序后的集合并输出键值对。


​	这样，你就可以实现对 HashMap 键值对的降序输出。

​	这启发了我在 main.rs 中对输出最常用五个词的实现。

* 关于 检查数组中是否存在重复元素，我想 ChatGPT 提问了“Vec 有内置的检查重复元素的方法吗？如何用 HashSet 检查数组内是否有重复元素” 

  ChatGPT 回答如下：

  Rust's Vec itself does not have a built-in method to check for duplicate elements, but you can easily implement this functionality using iterators and collections such as HashSet. Here’s how you can do it:

​	Using HashSet to Check for Duplicates
```rust
use std::collections::HashSet;

fn has_duplicates<T: Eq + std::hash::Hash>(vec: &[T]) -> bool {
    let mut set = HashSet::new();
    for item in vec {
        if !set.insert(item) {
            return true;
        }
    }
    false
}

fn main() {
    let vec = vec![1, 2, 3, 4, 4];
    if has_duplicates(&vec) {
        println!("The vector has duplicates.");
    } else {
        println!("The vector has no duplicates.");
    }
}

```
	Explanation
	Function Definition: The function has_duplicates takes a reference to a vector.
	Using HashSet: As elements are inserted into the HashSet, if an element cannot be inserted (because it is already present), the function returns true.
	Main Function: Demonstrates how to use has_duplicates with a vector.
	This approach leverages the properties of HashSet to efficiently check for duplicates.



​	这启发了我在 words_gen.rs 对检查重复元素的函数的实现

* 我在 2024-07-07 参考了 abmfy 在 https://github.com/abmfy/wordle/tree/master 编写的代码，引入到了 keyboade.rs , guesses.rs 和 letter.rs 代码中。这极大启发了我对 egui 的认识，并了解到用闭包和回调函数进行持续渲染刷新的手段。

  