---
title: "hecto, Chapter 4: A Text Viewer"
source: "https://philippflenker.com/hecto-chapter-4/#fnref1:1"
author:
  - "[[Philipp Flenker]]"
published: 2024-03-30
---
## Table of Contents

- [Introduction](https://philippflenker.com/hecto/)
- [Chapter 1: Setup](https://philippflenker.com/hecto-chapter-1/)
- [Chapter 2: Entering Raw Mode](https://philippflenker.com/hecto-chapter-2/)
- [Chapter 3: Raw Input and Output](https://philippflenker.com/hecto-chapter-3/)
- [Chapter 4: A Text Viewer](https://philippflenker.com/hecto-chapter-4/) 📍 You are here
- [Chapter 5: A Text Editor](https://philippflenker.com/hecto-chapter-5/)
- [Chapter 6: Search](https://philippflenker.com/hecto-chapter-6/)
- [Chapter 7: Syntax Highlighting](https://philippflenker.com/hecto-chapter-7/)
- [Appendices](https://philippflenker.com/hecto-appendices/)
- [Change Log](https://philippflenker.com/hecto-change-log/)

## Chapter 4: A Text Viewer

![flenker-1716646677-0.png|philipp's blog](https://philippflenker.com/img/optimized/9GBtYDktJ_-500.webp)

Let's see if we can turn `hecto` into a text viewer in this chapter.

## Representing Text

Handling *text* correctly in a program is a tricky topic, as you will see over the course of this chapter. Let’s assume for now that *text* is a sequence of visible characters on screen, like `Hello, World!`. We colloquially refer to this as a string.

Rust actually has two ways of representing this text: `str` (called string slices) and `String`.

A [str](https://doc.rust-lang.org/std/primitive.str.html) refers to one sequence of bytes in memory, it’s as barebones as that. We interact more with `&str`, which, as you might remember, is a pointer to the sequence of bytes in memory.. We call `str` a *literal string* and a `&str` a *string slice*.

When interacting with a string slice, we know only precious few things:

- a pointer to where the byte sequence is located in memory
- How many bytes are stored
- Any `str` value is always valid UTF-8
- Each byte represents a UTF-8 data point  
	… and that’s it. We know from [Chapter 2](https://philippflenker.com/hecto-chapter-2/) what bytes are and what a byte sequence might be. We also know what a pointer is, something we likened to a map where treasure might be hidden.

Let’s explore this in the following Rust Playground:

```rust
fn main() {
    let slice: &str = "Hello, World";  
    dbg!(slice.as_ptr());              
    dbg!(slice.len());                 
    dbg!(slice.as_bytes());            
}
```

[Link to Rust Playground](https://play.rust-lang.org/?version=stable&mode=debug&edition=2021&gist=9b33b1870f543d26da81125b436c5d40)

What is UTF-8? Let’s take half a step back first. In Chapter 2, we came from the symbols we find on our (QWERTY or similar) keyboards to something called ASCII, which is basically a simple mapping between a number (in memory) and a character (on screen). However, even a superficial glance at the [ASCII sheet](https://www.asciitable.com/) shows us that this mapping is not sufficient to cover all kinds of cases. As a German, for example, I am missing söme preciöus ümlaüts in there.

So we need a longer list of things that might appear in texts. One of these lists is called [Unicode](https://home.unicode.org/), which is pretty exhaustive and the de-facto standard today.[^1]

With the Unicode list, we standardise the conversion into binary (0s and 1s) so that others can interpret these bits universally.

This process, called *Encoding*, has multiple forms for Unicode, including UTF-8. An item on the Unicode list can be encoded into UTF-8 into up to 4 bytes, so what that last bullet point tries to tell us is: a byte in a `str` is one byte that might belong to at most 3 others to map to an entry on the Unicode list.

UTF-8 has one interesting property: it fully includes ASCII. This means that any character on the ASCII table has the same byte representation both when encoded with ASCII and with UTF-8. That is why we were able to focus on ASCII in the earlier chapters, despite Rust using UTF-8 under the hood.

By the way, if at this point you feel that something doesn’t *quite* fit together here with how texts are supposed to work - you might be onto something. We will return to this shortly.

As you might imagine, this kind of data structure is very efficient. It’s also very difficult to work with if you want to modify it. Essentially, you would be forced to recreate a `str` every time you’d want to extend it.

That’s where a `String` comes in. A `String` is a proper `struct` with functions implemented on it. Under the hood, it maintains a growable `struct` (which we will meet personally later this chapter) to contain the content, and as such, modifying a `String` is easy. Essentially, it does all the heavy lifting around `str` s, that I hinted at above, for us.

And the most awesome part? `String` implements a specific trait (`Deref<target=str>`, to be precise), and this trait makes sure that you can turn your `String` into a `&str` *just by dereferencing it* (which we usually do with the ampersand). This means we don’t have to worry about whether to accept a `&str` or rather a `&String` or how to convert one to the other.  
That is awesome, as it implies that we can keep working with `&str` all the time and only specifically require a function to pass a `String` when necessary.

Let’s see this in action.

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

[Link to Rust Playground](https://play.rust-lang.org/?version=stable&mode=debug&edition=2021&gist=6f5382263c70502f924fbfc6c117bb84)

## Assignment 5: A Poor Line Viewer

Let’s extend `hecto` a bit and provide it with some very basic Line Rendering capabilities.

Here is the assignment:

- Introduce a new `struct` called `View`
- That view exposes a method called `render()`
- Move the logic of `draw_rows` into `render()`. Move the necessary helper functions from `Editor` to `View`.
- Adapt the logic so that the first line reads `Hello, World!`(There is no need to hide the package name and version number in the middle of the screen just yet)
- This method should be called instead of `draw_rows`.
- Using the knowledge about `String` s and `str` s, change the signature of `Terminal::print` to no longer depend on traits.

**Code Review:** [Find my code here.](https://github.com/pflenker/hecto-tutorial/commit/34f8fed4e34ef99a63ad85165a832a438c88d684)

## Introducing a Buffer

Here’s what we’re going to build up over the course of this chapter:

- `Editor`: This component primarily mediates between different UI components, transferring focus and information among them. For now, we only have the full-sized `View`.
- `View`: This component receives every text-related event from the `Editor`, such as keypresses for characters, newlines, and so on. The `View` uses this information to enhance its rendering efficiency and passes modifying events to the `Buffer`.
- `Buffer`: This component will hold our text. We'll start with a simple representation as plain text and gradually extend it throughout this tutorial to accommodate more data as needed.

A `Buffer` is a common structure that holds everything a text editor needs to modify and display a text file. A View `interacts` with the `Buffer` to render it on the screen. In many text editors, you can easily switch from one `Buffer` to the next, allowing you to open multiple files in parallel.

The implementation details of a `Buffer` vary by text editor. For example, [Nano](https://www.nano-editor.org/) utilizes a straightforward `Buffer` structure with no other internal representation of the document. Upon saving a file, the contents of the `Buffer` (excluding data such as color for syntax highlighting) are saved to disk. On the other hand, [Vim](https://www.vim.org/) features a sophisticated internal structure to efficiently handle operations, even on large files, and includes a separate `Screen Buffer` that represents what is currently visible on the screen. These design choices reflect the visions of each editor: `Nano` is designed to be a small, lightweight editor (hence the name!), suited for quick updates to configuration files. `Vim`, meanwhile, is designed as a full, feature-rich text editor, ideal for working with lengthy files.

Before we can go on implementing our `Buffer`, we need to meet another useful Rust data structure.

## “What’s the Vector, Victor?”

Our Buffer will need to dynamically resize—grow when items are added and shrink when items are taken away—so we’re in need of a special kind of data structure. This is exactly what a String in Rust manages internally, using a structure known as a [Vector](https://doc.rust-lang.org/std/vec/struct.Vec.html), or `Vec`. Inside a `String`, there's a `Vec<u8>` —a vector of bytes, essentially. As we’ve seen in Chapter 3, `u8` stands for unsigned 8-bit numbers, which are essentially bytes. So, when you’re manipulating a String, you're actually working with a vector of bytes.

This `Vec` not only stores bytes but also ensures that any data it holds can be returned as a `&str`, which is a reference to a sequence of UTF-8 encoded Unicode characters. `Vec` s are designed to manage memory efficiently. For example: If you initialize a `Vec` and continually add elements in a loop, a simple approach might involve allocating new memory for each addition, transferring all existing items to this new space each time. This would be inefficient, s

What we need is a data structure which can grow and shrink as you add and remove items from it. A `String` uses such a data structure under the hood to allow modifying the text it holds. This data structure is called [a Vector](https://doc.rust-lang.org/std/vec/struct.Vec.html), `Vec`. A `String` maintains a `Vec` of `u8` internally, which is a concise way of saying: It uses the flexible data structure `Vec` to hold `u8`, so unsigned 8-bit numbers. 8 bit? That’s a byte. So basically, it’s a vector of bytes, and the `String` data structure ensures that whichever arbitrary data points this `Vec` saves, at the end of the day you’ll get a `&str` when you need it, which *also* points to bytes in memory, but these are guaranteed to part of UTF-8 encoded Unicode points.[^2]

`Vec` s internally try to ensure to be memory-efficient. Consider the scenario where you create a new `Vec` and keep adding to it in a loop. In a simple implementation, the `Vec` would allocate some memory for all the old items plus the new one and copy the old items over, but internally, `Vec` s ensure that this copying of data doesn’t need to happen frequently.

## Understanding Memory Management in Programming

Memory management can seem daunting if you're transitioning from languages that abstract away these details. Let's simplify it.

A computer's smallest memory unit is a bit, represented as either 0 or 1. For instance, as we’ve seen before, the ASCII character `a` is `01100001`. To represent two as (`aa`), they are placed sequentially: `0110000101100001`. Knowing the size and starting point of each character is crucial, as bits for different characters can be adjacent.

This is where pointers come in. They act like a map, indicating the starting point and size of data, making data retrieval efficient. Consider the `aa` example; a pointer needs only the start location and the size of one character to locate each `a`.

Adding more characters, like `aaa`, isn't straightforward. If memory behind the last `a` is in use, you'd need more space. This requires requesting additional memory from the operating system to accommodate the new characters, then copying over the previous `a` s, and then adding the next one - a slower process than simply using available memory.  
Rust addresses common memory management challenges at the compiler level:

- It prevents writing beyond allocated memory.
- It manages pointers carefully to avoid errors like modifying data through one pointer and accessing outdated data with another.
- Rust's ownership model, including Move Semantics, tracks and manages pointers effectively, reducing errors from outdated or conflicting data accesses.

⠀In contrast, languages like C require manual memory management, whereas Objective-C uses reference counting (`retain` and `release`) to manage memory. Java and JavaScript employ garbage collectors that automatically clean up unreferenced objects.

JavaScript, for example, uses arrays of pointers for strings, masking the complexity from developers. Although it appears as an array of strings, it's technically an array of pointers to strings, similar to Rust's `Vec<String> ` but without exposing the underlying pointers.

Continuing our discussion of `Vec` s, let’s consider how Rust’s `Vec` handles data:  
Adding elements to a `Vec` could seem inefficient if it copied existing elements to new memory locations every time. Instead, `Vec` optimizes this process by initially allocating more memory than currently required. This extra space allows the `Vec` to store additional elements without needing to copy the entire array each time. Only when this capacity is fully utilized does it allocate more space and copy the elements. Therefore, the number of items in a `Vec` doesn’t directly indicate the amount of memory it uses; the allocated memory usually exceeds the immediate needs to enhance performance. This strategy balances memory usage and processing efficiency.

## Using Vecs in a Buffer

Since we plan to handle an arbitrary amount of `Line` s per file, the `Vec` is an excellent data structure for us to use. To be able to properly use it, we need to understand one additional small conceptual and syntactical thing:  
As discussed above a `Vec` cannot hold arbitrary data. So we cannot talk about a `Vec` without talking about what it holds. We denote that by adding the type in pointy brackets. The data structure that the `String` is using internally is therefore a `Vec<u8>`.

Let’s play around with a `Vec` now:

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

[Link to Rust Playground](https://play.rust-lang.org/?version=stable&mode=debug&edition=2021&gist=29a07bd5c0844be3516c2e5d2c241dab)

## Assignment 6: A Simple Buffer

Let’s introduce a simple buffer. We’ll create this as its own `struct`, `Buffer`, which will for now only hold a `Vec<String>` (Vector of Strings). For now, you should manually add a string with `Hello, World!` to the buffer using `String::from(“Hello, World!”);`.

Before I present you with the assignment, let me quickly show you how to access elements in a `Vec`:

```rust
fn main() {
    let mut vec: Vec<String> = Vec::new();
    vec.pushfrom("Hello, World!");
    vec.pushfrom("Hello, hecto!");
    println!("{}",vec[0]); //Variant 1
    if let Some(element) = vec.get(1) {
        println!("{}", element); //Variant 2
    }
}
```

[Link to Rust Playground](https://play.rust-lang.org/?version=stable&mode=debug&edition=2021&gist=77c0be69c32fcf16685accf57e6ce5ce)  
We will discuss Variant 2, including `Some`, after this assignment.

Here is now the assignment:

- Create a new `struct` called `Buffer`, which holds a `Vec<String>`.
- Ensure that for this step, the `Buffer` holds one `String` with `Hello, World!`.
- Extend the `View` to hold a Buffer.
- With this change in `View`, you need to make `View` part of `Editor` and change `render` from an associated function to a method (i.e. to `render(&self)`)
- This, in turn, might require you to implement or derive `Default` for `View` and `Buffer`.
- Change the logic in `render`: for each entry in the buffer, render that entry. For every empty row on screen, still render a `~`. (You can still leave the welcome message untouched, we’ll handle this properly soon)

## Assignment 6: Code Review

[Here is my code on GitHub.](https://github.com/pflenker/hecto-tutorial/commit/83c71e67f06c7b6113d5560eaad1d2afe87cca8f)

Let’s discuss the two different approaches to access a `Vec` now. Variant 1 in the code sample above only works if the index you’re passing is actually valid. Otherwise, program execution will stop: the program will panic.  
The second variant ensures that even if your index is invalid, the program would not stop. Let’s take a closer look at what this means technically.

We have the need to represent, in code, that a function *might* return something - in this case the entry of a `Vec` - or not. We already met a concept that sounds like a good candidate to use: a `Result`, which returns you a box saying `Ok` if something is there (putting this *something* into the box), or a box saying `Err` if an error occurred, putting the error inside.

However, the semantics here are different: it says “There should be something here, and the fact that there is not is an error”, while what we want to express is “It’s normal that nothing might be there”. This is what an [Option](https://doc.rust-lang.org/std/option/) represents. It works almost exactly the same as a `Result`. To stay with the metaphor of Boxes: Retrieving an entry from a `Vec` returns you either a box labeled `Some`, with the entry in it, or a box (a slip of paper, really) saying `None`, meaning that there was nothing in the `Vec` at that position.

If you want to explore more - here is an example, this time taken straight from the docs:

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
    
    // The return value of the function is an option
    let result = divide(2.0, 3.0);
    
    // Pattern match to retrieve the value
    match result {
        // The division was valid
        Some(x) => println!("Result: {x}"),
        // The division was invalid
        None    => println!("Cannot divide by 0"),
    }
}
```

[Link to Rust Playground](https://play.rust-lang.org/?version=stable&mode=debug&edition=2021&gist=26c9366c897c1f9dd0b2fbc2c2a73195)

## Assignment 7: A Basic Line Viewer

It’s time to read an actual file and load its contents into our buffer. To do so, we allow `hecto` to be started with a command line argument which will hold the file name for us - `./hecto Cargo.toml`, for example, would open the `Cargo.toml` for us.

Here is a playground to get the filename from the arguments:

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

[Link to Rust Playground](https://play.rust-lang.org/?version=stable&mode=debug&edition=2021&gist=75c2dbd7ef5bd0bb7e1154a18ebc6b31)

`collect()` turns the output of `args()` into the `Vec` we’re after. We’ll discuss the details around `collect()` after this assignment.

Reading from the file system is done with a function called `std::fs::read_to_string`, which helpfully [opens a file and reads it to a string](https://doc.rust-lang.org/std/fs/fn.read_to_string.html).  
`let file_contents = std::fs::read_to_string(filename)?;` reads the file at `filename`.  
If it succeeds, it’s guaranteed that `file_contents` contains the file’s contents as a valid UTF-8 string. You can conveniently iterate over the individual lines in this String (or any String, for that matter) with the following code:

```rust
for line in file_contents.lines() {
     // Do something with the line
}
```

That should hopefully help you with the next task.

Here’s the assignment:

- If `hecto` is called without any command-line arguments, it should display the Welcome Screen from before: The left column filled with `~`, the package name and version centered on screen.
- `Editor` should check for a filename and pass it to `View`, for example to a method called `load`. This method should open the file and load its contents into the `Buffer`. (Remove the hardcoded “Hello, World!”).
- Your `Buffer` should expose a method called `is_empty`, which returns true if the Buffer is empty and false otherwise.
- If a `Buffer` is empty, the welcome message should be displayed, otherwise it should be hidden.
- Empty lines should still be rendered as a `~`.

## Assignment 7: Code Review

[Here’s how I solved this.](https://github.com/pflenker/hecto-tutorial/commit/6e4164e07d760fef2357d4b4f6d004b6c299e536)

Let’s take a closer look at how we generated the `Vec<String>` that holds the arguments. We did so by calling `collect()` on what `env::args()` gave us.

*Iterators* have been with us since the beginning of the tutorial, it’s time to meet them properly. As usual, I will try to find a non-technical example to illustrate it before explaining it technically in more detail.

I have two kids, and they can’t get enough sweets, ever. For all they care, they would just eat one, ask for more, and then eat another one. Let’s say I went shopping and come back with a huge bag full of groceries, some of them sweets. Every time they yell “Next!” I grab into the back and pull out some tasty treat, until I run out of things and then I return nothing. I am an Iterator then, and I am iterating over the sweets in the bag.

What a `for treat in dads_bag` loop does is that it calls `next()` on the Iterator `dads_bag` and passes the result as `treat` to the block that follows. If `next()` returns `None`, then the `for..in ` loop ends.

It’s important to understand that iterators are *lazy*, meaning that they generally don’t know if there is something in that bag until you call `next` - they compute their items as they are needed and not all at once. This can come in handy if whatever you’re pulling out of that bag is difficult to manage (angry stray cats for example, or maybe the metaphor just breaks down here) and you don’t need to know in advance how many things actually are in that bag.

Iterators have a few convenience methods, and `.collect()` is one of them. It depletes the iterator and stores all the results in a `Vec`, and is the technical equivalent of my kids yelling `next` at me rapidly until I hand over all the treats and they can manage them themselves.

A `Vec` is also an Iterator, it implements the corresponding trait. Here is a playground for you to understand Iterators a bit better:

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

[Link to Rust Playground](https://play.rust-lang.org/?version=stable&mode=debug&edition=2021&gist=1eb6d8b6d6d6e297648434aec58e96c4)

## Assignment 8: Better Rendering

We’re now able to show the lines of the document, but any line that is wider than the terminal will potentially wrap.  
We’re currently relying on printing out `\r\n` to move the caret to the next line, and continue rendering there. We should move the caret to the beginning of the next line instead.  
Besides this, we currently re-render the entire view on every pass through the `repl` loop - even though currently nothing actually changes in the View, it’s only the caret that is moved around.

We’re going to fix all this in this assignment:

- Update the rendering strategy to move the caret appropriately, thus getting rid printing out `\r\n` directly.
- Ensure to only render substrings of the current line. You can use a slice (i.e. `current_row[0..width]`, or you can leverage the fact that `get` accepts a range (i.e. `current_row.get(0..width)` - or you use String’s `truncate` function, which we’ve used to construct the `welcome_message` before.
- Introduce a flag `needs_redraw` on `View`. If `true`, the `View` needs to be re-drawn. If false, no rendering is needed. Set this flag in the appropriate places to ensure rendering is only performed when needed.
- Bonus Assignment: To make this work even when the Terminal is resized, you need to react to [the Resize event](https://docs.rs/crossterm/latest/crossterm/event/enum.Event.html) and ensure the `View` reacts appropriately.

**Code Review:** [Find my code here.](https://github.com/pflenker/hecto-tutorial/commit/0f2c1a781e0ce8389a2a36dd05a79e1a2fff4a9c)

## Error Handling

We’re about to enter territory where coding errors will panic `hecto` from time to time. This currently has an annoying side effect. To simulate it, just place `panic!` anywhere in your code. `panic!` is a macro which helpfully crashes your program, which is the exact same thing that might happen if you do something Rust doesn’t expect, like slicing a part of a `str` beyond its boundaries.

Annoyingly, we’re still in Raw Mode when this happens, meaning our Terminal will be all messed up. Time to give Error Handling, disabling Raw Mode, and the whole lifecycle of structs in Rust a closer look. Let’s start with learning what “panicking” actually means.

## Understanding Rust Program Execution and Panicking

When you run a Rust program, it all starts at the `main` function, where each subsequent function call adds a new entry to the *call stack*. This stack is essentially a record of what functions are active and their current states - you can imagine it as an actual stack of paper, where each sheet describes the content of each function. Here’s a simple setup:

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

In this example, `main` calls `function_a`, which then calls `function_b`, leading to `function_c` that outputs a message. Each function call creates a new *frame* on the call stack, and as each function completes, its *frame* is removed, working back to `main`. In other words: While we move on from `main()` to `function_c()`, the stack grows by first a piece of paper describing the context of `main()`, then `function_a()`, then `function_b()`, then `function_c()`, then whatever function is generated by `println!`, and after the text is printed out, each piece of paper is removed from the stack, one by one, until we’re back in `main()` and then end the program.

### Handling Errors with panic!

Rust uses `panic!` for errors that it doesn't know how to handle. The default action is to start *unwinding the stack,* which means cleaning up as it goes back up the chain (or through the stack of papers). Part of the cleanup is printing out information about the call stack (which appears to us as a *stack trace*) and disposing of variables and structs.  
Alternatively, you can configure Rust to abort execution immediately when a panic occurs, which can make your program smaller since it skips the unwinding. You set this up in your `Cargo.toml` with `panic = 'abort'`. This method is faster but be wary—it can leave some resources like open files or network connections hanging.

### Customizing What Happens on a Panic

Rust gives you options to handle panics your way:

- **Catching Unwinds:** You can catch a panic in the act of unwinding, allowing your program to potentially continue running. This is handled using `std::panic::catch_unwind`.
- **Creating Custom Panic Handlers:** If you're designing software where you can't just hand back control to the operating system (like in embedded systems), you can define exactly what happens in a panic situation.
	Here's a peek at what a custom panic handler might look like:
	```rust
	#[panic_handler]
	fn panic(info: &PanicInfo) -> ! {
	    loop {}
	}
	```
	This code says that once a panic happens, the program should just loop indefinitely—basically freezing up, indicated by `->` (meaning: this function must not ever return)
- **Panic Hooks:** Before the panic unfolds, you can have functions that kick in to tidy up, setting up a more controlled crash or cleanup.

## When to Panic

As we’ve discussed before, handling errors locally or propagating them cleanly is usually preferred over causing your program to `panic!`. However, an exception to this rule applies in the context of defensive programming.

Defensive programming means to anticipate and protect against unexpected conditions. For instance, our current `Buffer` implementation assumes its `String` s never contain a newline. This assumption holds for now because of our specific implementation, but future modifications (maybe by ourselves) could introduce bugs resulting in undefined behavior.

To safeguard against this, we should implement checks which panic in Debug builds—but not in Release builds. This is advisable to avoid leaking function internals into function signatures and performing unnecessary checks in the final version.

Here’s an example to experiment with:

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

[Link to Rust Playground](https://play.rust-lang.org/?version=stable&mode=debug&edition=2021&gist=7469cd789a9d25760ce67f10b40c6639)

The key takeaway here is that code within `#[cfg(debug_assertions)]` or used in `debug_assert!` is entirely removed from the final release build at compile time, optimizing performance.

## Struct Lifecycle

Let’s zoom in a bit on the lifecycle mechanisms in Rust. Unsurprisingly, the stages in the lifetime cycle are: creation, usage and destruction.

### Creation

Creating variables and structs in rust is pretty straightforward. For structs, we’ve already used default() extensively. Default is meant to provide a simple, zero-argument constructor returning a default type without any configuration.

More extensive setup logic, like starting network connections or putting the terminal in Raw Mode, should not go into default. The idiomatic way to do so is usually to provide a method called new, which could expect arguments if needed, and which then performs the setup we need in order to fully use our struct.

### Usage

This section will cover an important rust concept, which we will barely use in this tutorial. It’s safe to skip if you’re just here for the parts relevant to building hecto.

Our code so far only uses easy usage patterns: We create what we need, and once our function completes, Rust disposes of any leftover variables. One of the reasons why this is so easy for us is that we’re not working extensively with references yet - we mostly copy around what we need, which is perfectly acceptable in our case. Things get more complicated when references are involved. As we’ve learned previously, Rust ensures *Memory Safety*, meaning that the references we work with are always valid. It also ensures that no Memory Leaks happen, which means that all the data allocated for us in memory are actually needed, and that there is nothing allocated which would never be used by any part of the program any more.

Part of how Rust ensures this is that it ensures during compiling that the lifetimes of each variable we’re working with. Let’s examine an example where this does not work:

```rust
fn longest(x: &str, y: &str) -> &str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}
```

This code does not compile. The reason why it doesn’t is that Rust can’t determine at compile time how long a `&str` returned from `longest` will live, and therefore `longest()` can not reliably output a `&str` with a predictable lifetime. One way to solve this is to explicitly define the lifetimes of `x` and `y`. Here is how we enforce that `x` and `y` have the same lifetime:

```rust
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}
```

The `’a` here is a generic lifetime specifier, and this context can be understood to accept only two `&str` s with the same lifetime, and returns another `&str` with that exact same lifetime. So in this case, we put the burden of ensuring the correct lifetime on the caller.

There is also the lifetime specifier `'static`, denoting that this data lives for the entire duration of the program, and there’s `'_` which is an anonymous lifetime - it doesn’t need to be named because it can be inferred by the compiler.

### Destruction

Removing things from memory is done by Rust once the end of its lifetime is reached. However, we sometimes have the need to do some cleanup if a `struct` of ours goes out of memory - for instance, disabling raw mode again. This is where the `Drop` trait comes in: If it’s implemented, its `drop()` method will be called once the struct goes out of memory.

This comes with an important addition or pitfall: `Drop` is *also* called during panicking, which means that even if the program crashes, the cleanup activities are performed. However, the code in `drop()` can *also* panic, and if it does, you’ll face a 'double panic', which forces Rust to stop everything and abort. Thus, ensure your cleanup code is bulletproof to avoid these scenarios.

## Alternate Screens

While we’re reasoning about lifecycles in `hecto`, let's also revisit how we set up the terminal. We can further refine `hecto` by using the terminal’s "Alternate Screen" feature, an ancient functionality that saves the current screen buffer and switches to a new one at our disposal. Upon exiting the alternate screen, the original screen is restored. This can be managed with `EnterAlternateScreen` and `LeaveAlternateScreen` from [crossterm](https://docs.rs/crossterm/latest/crossterm/terminal/struct.EnterAlternateScreen.html#). One major benefit for us, as developers, is that `clippy` warnings are preserved and not wiped off the screen; they remain available upon exiting `hecto`.

The terminal provides two buffers: the main and the alternate screen. This setup offers a clean slate for our application, while ensuring that once we exit the alternate screen and end the program, the user's original environment is untouched and ready for further use.

This functionality also enables us to implement a logging feature without cluttering our UI. We can temporarily leave the alternate screen to output necessary logs to the main screen, then return to the alternate screen without needing to re-render the user interface, as it retains its previous state.This would not be a good idea for high-frequency logging (which should go into a file, anyway), since this would create a noticeable flicker effect.

## Assignment 9: Improved Lifecycle Management and Error Handling

This assignment is going to put everything together what we’ve just learned. As part of the assignment we will need to implement a custom Panic Hook, since the process of panicking will already start writing out before our cleanup logic in the implementation of `Drop` would be called.  
The following code does this:

```rust
let current_hook = std::panic::take_hook();
std::panic::set_hook(Box::new(move |panic_info| {
    //do the cleanup work, e.g. Terminal::terminate()
    current_hook(panic_info);
}));
```

We will dive deeper into what this code is actually doing during the code review.

Here’s the assignment:

- Revisit the lifecycle management in your code:
	- Ensure to implement `Default` only for side-effect free defaults and to implement a `new()` function for more complex setup work.
		- Ensure to implement `Drop` for necessary cleanup work after a struct goes out of memory. Make sure it doesn’t panic.
- Revisit Error Handling:
	- Look out for errors which you are currently passing up, but which you can, in fact, handle locally.
		- Look out for errors which should `panic!` on a Debug build, but not on Release builds.
		- Look out for implicit contracts or assumptions in your code. Can you explicitly check them on Debug builds and panic?
- Use the Alternate Screen:
	- Switch to the Alternate Screen when `hecto` starts and cleanly leave it when it ends.
		- Implement a custom Panic Hook to properly leave the alternate screen and raw mode even upon panicking.

## Assignment 9: Code Review

[Here is how I solved it.](https://github.com/pflenker/hecto-tutorial/commit/f04927719386af56d278afc02e074ab1c20df22e?diff=split&w=0)

Now, let’s take a closer look at the panic hook and the code snippet from above and learn a bit about closures:

We’ve talked about different types before - `usize`, `struct` and all the good stuff - but sometimes we have the need to pass around *functions* as parameters. In our case, we need to provide `set_hook` with a function that should be called in case a `panic!` occurs. One way to define anonymous (i.e. unnamed) functions are closures (The syntax to do this, by the way, [is taken from Ruby](https://www.sitepoint.com/closures-ruby/)).

Here’s a playground to play around with it:

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

[Link to Rust Playground](https://play.rust-lang.org/?version=stable&mode=debug&edition=2021&gist=be2c5802fee314941a2d0d3ee67b74fc)

I think after some playing around with it, you’ll get a good grasp of the similarities between a closure and a function.

Okay, so the code above defines a function which is passed to `set_hook`. But apparently we can access `current_hook`, which is defined outside of the closure, from within it. Let’s look at that a bit closer - here’s a simpler example to illustrate what’s going on:

```rust
fn main() {
    let message = String::from("Hello");
    let greet = move |name| println!("{}, {}!", message, name); 
    greet("Philipp"); // that's my name, in case you forgot 
}
```

[Link to Rust Playground](https://play.rust-lang.org/?version=stable&mode=debug&edition=2021&gist=65060827f19d5dbf0e83fa8cfa1b9fa7)

As you can see, closures can access their surrounding variables. That’s neat, but at times also a bit difficult to reconcile with how Rust keeps track of references. If you use something that implements the `Copy` trait, the closure will have a copy available to work with. If not, you can use the key word `move` to make use of the Move Semantics we discussed in Chapter 3. In the example above, `message` is inaccessible for `main` after `greet` has been defined.

So what happened in the code above is that we first retrieved the current panic hook with `take_hook`, and we used `move` to pass it to the closure, where we could finally execute it after our cleanup. Why would we not use `take_hook` then and there, instead of using the `move` magic? Think about it for a bit, because the solution is crucial to understand closures. Find the solution in the footnotes.[^3]

Last but not least, there’s this weird `Box::new` thing in there. What does it do?

This is related to Lifetimes, which we discussed above. In this case, we have a need to ensure someone *else* is getting a reference that stays valid for as long as *they* need it even after the end of the life of our struct.

Let's break down why using a Box is crucial when setting a panic hook. If we didn't, we'd define our closure and pass it to `set_hook`. However, at some point, our code might lose all references to this closure, causing it to be deallocated. This is problematic because panic hooks need to persist for the entire lifetime of the program – potentially far longer than our specific `Editor` object.

To ensure the closure's survival, we need to transfer its ownership to `set_hook`. We achieve this by putting it inside a [Box](https://doc.rust-lang.org/std/boxed/struct.Box.html). This "boxes" the closure in memory, and we pass a pointer to this boxed closure to `set_hook`. Now, the panic hook function owns the data within the box, ensuring the closure persists even if our original code is gone.

This, by the way, reinforces our design decision to make `Editor` the one data structure that is there from the beginning to the end of the program

Here’s now an annotated version of the same code above:

```rust
//Retrieve the current hook, which by default does some nice printing of the panic
let current_hook = std::panic::take_hook(); 
// Define a new closure that takes a reference to the PanicInfo.
// Move any external variables needed within the closure here. 
// Place the closure into a Box and set it as the new panic hook.
std::panic::set_hook(Box::new(move |panic_info| {
    // Our custom panic hook logic goes here
    // Execute the original hook to retain default panic output behavior.
    current_hook(panic_info);
}));
```

## Assignment 10: Scrolling

Let’s implement scrolling now. In this step, we will not respect document boundaries yet, instead we allow potentially endless scrolling.

Our strategy will be to keep track of the location within the text in the view, as well as the top-left location in the text which is currently visible at the top-left corner of the terminal. Every time the location in the text goes out or the area currently visible on screen, we scroll by updating the `scroll_offset` to bring it back into view. We convert the location in text in the position on screen by subtracting the offset dimensions from the location in the text.

Here’s the assignment:

- Handling User Input:
	- Start redirecting necessary key presses to `View`
		- Move tracking the location away from `Editor` to `View` or `Buffer`.
		- Lift the current restrictions around the Terminal boundaries and allow movement further to the bottom and to the left.
- Implement scrolling:
	- Introduce a `scroll_offset` which keeps track of the top-left position in the text which is visible on the screen.
		- Whenever the cursor is about to go off-screen, update `scroll_offset`, for example:
		- If it goes off to the right, increment `scroll_offset.x` by the appropriate amount of steps
				- If it goes off to the left, set `scroll_offset.x` to the `x` of the new position
				- If it goes off to the bottom, increment `scroll_offset.y` by the appropriate amount of steps
				- If it goes off to the top, set `scroll_offset.y` to the `y` of the new position.
		- Call this functionality every time the cursor might go off-screen, including resizes.
		- Update the rendering mechanism to handle rendering parts of the row for horizontal scrolling, and a different subset of lines for vertical scrolling, based on `scroll_offset`.
		- Expose the position on screen from `View` to `Editor` to correctly place the caret on screen. You can subtract `scroll_offset` from the position in the text to determine the position on screen.

## Assignment 10: Code Review

[Here is how I solved it.](https://github.com/pflenker/hecto-tutorial/commit/6cbc0f000ee5c457de9cca5a6b973b9d2c58b22b?diff=split&w=0) Let’s discuss three core parts of my code, two are Rust-related, and one is design-related:

- The Command dispatch structure I introduced
- The From and TryFrom traits
- The Range parameter

Starting with the first one: Instead of continuing to react on `crossterm` ’s events directly, I decided to introduce an internal Command system, where the `Editor` dispatches commands based on the `crossterm` events. This has two advantages:  
The first is that the code will be easier to reason about as it grows. We anticipate more commands around inserting, deleting and all the other text-related things we will need to do, and with the new structure it’ll be easier to reason about the changes going forward.  
The second one is that we’ve now paved the way for multiple possible improvements in the future. The most immediate one would be an Undo/Redo mechanism, which could be implemented as a stack of Commands, where each new Command (say, Insert(Char)) produces its opposite Command on the stack (Say, Delete). Undo would then simply mean popping the last command from stack and executing it.

### From and TryFrom

I’ve implemented two traits in my code: [From](https://doc.rust-lang.org/std/convert/trait.From.html) and [TryFrom](https://doc.rust-lang.org/std/convert/trait.TryFrom.html). The main difference between the two is that TryFrom accounts for cases where a conversion to the target type might fail. These traits have two interesting advantages:  
The first is, that by implementing e.g. from(), you get the corresponding into() functionality for free. In other words: If you implement `from` for types `A` and `B`, you can both call `B::from(a)` and `a.into()` to turn `a` into an instance of `B`.  
The second one is that Rust can use the trait to do implicit conversions for us. This is best understood in code, so here is a Rust Playground illustrating the concept:

```rust
#[derive(Debug)]
struct ErrorA;
#[derive(Debug)]
struct ErrorB;

impl From<ErrorB> for ErrorA { //We allow converting ErrorB into ErrorA here
    fn from(_: ErrorB) -> ErrorA {
        ErrorA{} // We would typically carry over some information from ErrorB into ErrorA. We don't do this here because we want to illustrate something else.
    }
}

fn do_something_else() -> Result<(), ErrorB> {
    Ok(())
}

fn do_something() -> Result<(), ErrorA> {
    do_something_else()?; // here is where the implicit conversion happens: Even though this function returns an ErrorB upon failure, Rust will use the from implementation to convert it into ErrorA and propagate it up.
    Ok(())
}

fn main() {
    do_something().unwrap();
}
```

[Link to Rust Playground](https://play.rust-lang.org/?version=stable&mode=debug&edition=2021&gist=ec28acb0c9870e6a8617a8f46f3af427)

### Ranges

Let’s talk a bit more about ranges. You saw me using the parameter `Range<usize> ` in my code, which allowed me to take something like `0..10` as a parameter.  
Here’s a list of Ranges which Rust provides:

- `Range` (`start..end`) - Represents a range that goes from start to just before end.
- `RangeInclusive` (`start..=end`) - This one includes the end value in the range.
- `RangeFrom` (`start..`) - Represents a range starting from start and continuing indefinitely.
- `RangeTo` (`..end`) - Represents a range from the beginning up to, but not including, end.
- `RangeToInclusive` (`..=end`) - Goes from the beginning up to and including end.
- `RangeFull` (`..`) - Represents the whole range possible for the type, essentially from start to end.

## Assignment 11: Snap Cursor to End of Line

We have now extended our code to distinguish clearly between the position of the caret on screen, and the location of the insertion point in the text.  
Now let’s ensure that this `location` only ever points to valid positions in the text, with the exception that we allow the point to be placed behind the last character at the end of line, and one row below the end of document, so that the user can add new characters easily.

Before we dive in, one word of caution: This assignment will only work for ASCII text. Should you encounter any issues with non-ASCII text, rest assured that we’ll address this in the assignment after this one.

We will break this assignment down into simple horizontal and vertical movement followed by more complex movements.

Here is the assignment:

- Horizontal Movement:
	- For simple arrow presses, ensure you can’t move past the end of the last character at the end of the line, or the beginning of the line.
		- `Home` should bring you to the start of the line, `End` one character past the end of the line.
- Vertical Movement:
	- For simple arrow presses, ensure that you can’t move past one line below the document
		- For `PgUp` and `PgDown`, move the point up or down by the height of the view, but at most to either the top or one line below the end of the document.
		- If you move from a long row to a shorter one, ensure that the point is placed correctly vertically by snapping it to the end of the shorter line.
- Complex movement:
	- Pressing left at the beginning of a line should move the point to the end of the line above.
		- Pressing left at the start of the document should do nothing.
		- Pressing right at the end of a line should move the point to the beginning of the line below.
		- Pressing right at the end of the last row should move the point to the beginning of one line below the document. From there, you should only be able to move back up, but not further down.

**Code Review**: [This is how I solved it.](https://github.com/pflenker/hecto-tutorial/commit/cb50da980a2287821f4a6e959c82acf895a64a1f?diff=unified&w=0)

## Introduction to Graphemes

As explained during the last assignment, the code above does not work for non-ASCII characters.  
Just take the following text, try to open it in `hecto` and see how scrolling and snapping the cursor to the end of the line works:

```
hecto is great!
hecto is 𝒻𝒶𝓃𝒸𝓎!
hecto is ѕ¢αяу!
ʎzɐɹɔ sı oʇɔǝɥ
```

To understand more what’s going on, I have prepared a Rust Playground for you:

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

[Link to Rust Playground](https://play.rust-lang.org/?version=stable&mode=debug&edition=2021&gist=22b3dec61998a23f86040c756a4d1d07)

What we can observe is that our intuitive understanding about what a character should be does not match what Rust thinks is the length of a string. By our intuitive understanding, all the strings above should have the same length: 14, one for each visible character. Instead, the length is different for all these strings!

Part of the reason is how UTF-8 works, as we have already discussed earlier in this chapter. To recap, UTF-8 encoded characters do not have a fixed width: They can take *up to 4 bytes* to store. Any ASCII character fits into 1 byte, anything non-ASCII might not. And the `len()` function on a string returns the numbers of bytes in it, not the number of visible characters - which does not match our intuitive understanding.

We’ve already briefly discussed that Chars always take up 4 bytes in Rust - this is the reason why. Rust uses UTF-32 internally for characters, which is another encoding like UTF-8, with the difference that it’s fixed-width, and can therefore always reliably hold a UTF-8 encoded character.

The lowercase a, which we investigated in an earlier chapter, is represented in ASCII the same as UTF-8 as `01100001`. In UTF-32, it would be represented as: `00000000 00000000 00000000 01100001`.

So why don’t we just use UTF-32 to encode strings? This would enable us to work on characters in chunks of 4 bytes at a time, right? Well, for one thing this would mean that normal ASCII texts would be unnecessarily bloated in memory. However, the reality is even more complicated.

Let’s take another look at a Rust Playground to understand things a bit better:

```rust
fn main() {
    let love = "We all love hecto 🧑🏿‍🤝‍🧑🏿";
    dbg!(love.len());
}
```

[Link to Rust Playground, which also suffers the issue we’re investigating here](https://play.rust-lang.org/?version=stable&mode=debug&edition=2021&gist=2f0222abfa41a9fc8ec9ba120bc6d95e)

We would expect this string to be at most 22 bytes long: 18 for the ASCII text, at most 4 for the emoji, based on our assumption that an emoji is a Character, and Characters take up 4 bytes.  
But the reality is that this string is 44 bytes long - the emoji alone takes 26 bytes!

So what is stored in the emoji?

Let me show you:

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

[Link to Rust Playground](https://play.rust-lang.org/?version=stable&mode=debug&edition=2021&gist=d5717672e32dfbfb7059b040fe088051) - Example taken and adapted from [here](https://stackoverflow.com/questions/71163103/i-need-to-know-what-are-the-more-than-four-bytes-character-in-utf-16-and-what-is).

`\u{200d}` tells Rust to use Unicode Point U+200D here, which is the [Zero Width Joiner](https://unicode-explorer.com/c/200D). Its purpose should become obvious as you investigate the example above.

Emojis are a great way to illustrate what is at play here: What we see as one character on the screen is technically something much more complex. This does not only apply to Emoji: When working with Arabic or Indic scripts we would face the same issue.

Let’s sort out some vocabulary now. One distinct unit that we want to display on the screen is what we call a *Grapheme*. And as we have learned by now, we cannot easily derive the number of *graphemes* in a sequence of bytes which make up a *string*. And a *Grapheme* and a *Character* are not necessarily the same - Unicode is fuzzy with its definition of a Character, and Rust’s Character datatype is for sure not a Grapheme - it’s a Unicode Scalar Value, basically one Unicode point, such as the Zero Width Joiner. There is no single Character in Rust representing the emoji above. Multiple Characters can make up a *Grapheme Cluster*, such as the 🧑🏿🤝🧑🏿 emoji we’ve built up with the code above.

Text Editing, which is what we want to do eventually, becomes quite difficult with Graphemes in the mix. Deleting a single byte might cause Rust to panic, because the result would no longer be a valid UTF8-String. Deleting 4 bytes to circumvent this would generate unwanted results. See for yourself:

```rust
fn main() {
    let emoji = "🧑🏿";
    //let fragment = &emoji[0..emoji.len() - 1];  //Deleting one byte panics
    let fragment = &emoji[0..emoji.len() - 4];    //Assuming Char=Grapheme and removing 4 bytes removes the skin color only
    println!("{}", fragment);
}
```

[Link to Rust Playground](https://play.rust-lang.org/?version=stable&mode=debug&edition=2021&gist=c64f145ee3e93138b8b343001d731acc)

To sum up the issue we’re facing: If we use Rust's internal `String`, the return value of `len()` is almost meaningless to us: It refers to the number of bytes, not the number of Characters or Graphemes in it.

## Introduction to unicode\_segmentation

We’re going to use a crate to help us with graphemes: [unicode\_segmentation](https://docs.rs/unicode-segmentation/latest/unicode_segmentation/index.html).

Let’s meet the crate in a Rust Playground:

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

[Link to Rust Playground](https://play.rust-lang.org/?version=stable&mode=debug&edition=2021&gist=33b92ab22248acd5199fe02a4b486adf)

Unicode\_segmentation provides us with an iterator over the grapheme clusters of a string, and with a trait which allows us to access this iterator by calling `graphemes()` on a `str` (we have to pass `true` to that function, otherwise Grapheme detection won’t always work as expected).

## Assignment 12: Proper Grapheme Handling (Pt. 1)

Let’s use this new knowledge to improve handle scrolling around grapheme clusters. We won’t fix it fully yet - scrolling around Emojis and other grapheme types will need an additional step - but we will already improve it noticeably.

Here is the assignment:

- Add `unicode_segmentation` to your project.
- Change the logic that handles substrings and the length of a line to work on Graphemes, not on byte length.
- You can use [this text file](https://raw.githubusercontent.com/pflenker/hecto-tutorial/a33a32990a4f7f75296431c358acb52bee0cc93b/test-graphemes-1.txt) to test `hecto`.

## Assignment 12: Code Review

My code uses a pretty straightforward and inefficient approach to solve this problem. The tricky part here was to effectively use the Iterator functions to get the results you want.  
As a minor foreshadowing I can already tell you that I will make my implementation more efficient in the next step.

When you added `unicode_segmentation` to your project, did you notice the text about `#![no_std] `? The [crate description](https://docs.rs/unicode-segmentation/latest/unicode_segmentation/index.html) mentions that it can be used in crates with this attribute. What does it mean?

Let’s start with describing `core`. `core` is Rusts library which contains the absolute essentials, like `Option`, `Result`, macros and other things. It is available in any Rust environment. `core` doesn’t assume you need OS support, because you might be building the OS yourself!

`std` includes everything in `core`, but adds a whole lot of other stuff that assumes OS-level features like threads, networking, heap memory allocation, file I/O, and more. Basically, `std` is `core` plus these standard OS-dependent functionalities. To remove the burden from figuring out what is `core` and what is `std`, `std` re-exports everything in `core`.

Compiling for a non-standard environments, for example embedded systems, is possible by setting `#![no_std]` in the main.rs, but it requires your dependencies to also work without `std`. So if you ever plan to work for embedded systems, you now know how to make emojis work there!

## Grapheme Cluster Width

Speaking of emojis: They do *not* work fully yet. Any line which includes an Emoji will still have scrolling issues, despite our best efforts: You won’t be able to fully scroll to the end of the line there.  
This also applies to certain other grapheme clusters. We have now corrected `hecto` so that the “length” of a line refers to the amount of visible grapheme clusters on screen, but what we didn’t account for yet is that grapheme clusters can have different widths!

Widths in unicode come in the following flavours:

- **Zero-Width Characters:** Characters like the Zero Width Joiner (ZWJ, we met this one earlier) are used in text formatting and have no width, meaning they do not consume space visually. The same (kind of) applies for non-printable characters. Since they are non-printable, they have no width.
- **Half-Width (Narrow) Characters:** Representing the standard printable characters, these typically have a width of 1 in monospaced environments like Terminals. This is the most common scenario in basic text handling.
- **Full-Width (Wide) Characters** This includes characters from complex scripts like Chinese, Japanese, and Korean. They are double-width because they take up two spaces in a grid of monospaced characters, which is crucial for proper alignment in text rendering. Here is the Latin alphabet as full width characters: ＡＢＣＤＥＦＧＨＩＪＫＬＭＮＯＰＱＲＳＴＵＶＷＸＹＺ.
- **Ambiguous:** Characters in this category are usually either narrow or wide depending on the context or font in which they appear. For example, certain symbols, like the mid dot `·` or various dashes `-`, are rendered as narrow or wide depending on the font.
- **Neutral:** These characters typically take up *some* space, but how much depends on the context or font. Emojis fall into this category, so do spaces or tabs.

We’re going to need another crate to help us out here: `unicode-width`. Let’s meet it in a Rust Playground:

```rust
use unicode_width::UnicodeWidthStr;
fn main() {
    let zero = "\x1b";              // Esc, see Chapter 3
    let half = "Ä";                 // German A with Umlaut
    let full = "Ａ";                // Like an A, but wider
    let neutral_1 = "👋";           // Normal Emoji
    let neutral_2 = "🧑🏿‍🤝‍🧑🏿"; // Compound Emoji
    let ambiguous = "·";            // Mid Dot
    println!("Type\tChar\tWidth\tWidth (CJK)");
    println!("Zero\t{zero}\t{}\t{}", zero.width(), zero.width_cjk());
    println!("Half\t{half}\t{}\t{}", half.width(), half.width_cjk());
    println!("Full\t{full}\t{}\t{}", full.width(), full.width_cjk());
    println!("N(1)\t{neutral_1}\t{}\t{}", neutral_1.width(), neutral_1.width_cjk());
    println!("N(2)\t{neutral_2}\t{}\t{}", neutral_2.width(), neutral_2.width_cjk()); 
    println!("A \t{ambiguous}\t{}\t{}", ambiguous.width(), ambiguous.width_cjk()); 
}
```

[Link to Rust Playground](https://play.rust-lang.org/?version=stable&mode=debug&edition=2021&gist=8bc95a5948e062344c4030886c6424f3)  
`cjk` stands for Chinese, Japanese and Korean, as these scripts are likely to contain characters of ambiguous widths.

Emojis are still giving us a headache. The good news is that „normal“ emojis can generally be considered to be full-width. The bad news is that all bets are off for compound emojis. The reason is that how emojis are rendered depends on the font: if it supports the compound emoji, it’s rendered as a wide character, if it doesn’t, it renders all the parts it is made of. Rust playground shows this nicely as it uses separate fonts - the input field doesn’t support compound emojis, the output field does.

## Character Width in hecto

Hecto will support narrow and wide characters only.

**Zero width characters** will be rendered with a narrow replacement character. This makes navigating and interacting with them easier anyways.

Our strategy for **ambiguous characters** will be to follow Unicode’s recommendation. The recommendation is to treat these ambiguous characters as narrow in non-CJK contexts, or where the context cannot be reliably determined, so I will use `width` instead of `width_cjk` going forward..

For **neutral characters**, `unicode-width` will make an educated guess whether it’s a wide character, a narrow one or something else entirely. If it’s 0, we treat it as above. If it’s above 2, we treat it as a wide character. This means that `hecto` will have visual glitches on terminals with  
fonts that don’t support compound characters.

## Assignment 13: Support Character Width

In this assignment, we’re going to properly support character widths. Depending on how closely your code followed my implementation steps, your concrete steps will differ from mine, but the ultimate goal stays the same: We want to properly support characters of different widths.

Our strategy will be to replace the `String` which stores the full string of the `Line` with a custom data structure per grapheme, which stores the grapheme itself, its width as approximate by `hecto` and a potential replacement. We’re going to try and pick a suitable replacement character for any character that won’t show up properly on screen, be it because it’s zero width or because it’s a control character like Bell.

This structure looks like this:

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

We need to update `from` to iterate over the grapheme clusters of the incoming string and generate our internal `Vec<TextFragment>`. For characters which `unicode-width` identifies as having a width of 0, we’re going to add a replacement character to be displayed instead of the actual grapheme. In this assignment, we’re only replacing any 0-width grapheme with a mid dot (`·`). We’ll add more sophisticated handling later.

Our `get` function needs to return graphemes visible in the given range. To achieve that, we need to iterate over our vector, keep track of the widths of each character we processed, and return graphemes which start and end in the given range. We also need to take special care at the boundaries of the screen: If a wide character is truncated by the screen edge, we’re going to print a horizontal ellipsis (`⋯`) to indicate that truncation happened.

Our view on what a `Location`, a `Position` and the Offset are needs to change a little bit. `Location` will now point to a single grapheme in a given line. `scroll_offset`, on the other hand, will no longer refer to a location within the text - it will instead refer to a point on an infinite grid full of cells, where each grapheme can fill one or two of these cells. Or in easier terms: If we imagine our terminal to extend endlessly, `scroll_offset` refers to a row and column on that endless terminal This means that a) `Position` no longer only describes the position on the visible screen, but the position on that grid, b) `scroll_offset` will no longer be a `Location`, but a `Position`, and c) we need a way to work with these structures to handle caret placement, movement within the document and scrolling properly.

To do so, we need a way to convert a `Location` into a `Position`. We do this by summing up all the widths that precede the current `x` of a given `Location`, to land at the correct cell. Since each line in a document takes up exactly one column on the grid, we can simply take over the value for `y` to determine the correct row.

With that change, our scrolling logic needs to convert the `Location` to a `Position` and work on that to determine whether or not the caret moved off-screen, and update `scroll_offset` accordingly. To determine the position of the caret on screen, we need to subtract the `Position` of the current grapheme on the grid from the `scroll_offset`.

We won’t need to touch our logic to move through the text. This means that we won’t always retain the correct vertical position if we change from one line to another, if the lines contain characters of different widths. But we either expect to work with consistent widths, or only with the occasional emoji, so the shift from one line to the next will be acceptable in most cases.

Here’s the assignment:

- Add `unicode-width` to your project.
- Refactor your code as outlined above.
- [Test your changes with this file.](https://raw.githubusercontent.com/pflenker/hecto-tutorial/7a8f2a734518ad016e5ed7ea5e804a3a5d718ce8/test-graphemes-2.txt)
- Ensure your structs, its members, its functions and variables are named appropriately to reflect the change.  
	**Code Review:** [This is how I did it.](https://github.com/pflenker/hecto-tutorial/commit/975c1dc8dfacff8ef2791d8ff0b47f0f6e8a42d8)

## Assignment 14: A Text Viewer

Let’s wrap this chapter up with some final touches and improve the way we handle invisible characters. Right now, we are replacing some of them with a mid dot, but not all of them. We’re also handling some zero-width characters a bit weirdly: if you have a string of 10 Zero-Width-Joiners, `unicode-segmentation` recognises them as *one* grapheme, therefore we replace them with *one* mid dot.  
However, `hecto` aims to be a text editor, the prime focus is displaying and editing visible text. Should anyone have the wish to edit invisible text, `hecto` is not for them.

Still, outputting invisible characters can have weird side effects on the terminal. For instance, a Tab (`\t`) can take up a weird amount of spaces on the terminal, which would lead to visual glitches. And if we won’t replace an Escape Character, which, as we learned, is a perfectly valid character in both ASCII and Unicode, it would be sent straight to the Terminal as a potential escape code that sets some weird terminal behaviour. This, by the way, is the reason terminals tend to get completely messed up if you mistakenly output a binary file as text to the terminal.

The reason why Tabs act so weird brings us back to mechanical typewriters again: Tab Stops - physical blockers which prevented the drum with the affixed paper from going past a certain point - where crucial in enabling consistent alignment of text, e.g. when typing up a resume. Since Tabs are so ancient, they are handled inconsistently depending on where they appear.

Unicode provides us with a wealth of replacement characters, should we be interested. All the “unprintables” from the ASCII shirt are there, including our old friends ␇ and ␛. [You can find the full list here](https://www.unicode.org/charts/PDF/U2400.pdf).

However, we’re not aiming for *completeness* in replacing characters, we’ll take a simpler approach.

Here is the assignment:

- A Space should render as a Space - this stays unchanged.
- Replace any tab (`\t`) with a Space.
- Replace every other visible whitespace (i.e. a whitespace with a non-zero length) with an [open box](https://www.compart.com/en/unicode/U+2423) (`␣`). To find out if a grapheme cluster is a whitespace, call `trim()` on it to remove all whitespaces, and then check with. `is_emtpy()` if anything remained.
- For zero-width-characters which are control characters like Bell or Null, replace them with a [White Vertical Rectangle](https://www.compart.com/en/unicode/U+25AF) (`▯`). You can use `is_control()` on a character to find out if it’s a control character, and you can turn both a `String` and a `str` into an iterator over its characters by calling `.chars()` on it.
- For any other zero-width character, we keep replacing them with the mid dot.
- You can test your changes with [this file.](https://raw.githubusercontent.com/pflenker/hecto-tutorial/ec2b6c025de70cbb8e40dc70db18fd3e0d415f00/test-character-replacement.txt)

**Code Review:** [This is how I did it.](https://github.com/pflenker/hecto-tutorial/commit/ec2b6c025de70cbb8e40dc70db18fd3e0d415f00)

## Wrap-Up and Outlook

Feature-wise, our progress was rather slow in this chapter. We did, however, learn a lot about Rust-Internals, like Strings, Vectors or how `panic!` works. We also learned a lot about the pitfalls of text processing and why emojis are so difficult to support.

The final result is now a working text viewer. In the [next chapter](https://philippflenker.com/hecto-chapter-5/), we will extend it to become a text editor.

---

[^1]: The cleaner solution here would be to save the panic hook in `new`, and to restore it in `drop`

[^2]: If you ask yourself: Why `u8` and not `Char`? Excellent question, hold onto it for a bit - at the end of this chapter, you’ll be able to answer it.

[^3]: At the time the closure is called, `take_hook` would no longer return the original hook, as it’s already overwritten by our own closure. So we would run into a recursion.