# 1. `#[derive(...)]` — Auto-Implementing Traits

Rust allows you to automatically implement some common traits using `#[derive]`.

## Example:
```rust
#[derive(Debug, Clone, Copy, PartialEq)]
struct Point {
    x: i32,
    y: i32,
}
Here’s what each trait does:

Debug → Enables {:?} formatting for easy debugging.
Clone → Allows .clone() to create a duplicate.
Copy → Enables copying by assignment (let p2 = p1; without needing .clone()).
PartialEq → Allows == and != comparisons.
```

## Example Usage:
```rust
fn main() {
    let p1 = Point { x: 3, y: 4 };
    let p2 = p1; // Allowed because of `Copy`
    
    println!("{:?}", p1); // Works because of `Debug`
    
    if p1 == p2 {
        println!("Equal!"); // Works because of `PartialEq`
    }
}
```

# 2. `#[allow(...)]` & `#[warn(...)]` — Control Compiler Warnings
Rust sometimes gives warnings about unused variables, dead code, etc. You can suppress or enable them using these attributes.

Example:
```rust
#[allow(dead_code)]
fn unused_function() {
    println!("This function is never used, but no warning!");
}

#[warn(unused_variables)]
fn main() {
    let x = 5; // Will show a warning
}
allow(dead_code) → Stops warnings for unused functions.
warn(unused_variables) → Forces a warning for unused variables.
```

# 3. `#[inline]` & `#[inline(always)]` — Function Inlining
For performance, you can hint the compiler to inline a function.

Example:
```rust
#[inline]
fn add(a: i32, b: i32) -> i32 {
    a + b
}

#[inline(always)]
fn multiply(a: i32, b: i32) -> i32 {
    a * b
}
#[inline] → Suggests inlining but compiler decides.
#[inline(always)] → Forces inlining (not always recommended).
```

# 4. `#[repr(C)]` & `#[repr(u8)]` — Controlling Memory Layout
Rust doesn't guarantee struct or enum layout in memory. Use repr to make it predictable.

Example:
```rust
#[repr(C)] // Makes memory layout compatible with C
struct Data {
    a: u8,
    b: u32,
}

#[repr(u8)] // Forces enum to use `u8` size
enum Color {
    Red = 1,
    Green = 2,
    Blue = 3,
}
```

# 5. `#[cfg(...)]` — Conditional Compilation
You can compile code only in specific conditions, like OS or debug mode.

Example:
```rust
#[cfg(target_os = "windows")]
fn platform_specific() {
    println!("Running on Windows!");
}

#[cfg(target_os = "linux")]
fn platform_specific() {
    println!("Running on Linux!");
}

fn main() {
    platform_specific();
}

// target_os = "windows" → Only compiles on Windows.
// target_os = "linux" → Only compiles on Linux.
```

# 6. `#[test]` — Marking Test Functions
Rust uses #[test] to mark functions for unit testing.

Example:
```rust
#[test]
fn test_addition() {
    assert_eq!(2 + 2, 4);
}

// Run tests with: cargo test
```

# 7. `#[no_mangle]` — Prevent Name Mangling
When calling Rust functions from C, the compiler renames them by default. #[no_mangle] prevents this.

Example:
```rust
#[no_mangle]
pub extern "C" fn my_function() {
    println!("This function can be used in C!");
}

// extern "C" → Uses C calling convention.
// no_mangle → Keeps the function name as my_function instead of renaming it.
```

# 8. `#[unsafe]` — Unsafe Code
You need unsafe to work with raw pointers, direct memory access, or FFI.

Example:
```rust
fn raw_pointer_demo() {
    let x = 5;
    let r = &x as *const i32; // Raw pointer
    
    unsafe {
        println!("Value at pointer: {}", *r); // Dereferencing requires `unsafe`
    }
}
```

# 9. `#[derive(Default)]` — Auto-Implement Default Values
Instead of manually writing a default constructor, use `#[derive(Default)]`.

Example:
```rust
#[derive(Default, Debug)]
struct User {
    name: String,
    age: u8,
}

fn main() {
    let user: User = Default::default();
    println!("{:?}", user); // User { name: "", age: 0 }
}
```