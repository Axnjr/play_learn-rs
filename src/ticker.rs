use tokio::time::{self, sleep};
use std::{sync::{atomic::{AtomicBool, AtomicUsize, Ordering}, mpsc::{self, Receiver, Sender}, Arc, Mutex}, thread, time::Duration};
use tokio::sync::Notify;


fn ticker<F>(mut func: F) where F: FnMut() + Send + 'static, {
    loop {
        func();
        thread::sleep(Duration::from_secs(1));
    }
}


/// ### The `tokio::select!` macro allows waiting on multiple async computations and returns when a single computation completes.
pub async fn async_ticker_with_notification_mechanism(signal: Arc<Notify>) {
    let mut counter = 0;
    loop {
        tokio::select! {
            _ = signal.notified() => {
                println!("Received shutdown signal, stopping ticker...");
                break;
            }
            _ = sleep(Duration::from_secs(1)) => {
                println!("Ticker executing. Counter: {}", counter);
                counter += 1;
            } 
        }
    }
}


pub async fn async_ticker_with_notification_mechanism_main() {

    let signal = Arc::new(Notify::new());
    let signal_clone = Arc::clone(&signal);

    let handle = tokio::spawn(async_ticker_with_notification_mechanism(signal_clone));

    sleep(Duration::from_secs(5)).await;
    println!("Sending shutdown signal...");

    signal.notify_waiters();

    handle.await.unwrap();
    println!("Ticker stopped gracefully.");
}


async fn _ticker_async_with_atomic_and_stop() {
    
    let counter = Arc::new(AtomicUsize::new(0));
    let running = Arc::new(std::sync::atomic::AtomicBool::new(true));

    let counter_clone = Arc::clone(&counter);
    let running_clone = Arc::clone(&running);

    let handle = tokio::spawn(async move {

        while running_clone.load(Ordering::SeqCst) {

            let current_count = counter_clone.fetch_add(1, Ordering::SeqCst);
            println!("Ticker executing. Counter: {}", current_count + 1); // Print the incremented value

            sleep(Duration::from_secs(1)).await;
        }

        println!("Ticker task stopped.");
    });

    sleep(Duration::from_secs(5)).await;
    println!("Main task doing some work...");
    sleep(Duration::from_secs(3)).await;

    println!("Stopping the ticker...");
    running.store(false, Ordering::SeqCst);

    handle.await.unwrap();
    println!("Ticker stopped. Main task continues.");
}


pub async fn ticker_async_with_mutex_and_stop() {

    let counter = Arc::new(Mutex::new(0));
    let stop_bool = Arc::new(AtomicBool::new(false));

    let counter_clone = Arc::clone(&counter);
    let stop_bool_clone = Arc::clone(&stop_bool);

    let handle = tokio::spawn(async move {
        while !stop_bool_clone.load(Ordering::SeqCst) {
            { 
                // critical section
                let mut count = counter_clone.lock().unwrap(); // Acquire the lock
                println!("Ticker executing the closure. Counter: {}", *count);
                *count += 1;
            } 
            // Lock released here
            sleep(Duration::from_secs(1)).await; // Asynchronous sleep
        }
    });

    // the main task running (otherwise, the spawned task will be terminated) or else join() the handle.
    // loop {
    //     sleep(Duration::from_secs(5)).await; // Example: Print something every 5 secs
    //     println!("Main task is still running!");
    // }

    sleep(Duration::from_secs(5)).await;
    println!("Main task doing some work...");
    sleep(Duration::from_secs(3)).await;

    // Stop the ticker task
    println!("Stopping the ticker...");
    stop_bool.store(true, Ordering::SeqCst);

    // handle.await.unwrap();
    println!("Ticker stopped. Main task continues.");

}


fn ticker_mpsc<F>(mut func: F) where F: FnMut() + Send + 'static, {
    
    let (tx, rx) = mpsc::channel();

    // Spawn a separate thread to act as a timer
    thread::spawn(move || {
        loop {
            tx.send(()).unwrap(); // Send a tick signal
            thread::sleep(Duration::from_secs(1)); // Avoid blocking the main thread
        }
    });

    // Main ticker loop reacting to tick events
    for _ in rx {
        func();
    }
}


fn ticker_mpsc_external(tx: Sender<()>) {
    // Spawn a separate thread to act as a timer
    thread::spawn(move || {
        loop {
            tx.send(()).unwrap(); // Send a tick signal
            thread::sleep(Duration::from_secs(1)); // Avoid blocking the main thread
        }
    });
}


pub fn ticker_mpsc_external_main() {

    let mut counter = 0;
    let (tx, rx) = mpsc::channel();

    ticker_mpsc_external(tx);

    let lt = thread::spawn(move || {
        println!("I the thread who listens to timer threads");
        for _ in rx {
            println!("Ticker executing the closure of the main function. Counter: {}", counter);
            counter += 1;
        }
    });

    println!("IM THE MAIN THREAD !!");
    
    lt.join().unwrap();
}


pub fn ticker_mpsc_main() {

    let mut counter = 0;

    let handle1 = thread::spawn(move || {
        ticker_mpsc(move || {
            println!("Ticker executing the closure of the main function. Counter: {}", counter);
            counter += 1;
        });
    });

    println!("I'm the main thread!");

    let handle2 = thread::spawn(|| {
        loop {
            println!("I am another thread!!");
            thread::sleep(Duration::from_secs(1)); // This part can also be removed with async
        }
    });

    println!("\n Main thread: JOINIG BOTH THREADS ! \n");

    handle1.join().unwrap();
    handle2.join().unwrap();

    println!("\n Main thread: BOTH THREADS JOINED \n");

}


pub async fn ticker_main() {

    let mut counter = 0;

    let handle1 = thread::spawn(move || {
        ticker(move || {
            println!("Ticker executing the closure of the main function. Counter: {}", counter);
            counter += 1;
        });
    });

    println!("I'm the main thread!");

    let handle2 = thread::spawn(|| {
        loop {
            println!("I am another thread!!");
            thread::sleep(Duration::from_secs(1));
        }
    });

    // Join threads to prevent the main thread from exiting early
    handle1.join().expect("Ticker thread panicked");
    handle2.join().expect("Another thread panicked");

    println!("Main thread got over!");

}