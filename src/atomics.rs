use std::{sync::atomic::{AtomicU16, AtomicU8, Ordering}, thread};

/// [`std::sync::atomic`] in Rust provide a way to work with shared mutable state across threads 
/// without needing explicit locks (like Mutex or RwLock) for certain specific operations. 
/// They achieve this through atomic operations, which are guaranteed to be performed indivisibly, 
/// even in the presence of multiple threads.
/// 
/// ## What Atomics Are: 
/// Atomics are data types that provide atomic operations.  
/// An atomic operation is an operation that is guaranteed to be completed in a single, uninterruptible step.  
/// Even if multiple threads try to perform the same atomic operation at the same time, 
/// the operation will be performed correctly, without any interference or data corruption.
/// 
/// ## Key Features of Atomics:
/// Atomic Operations: Atomics provide methods for performing atomic operations, such as:
/// - `fetch_add:` Atomically increments a value and returns the previous value.
/// - `fetch_sub:` Atomically decrements a value and returns the previous value.
/// - `compare_and_swap:` Atomically compares a value with an expected value and, if they are equal, replaces the value with a new value.
/// - `load:` Atomically loads a value.
/// - `store:` Atomically stores a value
/// 
/// ## Memory ordering "[`std::sync::atomic::Ordering`]": 
/// Atomic operations have memory ordering semantics.  
/// Memory ordering specifies how the effects of atomic operations are visible to other threads.  
/// Rust provides different memory ordering options (e.g., `SeqCst, Acquire, Release, Relaxed`) 
/// to control the level of synchronization. 
/// - Memory orderings specify the way atomic operations synchronize memory. 
/// - In its weakest `Ordering::Relaxed`, only the memory directly touched by the operation is synchronized. 
/// - On the other hand, a store-load pair of `Ordering::SeqCst` operations synchronize other memory while additionally preserving a total order of such operations across all threads.
/// 
/// ## Thread Safety: 
/// Atomic types are inherently thread-safe.  
/// You can safely share atomic variables between threads without needing any additional synchronization mechanisms 
/// (like Mutex).
/// 
/// 

pub fn __atomic_example() {

    let atomic1 = AtomicU16::new(0);
    let atomic2 = AtomicU16::new(0);

    thread::scope(|s| {

        // when storing the val in the atomic if Ordering is Relaxed, it would lazily sync the memory 
        // across threads, so when we try to print the atomic in another thread the atomic value
        // can show undefined behaviour based on which thread access it first

        // however if we pass the Ordering::SeqCast then it would instantly sync the atomic value
        // and all threads will read the same atomic value

        s.spawn(|| atomic1.store(1, Ordering::Relaxed)); // atomic store
        s.spawn(|| println!("THE ATOMIC VAL in spawned thread RELAXED: {:?}", atomic1.load(Ordering::Relaxed)));
        s.spawn(|| unsafe { atomic1.as_ptr().write(2) }); // non-atomic write

        // 

        s.spawn(|| atomic2.store(1, Ordering::SeqCst)); // atomic store
        s.spawn(|| println!("THE ATOMIC VAL in spawned thread SEQCST: {:?}", atomic2.load(Ordering::Relaxed)));
        s.spawn(|| unsafe { atomic2.as_ptr().write(2) }); // non-atomic write
    });

    println!("THE ATOMIC VALs: {:?}, {:?}", atomic1.load(Ordering::Relaxed), atomic2.load(Ordering::Relaxed));

    let mut num = 0;

    thread::scope(|s| {
        s.spawn(|| {
            num += 10; // ✅ Can mutate safely!
        });
    });

    println!("Updated num: {}", num);
}

// ! TODO:
// - subtyping
// - Coercions
// - macros revision
// - example