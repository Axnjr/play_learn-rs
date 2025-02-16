/// # TODO
/// - `Send`        ✅
/// - `Rc`          ✅
/// - `Arc`         ✅
/// - `Mutex`       ✅
/// - `RwLock`      ✅
/// 
/// - `Lifetimes`
/// - `unsafe`
/// - `Coercions` # obtain (something) from someone by using force or threats.
/// - `revison`

use std::{cell::RefCell, rc::Rc, thread};

/// ## The `Send`` trait in Rust is a `marker trait` that indicates that a type is `safe` to be sent between
/// `threads.`  
/// ## It's a crucial part of Rust's concurrency model, ensuring data race freedom.
/// ### Here's a breakdown:
/// 
///     - Marker Trait: A marker trait is a trait that doesn't define any methods.  
///         - Its sole purpose is to provide a compile-time guarantee about a type's properties.  
///         - Send is a marker trait; if a type implements Send, it tells the compiler 
///         - "this type is safe to send to another thread."
/// 
///     - Thread Safety:  The primary concern when working with multiple threads is data races. 
///         - A data race occurs when two or more threads access the same memory location at the same time, 
///         - and at least one of those accesses is a write, and the accesses are not synchronized. 
///         - Data races lead to undefined behavior and are a major source of bugs in concurrent programs.   

///     - Send's Role: The Send trait guarantees that a type does not have any internal mutability that 
///     could cause a data race if it were sent to another thread.  If a type T implements Send, it means 
///     that it's safe to move the ownership of a value of type T to another thread.

/// ## Types that are Send:  Most common types in Rust are Send:
/// - Primitive types (e.g., i32, f64, bool, char).
/// - Shared immutable data (e.g., &str, Rc<T> if T is Send).
/// - Types that own their data (e.g., String, Vec<T> if T is Send).
/// - Mutexes and other synchronization primitives (e.g., Mutex<T>, Arc<T>).
/// - Channels (mpsc::Sender<T>, mpsc::Receiver<T>).
/// ## Types that are not Send: 
/// Types that have interior mutability without proper synchronization are typically not Send. 
/// A prominent example is `Rc<RefCell<T>>`. RefCell allows interior mutability through its 
/// `borrow` and `borrow_mut` methods, but it does not provide any synchronization primitives, 
/// so using it across threads would be unsafe.  That's why Rc<RefCell<T>> is not Send.


/// This type is Send because it owns its data.
struct MyData {
    value: i32,
}

// This type is NOT Send because it allows unsynchronized interior mutability.
struct NotSend {
    some_counter: Rc<RefCell<i32>>,  // Imagine this, Rc<RefCell<>> is not Send
}

use std::sync::mpsc::{Sender, Receiver};
use std::sync::{mpsc, Arc, Mutex, RwLock};

static NTHREADS: i32 = 3;

/// Channels have two endpoints: the `Sender<T>` and the `Receiver<T>`,
/// where `T` is the type of the message to be transferred

pub fn __exmaple_channels() {

    let (transmitter, reciver): (Sender<String>, Receiver<String>) = mpsc::channel();
    let mut children = Vec::new();

    for id in 0..NTHREADS {
        // The sender endpoint can be copied
        let thread_tx = transmitter.clone();

        // Each thread will send its id via the channel
        let child = thread::spawn(move || {
            // The thread takes ownership over `thread_tx`
            // Each thread queues a message in the channel
            thread_tx.send(format!("This is thraed no. {}, sending message to u ppl !", id)).unwrap();

            // Sending is a non-blocking operation, the thread will continue
            // immediately after sending its message
            println!("thread {} finished", id);
        });

        children.push(child);
    }

    // Here, all the messages are collected
    let mut ids = Vec::with_capacity(NTHREADS as usize);
    for _ in 0..NTHREADS {
        // The `recv` method picks a message from the channel
        // `recv` will block the current thread if there are no messages available
        ids.push(reciver.recv());
    }
    
    // Wait for the threads to complete any remaining work
    for child in children {
        child.join().expect("oops! the child thread panicked");
    }

    // Show the order in which the messages were sent
    println!("{:?}", ids);
}


pub fn async_ops() {
    let data = MyData { value: 42 };

    // This is fine because MyData is Send.
    thread::spawn(move || {
        println!("{}", data.value); // Data is moved into the thread.
    });

    let _not_send_data = NotSend{
        some_counter: Rc::new(RefCell::new(23))
    };

    // ! `Rc<RefCell<i32>>` cannot be sent between threads safely
    // ! within `{closure@src\async.rs:67:19: 67:26}`, the trait `Send` is not implemented for `Rc<RefCell<i32>>`
    // The following will cause a compile-time error:
    // thread::spawn(move || {
    //     // Accessing not_send_data here would be unsafe.
    //     println!("Not Send data {:?}", not_send_data.some_counter);
    // });

    let mut children = vec![];

    for i in 0..10 {
        // Spin up another thread
        children.push(thread::spawn(move || {
            println!("this is thread number {}", i);
        }));
    }

    for child in children {
        // Wait for the thread to finish. Returns a result.
        let _ = child.join();
    }
}


/// ### The `lock()` function on `Mutex` returns a `MutexGuard` which we need to derefrence using `*` 
/// ### inorder to modify the interior data, this is called `Interior Mutability` because notice 
/// ### we've not marked our `shared_count` as `mut` !!
fn __shared_counter() {
    let mut threads = vec![];
    let shared_count = Arc::new(Mutex::new(0));
    for num in 0..3 {
        let each_thread_count_clone = Arc::clone(&shared_count); //shared_count.clone();
        let thread = thread::spawn(move || {
            let mut cnt = each_thread_count_clone.lock().unwrap();
            *cnt += 10;
            println!("I am thread no. {}, Counter: {}", num, cnt);
        });
        threads.push(thread);
    }
    for each_thread in threads {
        let _ = each_thread.join();
    }
    println!("Final Counter: {}", shared_count.lock().unwrap());
}


/// # What are these ?
/// [`Mutex`]: allows only one thread at a time to access the underlying data, either to `read or write / update` !
/// 
/// [`RwLock`]: allows any number of readers but at most one writer at any point in time. 
/// - The write portion of this lock typically allows modification of the underlying data (exclusive access) 
/// - and the read portion of this lock typically allows for read-only access (shared access).

/// # In comparison, 
/// a [`Mutex`] does not distinguish between `readers` or `writers` that acquire the lock, 
/// therefore blocking any threads waiting for the lock to become available. 
/// 
/// # Summary
/// ### An `RwLock` will allow any number of readers to acquire the lock as long as a writer is not holding the lock.
fn __mutex_poisoning_example() {

    let lock = Arc::new(Mutex::new(0));
    let lock2 = Arc::clone(&lock);

    let _ = thread::spawn(move || -> () {
        // This thread will acquire the mutex first, unwrapping the result of
        // `lock` because the lock has not been poisoned.
        let _guard = lock2.lock().unwrap();
        // This panic while holding the lock (`_guard` is in scope) will poison the mutex.
        // ! `Mutex` posioned here !!
        panic!();
    }).join();


    match lock.lock() {
        Ok(num) => println!("Counter: {}", num),
        Err(poisoned) => {
            println!("Mutex got poisoned! Recovering...");
            // use `into_inner()` to Recover the data regardless of possibilty that the data might be corrupted!
            let num = poisoned.into_inner(); 
            println!("Recovered Counter: {}", num);
        }
    };

    let _data = lock
        .lock()
        .unwrap_or_else(|poisoned| poisoned.into_inner())
    ;

}

