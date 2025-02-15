# 1. The `Send` Trait in Rust

The `Send` trait in Rust is a **marker trait** that indicates a type is safe to be sent between threads. It's a crucial part of Rust's concurrency model, ensuring **data race freedom**.

## Breakdown

### Marker Trait  
A **marker trait** is a trait that doesn't define any methods. Its sole purpose is to provide a **compile-time guarantee** about a type's properties.  

- `Send` is a marker trait.  
- If a type implements `Send`, it tells the compiler:  
  *"This type is safe to send to another thread."*

### Thread Safety  
The primary concern when working with multiple threads is **data races**. A **data race** occurs when:  

1. Two or more threads access the same memory location at the same time.  
2. At least one of those accesses is a **write**.  
3. The accesses are **not synchronized**.  

Data races lead to **undefined behavior** and are a major source of bugs in concurrent programs.  

### `Send`'s Role  
The `Send` trait guarantees that a type **does not have internal mutability** that could cause a data race if sent to another thread.  

- If a type `T` implements `Send`, it means that **moving** ownership of a `T` value to another thread is safe.  

## Types That Are `Send`  
Most common types in Rust implement `Send`:

- **Primitive types**: `i32`, `f64`, `bool`, `char`, etc.  
- **Shared immutable data**: `&str`, `Rc<T>` *(if `T` is `Send`)*.  
- **Types that own their data**: `String`, `Vec<T>` *(if `T` is `Send`)*.  
- **Synchronization primitives**: `Mutex<T>`, `Arc<T>`.  
- **Channels**: `mpsc::Sender<T>`, `mpsc::Receiver<T>`.  

## Types That Are *Not* `Send`  
Types that have **interior mutability** without proper synchronization are **not** `Send`.  

A common example is **`Rc<RefCell<T>>`**:  

- `RefCell<T>` allows interior mutability through `borrow()` and `borrow_mut()`.  
- However, it **does not provide synchronization primitives**.  
- Using `RefCell<T>` across threads would be unsafe, leading to race conditions.  
- That's why **`Rc<RefCell<T>>` is not `Send`**.  

<br>

---

# 2. Thread `Scopes`
in Rust allow spawned threads to borrow variables from the main thread safely. This is done using std::thread::scope, which ensures that all spawned threads complete execution before exiting the scope, preventing dangling references.

## 🔹 Problem Without Scoped Threads
Normally, when using thread::spawn, you must move ownership of variables into the thread, making them unavailable in the main thread:

```rust
use std::thread;

fn main() {
    let numbers = vec![1, 2, 3, 4];

    let handle = thread::spawn(move || {
        println!("Thread is using: {:?}", numbers);
    });

    // `numbers` is moved, so we cannot use it here
    // println!("{:?}", numbers); // ❌ ERROR!

    handle.join().unwrap();
}
```

## Problem:

We must move numbers into the thread, which prevents us from using it later.

## 🔹Solution: Using `thread::scope` (Scoped Threads)
Scoped threads allow borrowing instead of moving:

```rust
use std::thread;

fn main() {
    let numbers = vec![1, 2, 3, 4];

    thread::scope(|s| {
        s.spawn(|| {
            println!("Thread is using: {:?}", numbers); // ✅ Safe borrowing
        });
    });

    // `numbers` is still accessible after the thread ends
    println!("Main thread: {:?}", numbers);
}
```

## ✅ Why does this work?

- `thread::scope` ensures that all spawned threads finish execution before exiting the scope.

- This guarantees that numbers is still valid while the thread runs.

## 🔹 Benefits of Scoped Threads
- ✔ Borrowing Instead of Moving → Use variables in both main and spawned threads.
- ✔ No Need for Arc → Unlike normal threads, scoped threads can borrow without needing Arc<T>.
- ✔ Prevents Dangling References → Threads must finish before the scope exits, ensuring memory safety.

