---
title: "hecto，第 4 章：文本查看器"
source: "https://philippflenker.com/hecto-chapter-4/"
author:
  - "[[Philipp Flenker]]"
published: 2024-03-30
---
## 目录

- [介绍](hecto.md)
- [第 1 章：环境搭建](hecto-chapter-1.md)
- [第 2 章：进入原始模式](hecto-chapter-2.md)
- [第 3 章：原始输入与输出](hecto-chapter-3.md)
- [第 4 章：文本查看器](hecto-chapter-4.md) 📍 你在这里
- [第 5 章：文本编辑器](hecto-chapter-5.md)
- [第 6 章：搜索](hecto-chapter-6.md)
- [第 7 章：语法高亮](hecto-chapter-7.md)
- [附录](hecto-appendices.md)
- [变更日志](hecto-change-log.md)

## 第 4 章：文本查看器

![flenker-1716646677-0.png|philipp's blog](assets/9GBtYDktJ_-700.webp)

让我们看看能否在本章中将 `hecto` 变成一个文本查看器。

## 表示文本

在程序中正确处理*文本*是一个棘手的话题，你将在本章中看到。现在我们假设*文本*是屏幕上可见字符的序列，比如 `Hello, World!`。我们通俗地将其称为字符串。

Rust 实际上有两种表示这种文本的方式：`str`（称为字符串切片）和 `String`。

一个 [str](https://doc.rust-lang.org/std/primitive.str.html) 指的是内存中的一段字节序列，就这么简单。我们更多与 `&str` 交互，你可能记得，它是指向内存中字节序列的指针。我们称 `str` 为*字面字符串*，称 `&str` 为*字符串切片*。

在与字符串切片交互时，我们只知道很少的几件事：

- 一个指向字节序列在内存中位置的指针
- 存储了多少字节
- 任何 `str` 值始终是有效的 UTF-8
- 每个字节代表一个 UTF-8 数据点  
	……仅此而已。我们在[第 2 章](hecto-chapter-2.md)中知道了什么是字节以及字节序列可能是什么。我们也知道了什么是指针，我们将其比作一张可能藏有宝藏的地图。

让我们在以下 Rust Playground 中探索这一点：

```rust
fn main() {
    let slice: &str = "Hello, World";  
    dbg!(slice.as_ptr());              
    dbg!(slice.len());                 
    dbg!(slice.as_bytes());            
}
```

[Rust Playground 链接](https://play.rust-lang.org/?version=stable&mode=debug&edition=2021&gist=9b33b1870f543d26da81125b436c5d40)

什么是 UTF-8？让我们先退半步。在第 2 章中，我们从键盘上找到的符号走到了所谓的 ASCII，它基本上是一个数字（内存中）和字符（屏幕上）之间的简单映射。然而，即使只是粗略地看一眼 [ASCII 表](https://www.asciitable.com/)，我们就会发现这种映射不足以涵盖所有情况。例如，作为一个德国人，我在那里缺少一些宝贵的变音符号。

所以我们需要一个更长的可能出现在文本中的事物列表。其中一个列表叫做 [Unicode](https://home.unicode.org/)，它相当详尽，并且是当今的事实标准。[^1]

有了 Unicode 列表，我们标准化了到二进制（0 和 1）的转换，以便其他人可以普遍地解释这些位。

这个过程称为*编码*，Unicode 有多种编码形式，包括 UTF-8。Unicode 列表上的一个项目可以编码为 UTF-8，最多占用 4 个字节，所以最后一条要点试图告诉我们的是：`str` 中的一个字节是一个字节，它可能属于最多 3 个其他字节，以映射到 Unicode 列表上的一个条目。

UTF-8 有一个有趣的特性：它完全包含 ASCII。这意味着 ASCII 表上的任何字符在使用 ASCII 编码和使用 UTF-8 编码时具有相同的字节表示。这就是为什么我们在前面的章节中能够专注于 ASCII，尽管 Rust 在底层使用 UTF-8。

顺便说一句，如果你此时觉得关于文本应该如何工作的事情有些*不太*对劲——你可能已经发现了什么。我们很快就会回到这个问题。

正如你可能想象的那样，这种数据结构非常高效。如果你想修改它，它也非常难以使用。本质上，每次你想扩展一个 `str` 时，你都将被迫重新创建一个 `str`。

这就是 `String` 的用武之地。一个 `String` 是一个真正的 `struct`，上面实现了函数。在底层，它维护一个可增长的 `struct`（我们将在本章后面亲自见到）来包含内容，因此，修改一个 `String` 很容易。本质上，它为我们完成了围绕 `str` 的所有繁重工作，正如我上面暗示的那样。

最棒的部分？`String` 实现了一个特定的特质（确切地说是 `Deref<target=str>`），这个特质确保你可以*仅仅通过解引用*（我们通常用 & 符号来做）将你的 `String` 变成 `&str`。这意味着我们不必担心是接受 `&str` 还是 `&String`，或者如何将一个转换为另一个。  
这太棒了，因为它意味着我们可以一直使用 `&str` 工作，只在必要时才特别要求函数传递 `String`。

让我们看看实际效果。

```rust
fn prints_str(str: &str) {
    println!("I only print &strs, and the &str I got is: {str}");
}

fn main() {
    let slice: &str = "Hello!";
    prints_str(slice);
    
    let mut string: String = String::from("Hello!");
    string.pop();
    string.push_str(", World!");
    prints_str(&string);
}
```

[Rust Playground 链接](https://play.rust-lang.org/?version=stable&mode=debug&edition=2021&gist=6f5382263c70502f924fbfc6c117bb84)

## 任务 5：一个简陋的行查看器

让我们稍微扩展 `hecto`，为它提供一些非常基本的行渲染能力。

这是任务：

- 引入一个名为 `View` 的新 `struct`
- 该视图暴露一个名为 `render()` 的方法
- 将 `draw_rows` 的逻辑移入 `render()`。将必要的辅助函数从 `Editor` 移到 `View`。
- 调整逻辑，使第一行读取 `Hello, World!`（暂时不需要在屏幕中间隐藏包名和版本号）
- 应该调用此方法而不是 `draw_rows`。
- 利用关于 `String` 和 `str` 的知识，更改 `Terminal::print` 的签名，使其不再依赖特质。

**代码审查：** [在这里找到我的代码。](https://github.com/pflenker/hecto-tutorial/commit/34f8fed4e34ef99a63ad85165a832a438c88d684)

## 引入缓冲区

这是我们在本章中将逐步构建的内容：

- `Editor`：此组件主要在不同的 UI 组件之间进行调解，在它们之间传递焦点和信息。目前，我们只有全尺寸的 `View`。
- `View`：此组件接收来自 `Editor` 的每个与文本相关的事件，例如字符按键、换行等。`View` 使用此信息来提高其渲染效率，并将修改事件传递给 `Buffer`。
- `Buffer`：此组件将保存我们的文本。我们将从简单的纯文本表示开始，并随着本教程的进展逐渐扩展它，以根据需要容纳更多数据。

`Buffer` 是一个常见的结构，包含文本编辑器修改和显示文本文件所需的一切。`View` 与 `Buffer` 交互以将其渲染到屏幕上。在许多文本编辑器中，你可以轻松地从一个 `Buffer` 切换到下一个，允许你并行打开多个文件。

`Buffer` 的实现细节因文本编辑器而异。例如，[Nano](https://www.nano-editor.org/) 使用一个简单的 `Buffer` 结构，没有其他文档的内部表示。保存文件时，`Buffer` 的内容（不包括用于语法高亮的数据等）被保存到磁盘。另一方面，[Vim](https://www.vim.org/) 具有复杂的内部结构，可以高效地处理操作，即使在大型文件上也是如此，并且包含一个单独的 `Screen Buffer`，表示屏幕上当前可见的内容。这些设计选择反映了每个编辑器的愿景：`Nano` 被设计为一个小的、轻量级的编辑器（因此得名！），适合快速更新配置文件。而 `Vim` 则被设计为一个完整的、功能丰富的文本编辑器，非常适合处理冗长的文件。

在我们继续实现我们的 `Buffer` 之前，我们需要认识另一个有用的 Rust 数据结构。

## “Vector 怎么了，Victor？”

我们的 Buffer 需要动态调整大小——在添加项目时增长，在移除项目时缩小——所以我们需要一种特殊的数据结构。这正是 Rust 中的 String 在内部管理的，使用一种称为[向量](https://doc.rust-lang.org/std/vec/struct.Vec.html)或 `Vec` 的结构。在一个 `String` 内部，有一个 `Vec<u8>`——一个字节的向量，本质上。正如我们在第 3 章中看到的，`u8` 代表无符号 8 位数字，本质上就是字节。所以，当你操作一个 String 时，你实际上是在操作一个字节的向量。

这个 `Vec` 不仅存储字节，还确保它持有的任何数据都可以作为 `&str` 返回，`&str` 是对一串 UTF-8 编码的 Unicode 字符的引用。`Vec` 被设计为高效地管理内存。例如：如果你初始化一个 `Vec` 并在循环中不断添加元素，一种简单的方法可能涉及为每次添加分配新内存，每次都将所有现有项目转移到这个新空间。这将是低效的。

我们需要的是一个数据结构，可以在你添加和移除项目时增长和缩小。一个 `String` 在底层使用这样的数据结构来允许修改它持有的文本。这个数据结构被称为[向量](https://doc.rust-lang.org/std/vec/struct.Vec.html)，`Vec`。一个 `String` 内部维护一个 `u8` 的 `Vec`，这是一种简洁的说法：它使用灵活的数据结构 `Vec` 来持有 `u8`，即无符号 8 位数字。8 位？那是一个字节。所以基本上，它是一个字节的向量，而 `String` 数据结构确保无论这个 `Vec` 保存什么任意数据点，最终当你需要时，你会得到一个 `&str`，它*也*指向内存中的字节，但这些字节保证是 UTF-8 编码的 Unicode 点的一部分。[^2]

`Vec` 在内部试图确保内存高效。考虑这样一个场景：你创建一个新的 `Vec` 并在循环中不断添加。在一个简单的实现中，`Vec` 会为所有旧项目加上新项目分配一些内存，并将旧项目复制过来，但在内部，`Vec` 确保这种数据复制不需要频繁发生。

## 理解编程中的内存管理

如果你从抽象掉这些细节的语言过渡过来，内存管理可能看起来令人生畏。让我们简化一下。

计算机最小的内存单位是位，表示为 0 或 1。例如，正如我们之前看到的，ASCII 字符 `a` 是 `01100001`。要表示两个 `a`（`aa`），它们被顺序放置：`0110000101100001`。知道每个字符的大小和起始点至关重要，因为不同字符的位可能是相邻的。

这就是指针的用武之地。它们像地图一样，指示数据的起始点和大小，使数据检索高效。考虑 `aa` 的例子；一个指针只需要起始位置和一个字符的大小就能定位每个 `a`。

添加更多字符，比如 `aaa`，并不简单。如果最后一个 `a` 后面的内存正在使用中，你需要更多空间。这需要向操作系统请求额外的内存来容纳新字符，然后复制之前的 `a`，然后再添加下一个——这比简单地使用可用内存要慢。  
Rust 在编译器级别解决了常见的内存管理挑战：

- 它防止写入超出分配的内存。
- 它小心地管理指针，以避免通过一个指针修改数据并通过另一个指针访问过时数据等错误。
- Rust 的所有权模型，包括移动语义，有效地跟踪和管理指针，减少因过时或冲突的数据访问而产生的错误。

相比之下，像 C 这样的语言需要手动内存管理，而 Objective-C 使用引用计数（`retain` 和 `release`）来管理内存。Java 和 JavaScript 使用垃圾收集器自动清理未引用的对象。

例如，JavaScript 对字符串使用指针数组，向开发者隐藏了复杂性。虽然它看起来像一个字符串数组，但技术上它是一个指向字符串的指针数组，类似于 Rust 的 `Vec<String>`，但不暴露底层指针。

继续我们关于 `Vec` 的讨论，让我们考虑 Rust 的 `Vec` 如何处理数据：  
如果每次都将现有元素复制到新的内存位置，向 `Vec` 添加元素可能看起来效率低下。相反，`Vec` 通过最初分配比当前所需更多的内存来优化这个过程。这个额外的空间允许 `Vec` 存储额外的元素，而不需要每次都复制整个数组。只有当这个容量被完全利用时，它才会分配更多空间并复制元素。因此，`Vec` 中的项目数量并不直接表明它使用的内存量；分配的内存通常超过即时需求以提高性能。这种策略平衡了内存使用和处理效率。

## 在 Buffer 中使用 Vec

由于我们计划处理每个文件任意数量的 `Line`，`Vec` 对我们来说是一个极好的数据结构。为了能够正确使用它，我们需要理解一个额外的小概念和语法：  
如上所述，一个 `Vec` 不能持有任意数据。所以我们不能谈论一个 `Vec` 而不谈论它持有什么。我们通过将类型放在尖括号中来表示这一点。`String` 内部使用的数据结构因此是 `Vec<u8>`。

让我们现在玩一玩 `Vec`：

```rust
fn main() {
    let mut vec: Vec<usize> = Vec::new();
    println!("The vec: {:?}",vec);
    for num in 0..100 {
        println!();
        println!("Step {}", num);
        vec.push(num);
        println!("The vec: {:?}", vec);
        println!("Its capacity: {}", vec.capacity());
        println!("Its length: {}", vec.len());
        println!("Item number {}: {}", num, vec.get(num).unwrap());
    }
}
```

[Rust Playground 链接](https://play.rust-lang.org/?version=stable&mode=debug&edition=2021&gist=29a07bd5c0844be3516c2e5d2c241dab)

## 任务 6：一个简单的 Buffer

让我们引入一个简单的 buffer。我们将把它创建为自己的 `struct`，`Buffer`，目前它只持有一个 `Vec<String>`（字符串的向量）。现在，你应该使用 `String::from("Hello, World!");` 手动向 buffer 添加一个包含 `Hello, World!` 的字符串。

在我向你展示任务之前，让我快速展示如何访问 `Vec` 中的元素：

```rust
fn main() {
    let mut vec: Vec<String> = Vec::new();
    vec.pushfrom("Hello, World!");
    vec.pushfrom("Hello, hecto!");
    println!("{}",vec[0]); //变体 1
    if let Some(element) = vec.get(1) {
        println!("{}", element); //变体 2
    }
}
```

[Rust Playground 链接](https://play.rust-lang.org/?version=stable&mode=debug&edition=2021&gist=77c0be69c32fcf16685accf57e6ce5ce)  
我们将在本任务之后讨论变体 2，包括 `Some`。

现在是任务：

- 创建一个名为 `Buffer` 的新 `struct`，它持有一个 `Vec<String>`。
- 确保在这一步中，`Buffer` 持有一个包含 `Hello, World!` 的 `String`。
- 扩展 `View` 以持有一个 Buffer。
- 对 `View` 进行此更改后，你需要使 `View` 成为 `Editor` 的一部分，并将 `render` 从关联函数改为方法（即改为 `render(&self)`）
- 这反过来可能需要你为 `View` 和 `Buffer` 实现或派生 `Default`。
- 更改 `render` 中的逻辑：对于 buffer 中的每个条目，渲染该条目。对于屏幕上的每个空行，仍然渲染一个 `~`。（你仍然可以保持欢迎消息不变，我们很快就会正确处理它）

## 任务 6：代码审查

[这是我的 GitHub 代码。](https://github.com/pflenker/hecto-tutorial/commit/83c71e67f06c7b6113d5560eaad1d2afe87cca8f)

现在让我们讨论访问 `Vec` 的两种不同方法。上面代码示例中的变体 1 仅在你传递的索引实际有效时才有效。否则，程序执行将停止：程序将 panic。  
第二个变体确保即使你的索引无效，程序也不会停止。让我们更仔细地看看这在技术上意味着什么。

我们需要在代码中表示一个函数*可能*返回某些东西——在这种情况下是 `Vec` 的条目——或者不返回。我们已经遇到了一个听起来是个好候选的概念：`Result`，如果有东西存在，它会返回一个标有 `Ok` 的盒子（把这个*东西*放进盒子里），如果发生错误，则返回一个标有 `Err` 的盒子，把错误放在里面。

然而，这里的语义不同：它说“这里应该有东西，没有东西是一个错误”，而我们想表达的是“没有东西是正常的”。这就是 [Option](https://doc.rust-lang.org/std/option/) 所表示的。它的工作方式与 `Result` 几乎完全相同。继续使用盒子的比喻：从 `Vec` 中检索一个条目会返回给你一个标有 `Some` 的盒子，里面有条目，或者一张（实际上是一张纸条）写着 `None` 的盒子，意味着在那个位置 `Vec` 中没有任何东西。

如果你想探索更多——这里有一个例子，这次直接取自文档：

```rust
#![allow(unused)]
fn main() {
    fn divide(numerator: f64, denominator: f64) -> Option<f64> {
        if denominator == 0.0 {
            None
        } else {
            Some(numerator / denominator)
        }
    }
    
    // 函数的返回值是一个 option
    let result = divide(2.0, 3.0);
    
    // 模式匹配来检索值
    match result {
        // 除法是有效的
        Some(x) => println!("Result: {x}"),
        // 除法是无效的
        None    => println!("Cannot divide by 0"),
    }
}
```

[Rust Playground 链接](https://play.rust-lang.org/?version=stable&mode=debug&edition=2021&gist=26c9366c897c1f9dd0b2fbc2c2a73195)

## 任务 7：一个基本的行查看器

是时候读取一个实际文件并将其内容加载到我们的 buffer 中了。为此，我们允许 `hecto` 通过一个命令行参数启动，该参数将为我们保存文件名——例如，`./hecto Cargo.toml` 将为我们打开 `Cargo.toml`。

这里有一个 playground 来从参数中获取文件名：

```rust
fn main() {
 let args: Vec<String> = std::env::args().collect();
 if let Some(first_arg) = args.get(1) {
    println!("Do something with the argument");
 } else {
    println!("No arg given");
 }
}
```

[Rust Playground 链接](https://play.rust-lang.org/?version=stable&mode=debug&edition=2021&gist=75c2dbd7ef5bd0bb7e1154a18ebc6b31)

`collect()` 将 `args()` 的输出变成我们想要的 `Vec`。我们将在本任务之后讨论 `collect()` 的细节。

从文件系统读取是通过一个名为 `std::fs::read_to_string` 的函数完成的，它很有帮助地[打开一个文件并将其读取为字符串](https://doc.rust-lang.org/std/fs/fn.read_to_string.html)。  
`let file_contents = std::fs::read_to_string(filename)?;` 读取 `filename` 处的文件。  
如果成功，则保证 `file_contents` 包含文件内容作为有效的 UTF-8 字符串。你可以使用以下代码方便地遍历此 String（或任何 String，就此而言）中的各个行：

```rust
for line in file_contents.lines() {
     // 对该行做些什么
}
```

希望这能帮助你完成下一个任务。

这是任务：

- 如果 `hecto` 在没有任何命令行参数的情况下被调用，它应该显示之前的欢迎屏幕：左列填充 `~`，包名和版本号居中显示在屏幕上。
- `Editor` 应该检查文件名并将其传递给 `View`，例如传递给一个名为 `load` 的方法。此方法应打开文件并将其内容加载到 `Buffer` 中。（移除硬编码的“Hello, World!”）。
- 你的 `Buffer` 应该暴露一个名为 `is_empty` 的方法，如果 Buffer 为空则返回 true，否则返回 false。
- 如果 `Buffer` 为空，则应显示欢迎消息，否则应隐藏它。
- 空行仍应渲染为 `~`。

## 任务 7：代码审查

[这是我如何解决这个问题的。](https://github.com/pflenker/hecto-tutorial/commit/6e4164e07d760fef2357d4b4f6d004b6c299e536)

让我们更仔细地看看我们如何生成持有参数的 `Vec<String>`。我们通过对 `env::args()` 给我们的东西调用 `collect()` 来做到这一点。

*迭代器*从本教程开始就一直伴随着我们，是时候正式认识它们了。像往常一样，我将尝试在更详细地技术解释之前找到一个非技术例子来说明它。

我有两个孩子，他们永远吃不够糖果。对他们来说，他们会吃一个，要求更多，然后再吃一个。假设我去购物，带着一大袋杂货回来，其中一些是糖果。每次他们喊“下一个！”我就伸手进去，拿出一些美味的零食，直到我拿不出东西，然后我返回空。那时我就是一个迭代器，我正在迭代袋子里的糖果。

一个 `for treat in dads_bag` 循环所做的是在迭代器 `dads_bag` 上调用 `next()`，并将结果作为 `treat` 传递给后面的代码块。如果 `next()` 返回 `None`，那么 `for..in` 循环结束。

重要的是要理解迭代器是*惰性的*，意味着它们通常不知道袋子里是否有东西，直到你调用 `next`——它们根据需要计算项目，而不是一次性全部计算。如果你从袋子里拿出的东西很难管理（例如愤怒的流浪猫，或者可能这个比喻在这里就不成立了），并且你不需要预先知道袋子里实际有多少东西，这可能很方便。

迭代器有一些方便的方法，`.collect()` 是其中之一。它耗尽迭代器并将所有结果存储在一个 `Vec` 中，是我孩子们快速对我喊 `next` 直到我交出所有零食并且他们可以自己管理它们的技术等价物。

`Vec` 也是一个迭代器，它实现了相应的特质。这里有一个 playground 供你更好地理解迭代器：

```rust
pub fn main() {
    let range = 1..10;
    
    println!("First range");
    println!("{:?}", range);
    for n in range {
        println!("{:?}", n);
    }

    let second_range = 1..10;
     println!("Second range");
      println!("{:?}", second_range);
    for n in second_range.take(3) {
        println!("{:?}", n);
    }
     let third_range = 1..10;
    println!("Third range");
    println!("{:?}", third_range);
    for n in third_range.skip(2) {
        println!("{:?}", n);
    }
    
    let fourth_range = 1..10;
    let vec: Vec<u8> = fourth_range.skip(2).take(3).collect();
    println!("Vector");
    println!("{:?}", vec);
     for n in vec {
        println!("{:?}", n);
    }
}
```

[Rust Playground 链接](https://play.rust-lang.org/?version=stable&mode=debug&edition=2021&gist=1eb6d8b6d6d6e297648434aec58e96c4)

## 任务 8：更好的渲染

我们现在能够显示文档的行，但任何比终端宽的行都可能换行。  
我们目前依赖于打印出 `\r\n` 来将 caret 移动到下一行，并继续在那里渲染。我们应该改为将 caret 移动到下一行的开头。  
除此之外，我们目前在每次通过 `repl` 循环时重新渲染整个视图——尽管目前 View 中实际上没有任何变化，只是 caret 被移动了。

我们将在本任务中修复所有这些问题：

- 更新渲染策略以适当地移动 caret，从而摆脱直接打印 `\r\n`。
- 确保只渲染当前行的子字符串。你可以使用切片（即 `current_row[0..width]`，或者你可以利用 `get` 接受范围的事实（即 `current_row.get(0..width)`——或者你使用 String 的 `truncate` 函数，我们之前用它来构造 `welcome_message`。
- 在 `View` 上引入一个标志 `needs_redraw`。如果为 `true`，则需要重新绘制 `View`。如果为 false，则不需要渲染。在适当的位置设置此标志，以确保仅在需要时执行渲染。
- 加分任务：为了在终端调整大小时也能正常工作，你需要对[Resize 事件](https://docs.rs/crossterm/latest/crossterm/event/enum.Event.html)做出反应，并确保 `View` 适当地响应。

**代码审查：** [在这里找到我的代码。](https://github.com/pflenker/hecto-tutorial/commit/0f2c1a781e0ce8389a2a36dd05a79e1a2fff4a9c)

## 错误处理

我们即将进入这样一个领域：编码错误会不时地使 `hecto` panic。这目前有一个烦人的副作用。要模拟它，只需在代码的任何地方放置 `panic!`。`panic!` 是一个宏，它有助于使你的程序崩溃，这与你在做 Rust 不期望的事情时可能发生的情况完全相同，比如切片一个 `str` 超出其边界。

烦人的是，当这种情况发生时，我们仍然处于原始模式，这意味着我们的终端会一团糟。是时候更仔细地研究错误处理、禁用原始模式以及 Rust 中结构体的整个生命周期了。让我们从学习“panic”到底意味着什么开始。

## 理解 Rust 程序执行和 Panic

当你运行一个 Rust 程序时，一切都从 `main` 函数开始，每个后续的函数调用都会向*调用栈*添加一个新条目。这个栈本质上是活动函数及其当前状态的记录——你可以把它想象成一叠实际的纸，每张纸描述每个函数的内容。这里有一个简单的设置：

```rust
fn function_a() {
    function_b();
}

fn function_b() {
    function_c();
}

fn function_c() {
    println!("Inside function_c");
}

fn main() {
    function_a();
}
```

在这个例子中，`main` 调用 `function_a`，然后调用 `function_b`，导致 `function_c` 输出一条消息。每个函数调用都会在调用栈上创建一个新的*帧*，当每个函数完成时，它的*帧*被移除，返回到 `main`。换句话说：当我们从 `main()` 移动到 `function_c()` 时，栈增长，首先是一张描述 `main()` 上下文的纸，然后是 `function_a()`，然后是 `function_b()`，然后是 `function_c()`，然后是由 `println!` 生成的任何函数，在文本打印出来后，每张纸从栈中移除，一张一张地，直到我们回到 `main()` 然后结束程序。

### 使用 panic! 处理错误

Rust 使用 `panic!` 来处理它不知道如何处理的错误。默认操作是开始*展开栈*，这意味着在返回链（或穿过纸堆）时进行清理。清理的一部分是打印关于调用栈的信息（对我们来说显示为*栈回溯*）并处置变量和结构体。  
或者，你可以配置 Rust 在发生 panic 时立即中止执行，这可以使你的程序更小，因为它跳过了展开。你在 `Cargo.toml` 中用 `panic = 'abort'` 设置这个。这种方法更快，但要小心——它可能会让一些资源（如打开的文件或网络连接）悬挂。

### 自定义 Panic 时发生的事情

Rust 为你提供了按照你的方式处理 panic 的选项：

- **捕获展开：** 你可以在展开的过程中捕获 panic，允许你的程序可能继续运行。这是使用 `std::panic::catch_unwind` 处理的。
- **创建自定义 Panic 处理器：** 如果你正在设计不能简单地将控制权交还给操作系统的软件（例如在嵌入式系统中），你可以精确定义在 panic 情况下会发生什么。
	这里是一瞥自定义 panic 处理器可能的样子：
	```rust
	#[panic_handler]
	fn panic(info: &PanicInfo) -> ! {
	    loop {}
	}
	```
	这段代码说，一旦发生 panic，程序就应该无限循环——基本上冻结，由 `->` 表示（意思是：这个函数必须永远不返回）
- **Panic 钩子：** 在 panic 展开之前，你可以让函数介入进行整理，设置更受控的崩溃或清理。

## 何时 Panic

正如我们之前讨论的，在本地处理错误或干净地传播它们通常比导致程序 `panic!` 更可取。然而，这条规则的一个例外适用于防御性编程的背景下。

防御性编程意味着预见并防范意外情况。例如，我们当前的 `Buffer` 实现假设它的 `String` 永远不包含换行符。由于我们特定的实现，这个假设目前成立，但未来的修改（也许是我们自己做的）可能会引入 bug，导致未定义的行为。

为了防止这种情况，我们应该实现检查，在 Debug 构建中 panic——但在 Release 构建中不 panic。建议这样做，以避免将函数内部细节泄漏到函数签名中，并在最终版本中执行不必要的检查。

这里有一个可以试验的例子：

```rust
fn expensive_check() -> bool {
    println!("Performing expensive check!");
    return true;
}

#[cfg(debug_assertions)]
fn other_expensive_check() -> bool {
    println!("Thoroughly performing some other expensive check!");
    return true;
}

#[cfg(not(debug_assertions))]
fn other_expensive_check() -> bool {
    println!("Only superficially performing some other expensive  check!");
    return true;
}

fn main() {
    println!("Release Checks:");
    assert!(expensive_check());
    assert!(expensive_check(), "Expensive check failed in Release Build!");
    assert_eq!(expensive_check(), true);
    assert_ne!(expensive_check(), false);
    
    #[cfg(debug_assertions)]
    {
        println!("Debug Checks:");
    }
    
    debug_assert!(expensive_check());
    debug_assert!(expensive_check(), "Expensive check failed in Debug Build!");
    debug_assert_eq!(expensive_check(), true);
    debug_assert_ne!(expensive_check(), false);
    
    println!("Checks from conditional functions:");
    assert!(other_expensive_check());
    
    println!("All checks passed");
}
```

[Rust Playground 链接](https://play.rust-lang.org/?version=stable&mode=debug&edition=2021&gist=7469cd789a9d25760ce67f10b40c6639)

这里的关键要点是，`#[cfg(debug_assertions)]` 内的代码或 `debug_assert!` 中使用的代码在编译时从最终 release 构建中完全移除，从而优化性能。

## 结构体生命周期

让我们稍微放大 Rust 中的生命周期机制。不出所料，生命周期中的阶段是：创建、使用和销毁。

### 创建

在 Rust 中创建变量和结构体非常直接。对于结构体，我们已经广泛使用了 `default()`。Default 旨在提供一个简单的、零参数构造函数，返回一个没有任何配置的默认类型。

更广泛的设置逻辑，比如启动网络连接或将终端置于原始模式，不应该放入 default。符合语言习惯的方式通常是提供一个名为 `new` 的方法，它可以在需要时接受参数，然后执行我们完全使用结构体所需的设置。

### 使用

本节将涵盖一个重要的 Rust 概念，我们在本教程中几乎不会使用。如果你只是为了构建 hecto 的相关部分而来，可以安全地跳过。

到目前为止，我们的代码只使用了简单的使用模式：我们创建我们需要的东西，一旦我们的函数完成，Rust 就处置任何剩余的变量。这对我们来说如此简单的原因之一是我们还没有广泛地使用引用——我们大多复制我们需要的东西，这在我们的情况下是完全可以接受的。当涉及引用时，事情会变得更复杂。正如我们之前学到的，Rust 确保*内存安全*，意味着我们使用的引用总是有效的。它还确保不发生内存泄漏，这意味着为我们分配的所有内存实际上都是需要的，并且没有分配任何程序任何部分都不再使用的东西。

Rust 确保这一点的方式之一是在编译期间确保我们使用的每个变量的生命周期。让我们检查一个不起作用的例子：

```rust
fn longest(x: &str, y: &str) -> &str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}
```

这段代码不能编译。原因在于 Rust 无法在编译时确定从 `longest` 返回的 `&str` 将存活多久，因此 `longest()` 无法可靠地输出具有可预测生命周期的 `&str`。解决这个问题的一种方法是显式定义 `x` 和 `y` 的生命周期。以下是我们如何强制 `x` 和 `y` 具有相同的生命周期：

```rust
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}
```

这里的 `'a` 是一个泛型生命周期说明符，这个上下文可以理解为只接受两个具有相同生命周期的 `&str`，并返回另一个具有完全相同生命周期的 `&str`。所以在这种情况下，我们把确保正确生命周期的负担放在了调用者身上。

还有生命周期说明符 `'static`，表示此数据在程序的整个持续时间内都存活，还有 `'_`，它是一个匿名生命周期——它不需要命名，因为编译器可以推断它。

### 销毁

一旦达到其生命周期的终点，Rust 就会从内存中移除东西。然而，有时我们有一个结构体离开内存时需要做一些清理——例如，再次禁用原始模式。这就是 `Drop` 特质发挥作用的地方：如果实现了它，它的 `drop()` 方法将在结构体离开内存时被调用。

这带来了一个重要的补充或陷阱：`Drop` 在 panic 期间*也*会被调用，这意味着即使程序崩溃，清理活动也会被执行。然而，`drop()` 中的代码*也*可能会 panic，如果它 panic 了，你将面临“双重 panic”，这迫使 Rust 停止一切并中止。因此，确保你的清理代码是万无一失的，以避免这些场景。

## 备用屏幕

当我们在 `hecto` 中推理生命周期时，让我们也重新审视我们如何设置终端。我们可以通过使用终端的“备用屏幕”功能进一步改进 `hecto`，这是一个古老的功能，它保存当前屏幕缓冲区并切换到我们可以使用的新缓冲区。退出备用屏幕后，原始屏幕恢复。这可以通过 [crossterm](https://docs.rs/crossterm/latest/crossterm/terminal/struct.EnterAlternateScreen.html#) 的 `EnterAlternateScreen` 和 `LeaveAlternateScreen` 来管理。对我们作为开发者来说，一个主要好处是 `clippy` 警告被保留下来，不会被从屏幕上抹去；它们在退出 `hecto` 后仍然可用。

终端提供两个缓冲区：主屏幕和备用屏幕。这种设置为我们的应用程序提供了一个干净的石板，同时确保一旦我们退出备用屏幕并结束程序，用户的原始环境未被触及并准备好进一步使用。

这个功能还使我们能够实现日志功能而不弄乱我们的 UI。我们可以暂时离开备用屏幕，将必要的日志输出到主屏幕，然后返回备用屏幕而不需要重新渲染用户界面，因为它保留了之前的状态。这对于高频日志来说不是一个好主意（无论如何应该写入文件），因为这会产生明显的闪烁效果。

## 任务 9：改进的生命周期管理和错误处理

这个任务将把我们刚刚学到的一切放在一起。作为任务的一部分，我们需要实现一个自定义 Panic Hook，因为 panic 的过程将已经开始写出，然后我们 `Drop` 实现中的清理逻辑才会被调用。  
以下代码做到了这一点：

```rust
let current_hook = std::panic::take_hook();
std::panic::set_hook(Box::new(move |panic_info| {
    //做清理工作，例如 Terminal::terminate()
    current_hook(panic_info);
}));
```

我们将在代码审查期间更深入地探讨这段代码实际上在做什么。

这是任务：

- 重新审视你代码中的生命周期管理：
	- 确保只为无副作用的默认值实现 `Default`，并为更复杂的设置工作实现 `new()` 函数。
		- 确保为结构体离开内存后的必要清理工作实现 `Drop`。确保它不会 panic。
- 重新审视错误处理：
	- 留意你当前正在向上传递的错误，但事实上你可以在本地处理的错误。
		- 留意应该在 Debug 构建中 `panic!` 但不在 Release 构建中 panic 的错误。
		- 留意代码中的隐式约定或假设。你能在 Debug 构建中显式检查它们并 panic 吗？
- 使用备用屏幕：
	- 在 `hecto` 启动时切换到备用屏幕，并在结束时干净地离开它。
		- 实现一个自定义 Panic Hook，以便即使在 panic 时也能正确离开备用屏幕和原始模式。

## 任务 9：代码审查

[这是我如何解决这个问题的。](https://github.com/pflenker/hecto-tutorial/commit/f04927719386af56d278afc02e074ab1c20df22e?diff=split&w=0)

现在，让我们更仔细地看看 panic hook 和上面的代码片段，并学习一些关于闭包的知识：

我们之前讨论过不同的类型——`usize`、`struct` 以及所有好东西——但有时我们需要将*函数*作为参数传递。在我们的情况下，我们需要向 `set_hook` 提供一个在发生 `panic!` 时应该被调用的函数。定义匿名（即未命名）函数的一种方法是闭包（顺便说一下，这样做的语法[取自 Ruby](https://www.sitepoint.com/closures-ruby/)）。

这里有一个 playground 可以玩：

```rust
fn main() {
    let some_closure = |x| x+1;
    let some_value = some_closure(5);
    dbg!(some_value);
    let some_more_sophisticated_closure = |name| {
        println!("Hey, {}, how are you?", name);
    };
    some_more_sophisticated_closure("Philipp");
}
```

[Rust Playground 链接](https://play.rust-lang.org/?version=stable&mode=debug&edition=2021&gist=be2c5802fee314941a2d0d3ee67b74fc)

我认为在玩了一会儿之后，你会很好地理解闭包和函数之间的相似之处。

好的，所以上面的代码定义了一个传递给 `set_hook` 的函数。但显然我们可以从闭包内部访问在闭包外部定义的 `current_hook`。让我们更仔细地看看——这里有一个更简单的例子来说明发生了什么：

```rust
fn main() {
    let message = String::from("Hello");
    let greet = move |name| println!("{}, {}!", message, name); 
    greet("Philipp"); // 这是我的名字，以防你忘了 
}
```

[Rust Playground 链接](https://play.rust-lang.org/?version=stable&mode=debug&edition=2021&gist=65060827f19d5dbf0e83fa8cfa1b9fa7)

如你所见，闭包可以访问它们周围的变量。这很巧妙，但有时也有点难以与 Rust 跟踪引用的方式协调。如果你使用实现了 `Copy` 特质的东西，闭包将有一个可用的副本。如果没有，你可以使用关键字 `move` 来利用我们在第 3 章中讨论的移动语义。在上面的例子中，`greet` 定义后，`message` 对 `main` 来说就不可访问了。

所以上面代码中发生的是，我们首先用 `take_hook` 检索当前的 panic hook，我们使用 `move` 将其传递给闭包，在那里我们最终可以在清理后执行它。为什么我们不当时就使用 `take_hook`，而不是使用 `move` 魔法？想一想，因为解决方案对于理解闭包至关重要。在脚注中找到解决方案。[^3]

最后但同样重要的是，那里有这个奇怪的 `Box::new` 东西。它做什么？

这与我们上面讨论的生命周期有关。在这种情况下，我们需要确保*其他*人获得一个引用，该引用在他们需要的时候保持有效，即使我们的结构体生命周期结束后也是如此。

让我们分解为什么在设置 panic hook 时使用 Box 至关重要。如果我们不这样做，我们会定义我们的闭包并将其传递给 `set_hook`。然而，在某个时候，我们的代码可能会失去对这个闭包的所有引用，导致它被释放。这是有问题的，因为 panic hook 需要在整个程序的生命周期内持续存在——可能比我们特定的 `Editor` 对象长得多。

为了确保闭包的存活，我们需要将其所有权转移给 `set_hook`。我们通过将其放入一个 [Box](https://doc.rust-lang.org/std/boxed/struct.Box.html) 来实现这一点。这将闭包“装箱”在内存中，我们将指向这个装箱闭包的指针传递给 `set_hook`。现在，panic hook 函数拥有盒子内的数据，确保即使我们的原始代码消失了，闭包也能持续存在。

顺便说一句，这加强了我们使 `Editor` 成为从程序开始到结束都存在的一个数据结构的设计决策 <sup><a href="#fn1">[1:1]</a></sup>

现在这里是上面相同代码的注释版本：

```rust
//检索当前 hook，默认情况下它会打印一些漂亮的 panic 信息
let current_hook = std::panic::take_hook(); 
// 定义一个接受 PanicInfo 引用的新闭包。
// 在这里移动闭包内需要的任何外部变量。 
// 将闭包放入 Box 并将其设置为新的 panic hook。
std::panic::set_hook(Box::new(move |panic_info| {
    // 我们的自定义 panic hook 逻辑放在这里
    // 执行原始 hook 以保留默认的 panic 输出行为。
    current_hook(panic_info);
}));
```

## 任务 10：滚动

让我们现在实现滚动。在这一步中，我们暂时不尊重文档边界，而是允许可能无限的滚动。

我们的策略是跟踪视图中文本内的位置，以及当前在终端左上角可见的文本中的左上角位置。每次文本中的位置离开当前屏幕上可见的区域时，我们通过更新 `scroll_offset` 将其带回视图来滚动。我们通过从文本中的位置减去偏移量维度来将文本中的位置转换为屏幕上的位置。

这是任务：

- 处理用户输入：
	- 开始将必要的按键重定向到 `View`
		- 将跟踪位置从 `Editor` 移到 `View` 或 `Buffer`。
		- 解除当前围绕终端边界的限制，允许向底部和左侧进一步移动。
- 实现滚动：
	- 引入一个 `scroll_offset`，它跟踪屏幕上可见的文本中的左上角位置。
		- 每当光标即将离开屏幕时，更新 `scroll_offset`，例如：
		- 如果它向右离开，将 `scroll_offset.x` 增加适当的步数
				- 如果它向左离开，将 `scroll_offset.x` 设置为新位置的 `x`
				- 如果它向底部离开，将 `scroll_offset.y` 增加适当的步数
				- 如果它向顶部离开，将 `scroll_offset.y` 设置为新位置的 `y`。
		- 每次光标可能离开屏幕时调用此功能，包括调整大小。
		- 更新渲染机制，以基于 `scroll_offset` 处理水平滚动的行部分渲染，以及垂直滚动的不同行子集。
		- 从 `View` 向 `Editor` 暴露屏幕上的位置，以正确地将 caret 放置在屏幕上。你可以从文本中的位置减去 `scroll_offset` 来确定屏幕上的位置。

## 任务 10：代码审查

[这是我如何解决这个问题的。](https://github.com/pflenker/hecto-tutorial/commit/6cbc0f000ee5c457de9cca5a6b973b9d2c58b22b?diff=split&w=0) 让我们讨论我代码的三个核心部分，两个与 Rust 相关，一个与设计相关：

- 我引入的命令分发结构
- From 和 TryFrom 特质
- Range 参数

从第一个开始：我没有继续直接对 `crossterm` 的事件做出反应，而是决定引入一个内部命令系统，`Editor` 根据 `crossterm` 事件分发命令。这有两个优点：  
第一个是随着代码的增长，代码将更容易推理。我们预计会有更多关于插入、删除以及我们将需要做的所有其他与文本相关的命令，有了新结构，将来更容易推理更改。  
第二个是我们现在为未来的多种可能的改进铺平了道路。最直接的一个是撤销/重做机制，它可以实现为命令的栈，其中每个新命令（比如 Insert(Char)）在栈上产生其相反的命令（比如 Delete）。撤销然后简单地意味着从栈中弹出最后一个命令并执行它。

### From 和 TryFrom

我在代码中实现了两个特质：[From](https://doc.rust-lang.org/std/convert/trait.From.html) 和 [TryFrom](https://doc.rust-lang.org/std/convert/trait.TryFrom.html)。两者之间的主要区别是 TryFrom 考虑了转换到目标类型可能失败的情况。这些特质有两个有趣的优点：  
第一个是，通过实现例如 `from()`，你免费获得相应的 `into()` 功能。换句话说：如果你为类型 `A` 和 `B` 实现 `from`，你可以同时调用 `B::from(a)` 和 `a.into()` 来将 `a` 变成 `B` 的实例。  
第二个是 Rust 可以使用特质为我们做隐式转换。这在代码中最好理解，所以这里有一个说明这个概念的 Rust Playground：

```rust
#[derive(Debug)]
struct ErrorA;
#[derive(Debug)]
struct ErrorB;

impl From<ErrorB> for ErrorA { //我们允许在这里将 ErrorB 转换为 ErrorA
    fn from(_: ErrorB) -> ErrorA {
        ErrorA{} // 我们通常会从 ErrorB 携带一些信息到 ErrorA。我们这里不这样做，因为我们想说明其他事情。
    }
}

fn do_something_else() -> Result<(), ErrorB> {
    Ok(())
}

fn do_something() -> Result<(), ErrorA> {
    do_something_else()?; // 这里是隐式转换发生的地方：即使这个函数在失败时返回 ErrorB，Rust 也会使用 from 实现将其转换为 ErrorA 并向上传播。
    Ok(())
}

fn main() {
    do_something().unwrap();
}
```

[Rust Playground 链接](https://play.rust-lang.org/?version=stable&mode=debug&edition=2021&gist=ec28acb0c9870e6a8617a8f46f3af427)

### 范围

让我们再谈谈范围。你看到我在代码中使用了参数 `Range<usize>`，这允许我接受像 `0..10` 这样的参数。  
以下是 Rust 提供的范围列表：

- `Range`（`start..end`）——表示从 start 到 end 之前的范围。
- `RangeInclusive`（`start..=end`）——这个包括范围中的 end 值。
- `RangeFrom`（`start..`）——表示从 start 开始并无限持续的范围。
- `RangeTo`（`..end`）——表示从开始到 end（但不包括 end）的范围。
- `RangeToInclusive`（`..=end`）——从开始到包括 end。
- `RangeFull`（`..`）——表示该类型可能的整个范围，本质上是从开始到结束。

## 任务 11：将光标吸附到行尾

我们现在已经扩展了代码，以清楚地区分 caret 在屏幕上的位置和插入点在文本中的位置。  
现在让我们确保这个 `location` 只指向文本中的有效位置，例外是我们允许点在行尾最后一个字符之后，以及在文档末尾下方一行，以便用户可以轻松添加新字符。

在我们深入之前，一句警告：这个任务只适用于 ASCII 文本。如果你遇到非 ASCII 文本的任何问题，请放心，我们将在下一个任务中解决这个问题。

我们将把这个任务分解为简单的水平和垂直移动，然后是更复杂的移动。

这是任务：

- 水平移动：
	- 对于简单的箭头按键，确保你不能越过行尾最后一个字符的末尾，或行的开头。
		- `Home` 应该带你到行的开头，`End` 到行尾之后一个字符。
- 垂直移动：
	- 对于简单的箭头按键，确保你不能越过文档下方一行
		- 对于 `PgUp` 和 `PgDown`，将点向上或向下移动视图的高度，但最多到顶部或文档末尾下方一行。
		- 如果你从长行移动到较短的行，确保通过将点吸附到较短行的末尾来正确地垂直放置点。
- 复杂移动：
	- 在行首按左键应将点移动到上一行的末尾。
		- 在文档开头按左键应该什么都不做。
		- 在行尾按右键应将点移动到下一行的开头。
		- 在最后一行末尾按右键应将点移动到文档下方一行的开头。从那里，你应该只能向上移动，不能进一步向下。

**代码审查**：[这是我如何解决的。](https://github.com/pflenker/hecto-tutorial/commit/cb50da980a2287821f4a6e959c82acf895a64a1f?diff=unified&w=0)

## 字素入门

正如在上一个任务中解释的那样，上面的代码不适用于非 ASCII 字符。  
只需拿以下文本，尝试在 `hecto` 中打开它，看看滚动和将光标吸附到行尾是如何工作的：

```
hecto is great!
hecto is 𝒻𝒶𝓃𝒸𝓎!
hecto is ѕ¢αяу!
ʎzɐɹɔ sı oʇɔǝɥ
```

为了更好地理解发生了什么，我为你准备了一个 Rust Playground：

```rust
fn main() {
    let great = "hecto is great";
    let fancy = "hecto is 𝒻𝒶𝓃𝒸𝓎";
    let scary = "hecto is ѕ¢αяу";
    let crazy = "ʎzɐɹɔ sı oʇɔǝɥ";
    dbg!(great.len());
    dbg!(fancy.len());
    dbg!(scary.len());
    dbg!(crazy.len());
}
```

[Rust Playground 链接](https://play.rust-lang.org/?version=stable&mode=debug&edition=2021&gist=22b3dec61998a23f86040c756a4d1d07)

我们可以观察到的是，我们对字符应该是什么的直观理解与 Rust 认为的字符串长度不匹配。按照我们的直观理解，上面所有字符串的长度应该相同：14，每个可见字符一个。相反，所有这些字符串的长度都不同！

部分原因是 UTF-8 的工作方式，正如我们在本章前面已经讨论过的。回顾一下，UTF-8 编码的字符没有固定宽度：它们最多可以占用 4 个字节来存储。任何 ASCII 字符都适合 1 个字节，任何非 ASCII 字符可能不适合。字符串上的 `len()` 函数返回其中的字节数，而不是可见字符数——这与我们的直观理解不符。

我们已经简要讨论过，在 Rust 中 Char 总是占用 4 个字节——这就是原因。Rust 在内部为字符使用 UTF-32，这是另一种像 UTF-8 的编码，不同之处在于它是固定宽度的，因此总是可以可靠地容纳 UTF-8 编码的字符。

我们在前面章节中研究过的小写字母 a，在 ASCII 和 UTF-8 中表示为 `01100001`。在 UTF-32 中，它将表示为：`00000000 00000000 00000000 01100001`。

那么为什么我们不直接使用 UTF-32 来编码字符串呢？这将使我们能够一次以 4 个字节的块处理字符，对吧？嗯，一方面，这意味着普通 ASCII 文本在内存中将不必要地膨胀。然而，现实甚至更复杂。

让我们再看看一个 Rust Playground，以更好地理解事情：

```rust
fn main() {
    let love = "We all love hecto 🧑🏿‍🤝‍🧑🏿";
    dbg!(love.len());
}
```

[Rust Playground 链接，它也遭受了我们在这里调查的问题](https://play.rust-lang.org/?version=stable&mode=debug&edition=2021&gist=2f0222abfa41a9fc8ec9ba120bc6d95e)

我们预期这个字符串最多 22 个字节长：18 个用于 ASCII 文本，最多 4 个用于 emoji，基于我们的假设，即 emoji 是一个 Character，而 Character 占用 4 个字节。  
但现实是这个字符串有 44 个字节长——仅 emoji 就占了 26 个字节！

那么 emoji 中存储了什么？

让我展示给你看：

```rust
fn main() {
 let adult = "🧑";
    let skin_type = "🏿";
    let hand_shake = "🤝";
    let zero_width_joiner = "\u{200d}";
    
    println!("Output: {skin_type}  Len: {} - Skin Type",  skin_type.len());
    println!("Output: {hand_shake} Len: {} - Hand Shake",  hand_shake.len());
    println!("Output: {zero_width_joiner}   Len: {} - Zero Width Joiner", zero_width_joiner.len());
    println!("Output: {adult} Len: {} - Adult", adult.len());
    println!("Output: {adult}{skin_type} Len: {} - Adult, Skin Type", adult.len() + skin_type.len());
    println!("Output: {adult}{zero_width_joiner} Len: {} - Adult, Zero Width Joiner", adult.len() + zero_width_joiner.len());
    println!("Output: {adult}{zero_width_joiner}{hand_shake}{zero_width_joiner}{adult} Len: {} - Adult, Zero Width Joiner, Handshake, Zero Width Joiner, Adult",adult.len() + zero_width_joiner.len() + hand_shake.len() + zero_width_joiner.len() + adult.len());
    println!("Output: {adult}{skin_type}{zero_width_joiner}{hand_shake}{zero_width_joiner}{adult}{skin_type} Len: {} - Adult, Skin Type, Zero Width Joiner, Handshake, Zero Width Joiner, Adult, Skin Type", adult.len() + skin_type.len() +  zero_width_joiner.len() + hand_shake.len() + zero_width_joiner.len() + adult.len() + skin_type.len());
}
```

[Rust Playground 链接](https://play.rust-lang.org/?version=stable&mode=debug&edition=2021&gist=d5717672e32dfbfb7059b040fe088051) - 示例取自并改编自[这里](https://stackoverflow.com/questions/71163103/i-need-to-know-what-are-the-more-than-four-bytes-character-in-utf-16-and-what-is)。

`\u{200d}` 告诉 Rust 在这里使用 Unicode 点 U+200D，即[零宽度连接符](https://unicode-explorer.com/c/200D)。当你研究上面的例子时，它的目的应该变得明显。

Emoji 是说明这里发生什么的好方法：我们在屏幕上看到的一个字符在技术上要复杂得多。这不仅适用于 Emoji：当处理阿拉伯语或印度语系文字时，我们会面临同样的问题。

现在让我们理清一些词汇。我们想要在屏幕上显示的一个独立单元就是我们所说的*字素*。正如我们现在所了解的，我们无法轻易地从构成*字符串*的字节序列中推导出*字素*的数量。而*字素*和*字符*不一定相同——Unicode 对字符的定义是模糊的，而 Rust 的 Character 数据类型肯定不是字素——它是一个 Unicode 标量值，基本上是 Unicode 中的一个点，例如零宽度连接符。在 Rust 中没有一个单独的 Character 代表上面的 emoji。多个 Character 可以组成一个*字素簇*，比如我们用上面的代码构建的 🧑🏿‍🤝‍🧑🏿 emoji。

文本编辑，这是我们最终想要做的，在字素混入的情况下变得相当困难。删除一个字节可能会导致 Rust panic，因为结果将不再是有效的 UTF8 字符串。删除 4 个字节来规避这个问题会产生不想要的结果。你自己看看：

```rust
fn main() {
    let emoji = "🧑🏿";
    //let fragment = &emoji[0..emoji.len() - 1];  //删除一个字节会 panic
    let fragment = &emoji[0..emoji.len() - 4];    //假设 Char=Grapheme 并移除 4 个字节只移除肤色
    println!("{}", fragment);
}
```

[Rust Playground 链接](https://play.rust-lang.org/?version=stable&mode=debug&edition=2021&gist=c64f145ee3e93138b8b343001d731acc)

总结我们面临的问题：如果我们使用 Rust 内部的 `String`，`len()` 的返回值对我们来说几乎毫无意义：它指的是字节数，而不是其中的字符数或字素数。

## unicode_segmentation 入门

我们将使用一个 crate 来帮助我们处理字素：[unicode_segmentation](https://docs.rs/unicode-segmentation/latest/unicode_segmentation/index.html)。

让我们在 Rust Playground 中认识这个 crate：

```rust
use unicode_segmentation::UnicodeSegmentation;

fn main() {
    let text = "👋Hello, World🌍!";
    let graphemes = text.graphemes<Vec<&str>>(;
    println!("Graphemes: {:?}", graphemes);
    println!("Byte length: {}", text.len());
    println!("Grapheme length: {}", graphemes.len());
    println!("Slice-Based Substring: {}", &text[8..13]);
    println!("Grapheme-Based Substring: {:?}", graphemes[8..13].concat());
}
```

[Rust Playground 链接](https://play.rust-lang.org/?version=stable&mode=debug&edition=2021&gist=33b92ab22248acd5199fe02a4b486adf)

Unicode_segmentation 为我们提供了一个遍历字符串字素簇的迭代器，以及一个特质，允许我们通过在 `str` 上调用 `graphemes()` 来访问这个迭代器（我们必须向该函数传递 `true`，否则字素检测不会总是按预期工作）。

## 任务 12：正确的字素处理（第 1 部分）

让我们利用这些新知识来改进围绕字素簇的滚动处理。我们还没有完全修复它——围绕 Emoji 和其他字素类型滚动还需要一个额外的步骤——但我们已经可以显著改进它。

这是任务：

- 将 `unicode_segmentation` 添加到你的项目中。
- 更改处理子字符串和行长度的逻辑，使其基于字素，而不是字节长度。
- 你可以使用[这个文本文件](https://raw.githubusercontent.com/pflenker/hecto-tutorial/a33a32990a4f7f75296431c358acb52bee0cc93b/test-graphemes-1.txt)来测试 `hecto`。

## 任务 12：代码审查

我的代码使用了一种非常直接且低效的方法来解决这个问题。这里棘手的部分是有效地使用迭代器函数来获得你想要的结果。  
作为一个小的预示，我已经可以告诉你，我将在下一步中使我的实现更高效。

当你将 `unicode_segmentation` 添加到你的项目时，你注意到关于 `#![no_std]` 的文本了吗？[crate 描述](https://docs.rs/unicode-segmentation/latest/unicode_segmentation/index.html)提到它可以在带有此属性的 crate 中使用。这是什么意思？

让我们从描述 `core` 开始。`core` 是 Rust 的库，包含绝对必要的东西，如 `Option`、`Result`、宏和其他东西。它在任何 Rust 环境中都可用。`core` 不假设你需要操作系统支持，因为你可能正在自己构建操作系统！

`std` 包含 `core` 中的所有内容，但添加了大量其他假设操作系统级功能的东西，如线程、网络、堆内存分配、文件 I/O 等。基本上，`std` 是 `core` 加上这些依赖操作系统的标准功能。为了减轻确定什么是 `core`、什么是 `std` 的负担，`std` 重新导出了 `core` 中的所有内容。

通过在主 `main.rs` 中设置 `#![no_std]`，可以为非标准环境（例如嵌入式系统）编译，但它要求你的依赖也能在没有 `std` 的情况下工作。所以如果你曾经计划为嵌入式系统工作，你现在知道如何让 emoji 在那里工作了！

## 字素簇宽度

说到 emoji：它们还*不能*完全工作。任何包含 Emoji 的行仍然会有滚动问题，尽管我们尽了最大努力：你将无法在那里完全滚动到行尾。  
这也适用于某些其他字素簇。我们现在已经纠正了 `hecto`，使行的“长度”指的是屏幕上可见字素簇的数量，但我们还没有考虑到字素簇可以有不同的宽度！

Unicode 中的宽度有以下几种：

- **零宽度字符：** 像零宽度连接符（ZWJ，我们之前遇到过）这样的字符用于文本格式化，没有宽度，意味着它们在视觉上不占用空间。同样（某种程度上）适用于不可打印字符。由于它们是不可打印的，它们没有宽度。
- **半角（窄）字符：** 代表标准可打印字符，在等宽环境（如终端）中通常宽度为 1。这是基本文本处理中最常见的情况。
- **全角（宽）字符** 这包括来自复杂文字（如中文、日文和韩文）的字符。它们是双宽度的，因为在等宽字符网格中它们占用两个空格，这对于文本渲染中的正确对齐至关重要。这里是全角字符的拉丁字母：ＡＢＣＤＥＦＧＨＩＪＫＬＭＮＯＰＱＲＳＴＵＶＷＸＹＺ。
- **模糊：** 此类别中的字符通常根据它们出现的上下文或字体而窄或宽。例如，某些符号，如中点 `·` 或各种破折号 `-`，根据字体渲染为窄或宽。
- **中性：** 这些字符通常占用*一些*空间，但多少取决于上下文或字体。Emoji 属于此类别，空格或制表符也是如此。

我们将需要另一个 crate 来帮助我们：`unicode-width`。让我们在 Rust Playground 中认识它：

```rust
use unicode_width::UnicodeWidthStr;
fn main() {
    let zero = "\x1b";              // Esc，见第 3 章
    let half = "Ä";                 // 德语 A 带变音符号
    let full = "Ａ";                // 像一个 A，但更宽
    let neutral_1 = "👋";           // 普通 Emoji
    let neutral_2 = "🧑🏿‍🤝‍🧑🏿"; // 复合 Emoji
    let ambiguous = "·";            // 中点
    println!("Type\tChar\tWidth\tWidth (CJK)");
    println!("Zero\t{zero}\t{}\t{}", zero.width(), zero.width_cjk());
    println!("Half\t{half}\t{}\t{}", half.width(), half.width_cjk());
    println!("Full\t{full}\t{}\t{}", full.width(), full.width_cjk());
    println!("N(1)\t{neutral_1}\t{}\t{}", neutral_1.width(), neutral_1.width_cjk());
    println!("N(2)\t{neutral_2}\t{}\t{}", neutral_2.width(), neutral_2.width_cjk()); 
    println!("A \t{ambiguous}\t{}\t{}", ambiguous.width(), ambiguous.width_cjk()); 
}
```

[Rust Playground 链接](https://play.rust-lang.org/?version=stable&mode=debug&edition=2021&gist=8bc95a5948e062344c4030886c6424f3)  
`cjk` 代表中文、日文和韩文，因为这些文字可能包含宽度模糊的字符。

Emoji 仍然让我们头疼。好消息是，“普通” emoji 通常可以被认为是全角的。坏消息是，对于复合 emoji，一切都不确定了。原因是 emoji 的渲染方式取决于字体：如果字体支持复合 emoji，它会被渲染为宽字符；如果不支持，它会渲染构成它的所有部分。Rust playground 很好地展示了这一点，因为它使用单独的字体——输入字段不支持复合 emoji，输出字段支持。

## hecto 中的字符宽度

Hecto 将只支持窄字符和宽字符。

**零宽度字符**将使用窄替换字符渲染。无论如何，这使导航和与它们的交互更容易。

我们对**模糊字符**的策略将遵循 Unicode 的建议。建议是在非 CJK 上下文中，或在无法可靠确定上下文的情况下，将这些模糊字符视为窄字符，所以我将使用 `width` 而不是 `width_cjk`。

对于**中性字符**，`unicode-width` 将对它是宽字符、窄字符还是完全其他东西做出有根据的猜测。如果它是 0，我们如上处理。如果它大于 2，我们将其视为宽字符。这意味着 `hecto` 将在字体不支持复合字符的终端上出现视觉故障。

## 任务 13：支持字符宽度

在这个任务中，我们将正确地支持字符宽度。根据你的代码跟随我的实现步骤的紧密程度，你的具体步骤将与我的不同，但最终目标保持不变：我们希望正确支持不同宽度的字符。

我们的策略是将存储 `Line` 的完整字符串的 `String` 替换为每个字素的自定义数据结构，该结构存储字素本身、其由 `hecto` 近似的宽度以及一个潜在的替换字符。我们将尝试为任何无法在屏幕上正确显示的字符选择一个合适的替换字符，无论是因为它是零宽度的，还是因为它是像 Bell 这样的控制字符。

这个结构看起来像这样：

```rust
enum GraphemeWidth {
    Half,
    Full
}

struct TextFragment {
    grapheme: String,
    rendered_width: GraphemeWidth,
    replacement: Option<char>
}
```

我们需要更新 `from` 以遍历传入字符串的字素簇并生成我们内部的 `Vec<TextFragment>`。对于 `unicode-width` 识别为宽度为 0 的字符，我们将添加一个替换字符来显示，而不是实际的字素。在这个任务中，我们只将任何 0 宽度字素替换为中点（`·`）。我们稍后将添加更复杂的处理。

我们的 `get` 函数需要返回给定范围内可见的字素。为了实现这一点，我们需要遍历我们的向量，跟踪我们处理的每个字符的宽度，并返回在给定范围内开始和结束的字素。我们还需要特别注意屏幕的边界：如果一个宽字符被屏幕边缘截断，我们将打印一个水平省略号（`⋯`）来表示发生了截断。

我们对 `Location`、`Position` 和 Offset 的看法需要稍微改变。`Location` 现在将指向给定行中的单个字素。另一方面，`scroll_offset` 将不再指文本中的位置——它将指向一个充满单元格的无限网格上的一个点，其中每个字素可以填充这些单元格中的一个或两个。或者用更简单的话说：如果我们想象我们的终端无限延伸，`scroll_offset` 指的是那个无限终端上的行和列。这意味着 a) `Position` 不再只描述可见屏幕上的位置，而是那个网格上的位置，b) `scroll_offset` 将不再是 `Location`，而是 `Position`，c) 我们需要一种方法来使用这些结构来正确处理 caret 放置、文档内移动和滚动。

为此，我们需要一种将 `Location` 转换为 `Position` 的方法。我们通过将给定 `Location` 的当前 `x` 之前的所有宽度相加来实现这一点，以落在正确的单元格上。由于文档中的每一行在网格上恰好占一列，我们可以简单地采用 `y` 的值来确定正确的行。

有了这个变化，我们的滚动逻辑需要将 `Location` 转换为 `Position` 并基于此工作，以确定 caret 是否移出屏幕，并相应地更新 `scroll_offset`。为了确定 caret 在屏幕上的位置，我们需要从 `scroll_offset` 中减去当前字素在网格上的 `Position`。

我们不需要触及我们遍历文本的逻辑。这意味着如果行包含不同宽度的字符，当我们从一行更改为另一行时，我们不会总是保持正确的垂直位置。但我们期望要么处理一致的宽度，要么只偶尔有 emoji，所以在大多数情况下，从一行到下一行的偏移是可以接受的。

这是任务：

- 将 `unicode-width` 添加到你的项目中。
- 如上所述重构你的代码。
- [用这个文件测试你的更改。](https://raw.githubusercontent.com/pflenker/hecto-tutorial/7a8f2a734518ad016e5ed7ea5e804a3a5d718ce8/test-graphemes-2.txt)
- 确保你的结构体、其成员、函数和变量命名适当，以反映变化。  
	**代码审查：** [这是我如何做的。](https://github.com/pflenker/hecto-tutorial/commit/975c1dc8dfacff8ef2791d8ff0b47f0f6e8a42d8)

## 任务 14：一个文本查看器

让我们用一些最后的润色来结束这一章，并改进我们处理不可见字符的方式。现在，我们正在用中点替换其中一些，但不是全部。我们还以有点奇怪的方式处理一些零宽度字符：如果你有一个由 10 个零宽度连接符组成的字符串，`unicode-segmentation` 将它们识别为*一个*字素，因此我们用*一个*中点替换它们。  
然而，`hecto` 旨在成为一个文本编辑器，主要焦点是显示和编辑可见文本。如果有人希望编辑不可见文本，`hecto` 不适合他们。

尽管如此，输出不可见字符可能会对终端产生奇怪的副作用。例如，一个制表符（`\t`）可能会在终端上占用奇怪数量的空格，这会导致视觉故障。如果我们不替换转义字符（正如我们所学，它在 ASCII 和 Unicode 中都是完全有效的字符），它将被直接发送到终端作为潜在的转义代码，设置一些奇怪的终端行为。顺便说一句，这就是为什么如果你错误地将二进制文件作为文本输出到终端，终端往往会完全混乱的原因。

制表符行为如此奇怪的原因再次把我们带回到机械打字机：制表位——防止带有固定纸张的滚筒越过某个点的物理阻挡物——对于实现文本的一致对齐至关重要，例如在打字简历时。由于制表符如此古老，它们的处理方式因出现的位置而异。

Unicode 为我们提供了丰富的替换字符，如果我们感兴趣的话。ASCII 表中的所有“不可打印”字符都在那里，包括我们的老朋友 ␇ 和 ␛。[你可以在这里找到完整列表](https://www.unicode.org/charts/PDF/U2400.pdf)。

然而，我们的目标不是*完整*替换字符，我们将采取更简单的方法。

这是任务：

- 空格应渲染为空格——这保持不变。
- 将任何制表符（`\t`）替换为空格。
- 将每个其他可见空白（即长度非零的空白）替换为[空心方块](https://www.compart.com/en/unicode/U+2423)（`␣`）。要找出一个字素簇是否是空白，对其调用 `trim()` 以移除所有空白，然后检查 `is_empty()` 是否还有任何东西剩下。
- 对于像 Bell 或 Null 这样的控制字符的零宽度字符，将它们替换为[白色垂直矩形](https://www.compart.com/en/unicode/U+25AF)（`▯`）。你可以对字符使用 `is_control()` 来找出它是否是控制字符，你可以通过对其调用 `.chars()` 将 `String` 和 `str` 都变成其字符的迭代器。
- 对于任何其他零宽度字符，我们继续用中点替换它们。
- 你可以用[这个文件](https://raw.githubusercontent.com/pflenker/hecto-tutorial/ec2b6c025de70cbb8e40dc70db18fd3e0d415f00/test-character-replacement.txt)测试你的更改。

**代码审查：** [这是我如何做的。](https://github.com/pflenker/hecto-tutorial/commit/ec2b6c025de70cbb8e40dc70db18fd3e0d415f00)

## 总结与展望

在功能方面，我们在这一章中的进展相当缓慢。然而，我们确实学到了很多关于 Rust 内部的知识，比如 String、Vector 或 `panic!` 如何工作。我们还学到了很多关于文本处理的陷阱以及为什么 emoji 如此难以支持。

最终结果现在是一个可工作的文本查看器。在[下一章](hecto-chapter-5.md)中，我们将扩展它使其成为文本编辑器。

---

[^1]: 这里更干净的解决方案是在 `new` 中保存 panic hook，并在 `drop` 中恢复它

[^2]: 如果你问自己：为什么是 `u8` 而不是 `Char`？好问题，先记住它——在本章末尾，你将能够回答它。

[^3]: 在闭包被调用时，`take_hook` 将不再返回原始 hook，因为它已经被我们自己的闭包覆盖了。所以我们会陷入递归。