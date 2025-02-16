//! 1. Macros
//! Implement a macro vec_of_strings! that takes a list of string literals and converts them into a Vec<String>.
//! Implement a debug_log! macro that takes a message and prints it along with the file and line number.
//! Implement a DSL-like macro for defining HTTP routes.

#[allow(dead_code, unused_imports, unused_variables)]

mod map_reduce;
mod r#async;
mod refs;
mod leetcode;
mod my_mod;
mod atomics;
mod ticker;

use std::fmt::{Debug, Display};
use r#async::__exmaple_channels;
use map_reduce::{map_reduce_async, map_reduce_sync};
use refs::refs;
use tokio::time::{sleep, Duration};

macro_rules! calculate {
    // this will be invoked if: calculate { 23 + 34 };
    ($e:expr) => {
        {
            let val: usize = $e; // Force types to be unsigned integers
            println!("{} = {}", stringify!{$e}, val);
        }
    };
}

/// A DSL is a mini "language" embedded in a Rust macro. 
/// It is completely valid Rust because the macro system expands into normal Rust constructs, 
/// but it looks like a small language. 
/// This allows you to define concise or intuitive syntax for some special functionality (within bounds).
/// ## I have hardcoded `eval` keyword without which it wont work, that's `DSL (Domain specific lang)`
macro_rules! calculate_with_dsl {
    // this will be invoked if calcualte {eval 23 + 34};
    (eval $e:expr) => {
        {
            let val: usize = $e; // Force types to be unsigned integers
            println!("{} = {}", stringify!{$e}, val);
        }
    };
}

macro_rules! vec_of_strings {
    ($($s: expr);*) => {
        vec![$($s.to_string()),*]
    };
}

macro_rules! find_min {
    ($val: expr) => {$val};
    ($val1: expr, $($val2: expr),+) => {
        std::cmp::min($val1, find_min!($($val2), +))
    }
}

macro_rules! m {
    (1) => {};
}

macro_rules! hash_map {
    // creates an empty hashmap
    () => {
        std::collections::HashMap::new()
    };
    // creates a hashmap with key values being seperated by "=>"
    ($($key: expr => $val: expr), *) => {
        {
            let mut map = std::collections::HashMap::new();
            $(map.insert($key, $val);)*
            map
        }
    };
}

macro_rules! repeat {
    ($num: expr, $e: expr) => {
        {
            let mut v = Vec::with_capacity($num);
            for _ in 0..$num {
                v.push($e);
            }
            v
        }
    };
}

/// you can give an optional message to log at end of benchmarking
/// the time elapsed will added at the end of the provided message
macro_rules! benchmark {
    ($code: block) => {
        {
            let time = std::time::Instant::now();
            let result = $code;
            println!("It took {:?} to execute the given code block: {:?}", time.elapsed(), $code);
            result
        }
    };
    ($code: block, $mes: expr) => {
        {
            let time = std::time::Instant::now();
            let result = $code;
            println!("{}: {:?}", $mes, time.elapsed());
            result
        }
    };
}

// how do i implement a derrive macro for this trait ?? so that i can use it: #[derive(OwnDefault)]
pub trait OwnDefault {
    fn own_default() -> Self;
}

#[derive(Clone, Debug)]
struct Stack<T> {
    items: Vec<T>,
    top_item: T
}

impl<T: Default> OwnDefault for Stack<T> {
    fn own_default() -> Self {
        Stack {
            items: Vec::new(),
            top_item: T::default()
        }
    }
}

impl<T: std::default::Default + Clone> Stack<T> {
    pub fn new() -> Self {
        Stack::own_default()
    }

    pub fn push(&mut self, item: T) {
        self.items.push(item.clone());
        self.top_item = item;
    }

    pub fn pop(&mut self) -> Option<T> {
        // let last_el = &self.items[self.items.len() - 1];
        self.items.pop()
    }

    pub fn top(&self) -> T {
        self.top_item.clone()
    }

    fn empty(&self) -> bool {
        if self.items.len() == 0 {
            return true;
        }
        false
    }
}


#[derive(Debug, PartialEq, PartialOrd, Clone)]
struct Pair<T> {
    one: T,
    two: T
}

trait OwnAdd<T = Self> {
    type AddOutput;
    fn own_add(&self, rhs: T) -> Self;
}

trait OwnMul<X = Self> {
    type MulOutput;
    fn own_multiply(&self, rhs: X) -> Self::MulOutput;
}

impl<X: std::ops::Mul<Output = X> + Copy> OwnMul for Pair<X> {
    type MulOutput = Pair<X>;
    fn own_multiply(&self, rhs: Self) -> Self::MulOutput {
        Pair {
            one: self.one * rhs.one,
            two: self.two * rhs.two
        }
    }
}

impl<T: std::ops::Add<Output = T> + Copy> OwnAdd for Pair<T> {
    type AddOutput = Pair<T>;
    fn own_add(&self, rhs: Pair<T>) -> Self::AddOutput {
        Pair {
            one: self.one + rhs.one,
            two: self.two + rhs.two,
        }
    }
}

impl<T: PartialOrd + Debug> Pair<T> {
    fn new(one: T, two: T) -> Self {
        Pair {
            one,
            two
        }
    }
    fn larger(&self) {
        if self.one > self.two {
            println!("{:?}", self.one);
        }
        else if self.two > self.one {
            println!("{:?}", self.two);
        }
        else {
            println!("Both are same");
        }
    }
}

fn add_generic_vals<T>(val1: &dyn Display, val2: &dyn Display) -> String  {
    format!("{} + {} = {}", val1, val2, val1.to_string() + &val2.to_string())
}

fn apply_twice<F: Fn(usize) -> usize>(applier: F, num: usize) -> usize {
    let once = applier(num);
    applier(once)
}

trait Speak {
    fn say(&self) -> String;
}

struct Dog {
    name: &'static str,
}

struct Cat {
    name: &'static str,
}

struct Me {
    name: &'static str,
}

impl Speak for Dog {
    fn say(&self) -> String {
        format!("My name is {}, i say: {}", self.name, "WOOF !!")
    }
}

impl Speak for Cat {
    fn say(&self) -> String {
        format!("My name is {}, i say: {}", self.name, "MEOW !!")
    }
}

impl Speak for Me {
    fn say(&self) -> String {
        format!("My name is {}, i say: {}", self.name, "Radha Krishn !!")
    }
}

struct Container<X, Y>(X, Y);

trait Contains<A, B> {
    fn contains(&self, _: &A, _: &B) -> bool;
    fn first   (&self)               -> A;
    fn last    (&self)               -> B; 
}

impl<A, B> Contains<A, B> for Container<A, B> where 
    A: Copy + std::cmp::PartialEq<A>, 
    B: Copy + std::cmp::PartialEq<B>  
{
    fn contains(&self, item1: &A, item2: &B) -> bool {
        self.0 == *item1 && self.1 == *item2
    }
    fn first(&self) -> A {
        self.0
    }
    fn last(&self) -> B {
        self.1
    }
}

/// associate a trait bound with the type `C` using the `where` keyword
/// # Used trait bound with `where` keyword
fn __diff_i32_with_where<C>(container: C) -> i32 where C: Contains<i32, i32> {
    container.first() - container.last()
}

/// associate a trait bound with the type `C` directly in `<>` using `:`
/// # Used direct trait bound
fn __diff_i32_with_direct_bounds<C: Contains<i32, i32>>(container: C) -> i32 {
    container.first() - container.last()
}

/// directly tell compiler that the `argument: container` is a reference to `any type` that implements 
/// the `Contains<i32, i32>` trait using the `dyn` keyword.
/// # Used `dyn` keyword
fn __diff_i32_with_dyn(container: &dyn Contains<i32, i32>) -> i32 {
    container.first() - container.last()
}


/// A custom destructor trait implementation for a struct `Pair`
/// whenever `Pair` goes out of scope the `drop()` methods would be invoked !!
impl<T> Drop for Pair<T> {
    fn drop(&mut self) {
        println!("A Pair was dropped out of scope! Its the destructor :)")
    }
}

// This function takes ownership of a box and destroys it
fn eat_box_i32(boxed_i32: Box<i32>) {
    println!("Destroying box that contains {}", boxed_i32);
}

// This function borrows an i32
fn borrow_i32(borrowed_i32: &i32) {
    println!("This int is: {}", borrowed_i32);
}

#[cfg(panic = "unwind")]
fn ah() {
    println!("Spit it out!!!!");
}

#[cfg(not(panic = "unwind"))]
fn ah() {
    println!("This is not your party. Run!!!!");
}

fn drink(beverage: &str) {
    if beverage == "lemonade" {
        ah();
    } else {
        println!("Some refreshing {} is all I need.", beverage);
    }
}

fn _multiply(first_number_str: &str, second_number_str: &str) -> i32 {
    // Let's try using `unwrap()` to get the number out. Will it bite us?
    let first_number = first_number_str.parse::<i32>().unwrap();
    let second_number = second_number_str.parse::<i32>().unwrap();
    first_number * second_number
}

use std::{num::ParseIntError, str::FromStr};

#[derive(Debug, PartialEq)]
struct Point {
    x: i32,
    y: i32
}

#[derive(Debug, PartialEq, Eq)]
struct ParsePointError;

impl FromStr for Point {
    
    type Err = ParsePointError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {

        let (x, y) = s
            .strip_prefix('(')
            .and_then(|s| s.strip_suffix(')'))
            .and_then(|s| s.split_once(','))
            .ok_or(ParsePointError)?
        ;

        let x_fromstr = x.parse::<i32>().map_err(|_| ParsePointError)?;
        let y_fromstr = y.parse::<i32>().map_err(|_| ParsePointError)?;

        Ok(Point { x: x_fromstr, y: y_fromstr })
    }
}

fn _double_first(vec: Vec<&str>) -> Result<Option<i32>, ParseIntError> {
    let opt = vec.first().map(|first| {
        first.parse::<i32>().map(|n| 2 * n)
    });
    // swap Option with Result 
    opt.transpose()
}

struct Data<'a> {
    value: &'a str,
}

/// # WE CAN DO SUCH THINGS IN `Rust`
struct Closure<F> {
    data: (u8, u16),
    func: F,
}

impl<F> Closure<F>
    where F: Fn(&(u8, u16)) -> &u8,
{
    fn call(&self) -> &u8 {
        (self.func)(&self.data)
    }
}

fn do_it(data: &(u8, u16)) -> &u8 { &data.0 }

/// ### Note: debug expects two parameters with the *same* lifetime
fn debug<'a>(a: &'a str, b: &'a str) {
    println!("a = {a:?} b = {b:?}");
}


#[tokio::main]
async fn main() {
    // ticker::ticker_main();
    // ticker::ticker_mpsc_main();
    // ticker::ticker_mpsc_external_main();
    ticker::ticker_async_with_mutex_and_stop().await;
}

fn main3() {

    let clo = Closure { data: (0, 1), func: do_it };
    println!("\n looking for :{} \n", clo.call());

    let hello: &str = "hello";
    {
        let world = String::from("world");
        let world = &world; // 'world has a shorter lifetime than 'static
        debug(hello, world);
    }

    // __exmaple_channels();

    let v = String::from("Rust");

    let d = Data { 
        value: &v 
    };

    println!("{}", d.value);

    // More robust example showing the issue with temporary values:
    {
        let temp_string = String::from("Temporary");
        let temp_data = Data { value: &temp_string }; // This is fine within this scope
        println!("{}", temp_data.value);
    } 
    
    println!("{}", d.value);

    // my_mod::mods();

    atomics::__atomic_example();

}

fn main2() {

    refs();

    let strings = vec!["tofu", "93", "18"];
    let (numbers, errors): (Vec<_>, Vec<_>) = strings
        .into_iter()
        .map(|s| s.parse::<i32>())
        // partitions the vector into 2 vector based on the given closure of predicate !!
        .partition(Result::is_ok)
    ;

    println!("Numbers: {:?}", numbers);
    println!("Errors: {:?}", errors);

    drink("water");
    drink("lemonade");

    calculate! {23 + 56 / 89 + 70};
    calculate_with_dsl! {
        eval 23 + 56 / 89 + 70
    };
    
    // benchmark!({
    //     map_reduce_async();
    // }, "Async map-reduce took");

    // benchmark!({
    //     map_reduce_sync();
    // }, "Sync map-reduce took");

    // println!("\n ASYNC STUFF LOGGED !! \n");

    let boxed_i32 = Box::new(5_i32);
    let stacked_i32 = 6_i32;

    // Borrow the contents of the box. Ownership is not taken,
    // so the contents can be borrowed again.
    borrow_i32(&boxed_i32);
    borrow_i32(&stacked_i32);

    {
        // Take a reference to the data contained inside the box
        let _ref_to_i32: &i32 = &boxed_i32;

        // Error!
        // Can't destroy `boxed_i32` while the inner value is borrowed later in scope.
        // eat_box_i32(boxed_i32);
        eat_box_i32(boxed_i32.clone());
        // ^ FIXME! Comment out this line

        // Attempt to borrow `_ref_to_i32` after inner value is destroyed
        borrow_i32(_ref_to_i32);
        // `_ref_to_i32` goes out of scope and is no longer borrowed.
    }

    // `boxed_i32` can now give up ownership to `eat_box_i32` and be destroyed
    eat_box_i32(boxed_i32);

    benchmark!({
        let _tt = vec_of_strings!("134"; "2"; "3");
        let small = find_min!(1,23,45,2);

        m!(1);

        let _map = hash_map!("a" => 1, "b" => 2);
        let _str3 = repeat!(5, "Radha");
        println!("{:?}", small);
    });

    {
        let p = Pair {
            one: 23,
            two: 34
        };
    
        let new_p = p.own_add(Pair {
            one: 35,
            two: 12
        });
    
        let multiplied_p = p.own_multiply(new_p.clone());
    
        println!("{:?}", new_p);
        println!("{:?}", multiplied_p);
    }

    println!("After this line: 320. Pairs would be dropped !");

    let add_two = |x: usize| x + 2;
    let result = apply_twice(add_two, 5);

    println!("{}", result); // Output: 9

    // a vector of which ever values that implement 'Speak' trait 
    // 'dyn' keyword 
    let vec_of_dyn_trait_objects: Vec<Box<dyn Speak>> = vec![
        Box::new(Dog {
            name: "kutta"
        }),
        Box::new(Cat {
            name: "billi"
        }),
        Box::new(Me {
            name: "axn"
        })
    ];

    for trait_objects in vec_of_dyn_trait_objects {
        println!("{}", trait_objects.say());
    }

    // tuple struct
    let container = Container(23,34); 
    println!("{} + {} = {}", container.0, container.1, container.0 + container.1);

}