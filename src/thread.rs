// Attribution: large part of this file was written by ChatGPT.
// The author has then verified the correctness of the code and
// added some tests to cover missing functionality.

use std::sync::{Arc, Barrier, Condvar, Mutex, OnceLock, RwLock, mpsc};
use std::thread;
use std::time::Duration;

use crate::{assert_eq_res, assert_res};

const THREAD_COUNT: usize = 10;
const ITER_COUNT: usize = 20;

// Test creating a thread and ensuring it runs
pub fn test_create_thread() -> Result<(), Box<dyn std::error::Error>> {
    let handle = thread::spawn(|| {
        // Simulate work
        thread::sleep(Duration::from_millis(10));
    });

    handle.join().map_err(|_| "Thread panicked".to_string())?;
    Ok(())
}

// Test shared mutable state with Mutex
pub fn test_mutex_counter() -> Result<(), Box<dyn std::error::Error>> {
    let counter = Arc::new(Mutex::new(0));
    let mut handles = vec![];

    for _ in 0..THREAD_COUNT {
        let counter = Arc::clone(&counter);
        let handle = thread::spawn(move || {
            for _ in 0..ITER_COUNT {
                let mut num = counter
                    .lock()
                    .map_err(|_| "Failed to lock mutex".to_string())?;
                *num += 1;
            }
            Ok::<_, String>(())
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().map_err(|_| "Thread panicked".to_string())??;
    }

    let result = *counter
        .lock()
        .map_err(|_| "Failed to lock mutex".to_string())?;
    assert_eq_res!(result, THREAD_COUNT * ITER_COUNT);
    Ok(())
}

// Test shared mutable state with Mutex
pub fn test_scheduling() -> Result<(), Box<dyn std::error::Error>> {
    let counter = Arc::new(Mutex::new(0));
    let mut handles = vec![];

    for i in 0..THREAD_COUNT {
        let counter = Arc::clone(&counter);
        let handle = thread::spawn(move || {
            for j in 0..ITER_COUNT {
                let mut num = counter
                    .lock()
                    .map_err(|_| "Failed to lock mutex".to_string())?;

                let upper = j * THREAD_COUNT;
                let lower = (j + 1) * THREAD_COUNT;
                if *num < upper {
                    return Err(format!(
                        "Thread {i}, loop iteration {j}: value {} not in expected range {}..{}",
                        *num, upper, lower
                    ));
                }
                *num += 1;
                drop(num); // unlock
                std::thread::sleep(Duration::from_millis(5));
            }
            Ok::<_, String>(())
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().map_err(|_| "Thread panicked".to_string())??;
    }

    let result = *counter
        .lock()
        .map_err(|_| "Failed to lock mutex".to_string())?;
    assert_eq_res!(result, THREAD_COUNT * ITER_COUNT);
    Ok(())
}

// Test condition variable usage
pub fn test_condvar() -> Result<(), Box<dyn std::error::Error>> {
    let pair = Arc::new((Mutex::new(false), Condvar::new()));
    let pair1 = Arc::clone(&pair);
    let pair2 = Arc::clone(&pair);

    let thread1 = thread::spawn(move || {
        let (lock, cvar) = &*pair1;
        let mut started = lock
            .lock()
            .map_err(|_| "Failed to lock mutex".to_string())?;
        *started = true;
        cvar.notify_one(); // Notify that the condition has changed
        Ok::<_, String>(())
    });

    let thread2 = thread::spawn(move || {
        let (lock, cvar) = &*pair2;
        let mut started = lock
            .lock()
            .map_err(|_| "Failed to lock mutex".to_string())?;
        while !*started {
            started = cvar
                .wait(started)
                .map_err(|_| "Failed to wait".to_string())?;
        }
        Ok::<_, String>(())
    });

    thread1
        .join()
        .map_err(|_| "Thread 1 panicked".to_string())??;
    thread2
        .join()
        .map_err(|_| "Thread 2 panicked".to_string())??;

    // Check that the condition was indeed changed
    let (lock, _) = &*pair;
    let started = lock
        .lock()
        .map_err(|_| "Failed to lock mutex".to_string())?;
    assert_res!(*started);
    Ok(())
}

// Test thread joining
pub fn test_thread_join() -> Result<(), Box<dyn std::error::Error>> {
    let handle = thread::spawn(|| {
        // Return a value from the thread
        42
    });

    let result = handle.join().map_err(|_| "Thread panicked".to_string())?;
    assert_eq_res!(result, 42);
    Ok(())
}

// Test sleeping a thread
pub fn test_thread_sleep() -> Result<(), Box<dyn std::error::Error>> {
    let start_time = std::time::Instant::now();
    let handle = thread::spawn(|| {
        thread::sleep(Duration::from_millis(50));
    });

    handle.join().map_err(|_| "Thread panicked".to_string())?;
    let elapsed = start_time.elapsed();
    assert_res!(elapsed.as_millis() >= 50);
    assert_res!(elapsed.as_millis() < 100); // Ensure it didn't sleep too long
    Ok(())
}

// Test using RwLock for read and write access
pub fn test_rwlock() -> Result<(), Box<dyn std::error::Error>> {
    let rwlock = Arc::new(RwLock::new(0));
    let mut handles = vec![];

    // Create writer threads
    for _ in 0..THREAD_COUNT {
        let rwlock = Arc::clone(&rwlock);
        let handle = thread::spawn(move || {
            let mut num = rwlock
                .write()
                .map_err(|_| "Failed to lock RwLock for writing".to_string())?;
            *num += 1;
            Ok::<_, String>(())
        });
        handles.push(handle);
    }

    // Create reader threads
    for _ in 0..THREAD_COUNT {
        let rwlock = Arc::clone(&rwlock);
        let handle = thread::spawn(move || {
            let num = rwlock
                .read()
                .map_err(|_| "Failed to lock RwLock for reading".to_string())?;
            assert_res!(*num >= 1);
            Ok::<_, String>(())
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().map_err(|_| "Thread panicked".to_string())??;
    }

    let final_count = *rwlock
        .read()
        .map_err(|_| "Failed to lock RwLock for reading".to_string())?;
    assert_eq_res!(final_count, THREAD_COUNT as i32);
    Ok(())
}

// Test using channels for sending data between threads
pub fn test_channel() -> Result<(), Box<dyn std::error::Error>> {
    let (sender, receiver) = mpsc::channel();
    let sender_handle = thread::spawn(move || {
        for i in 0..THREAD_COUNT {
            sender
                .send(i)
                .map_err(|_| "Failed to send message".to_string())?;
        }
        Ok::<_, String>(())
    });

    let receiver_handle = thread::spawn(move || {
        let mut received = vec![];
        for _ in 0..THREAD_COUNT {
            let num = receiver
                .recv()
                .map_err(|_| "Failed to receive message".to_string())?;
            received.push(num);
        }
        Ok::<_, String>(received)
    });

    sender_handle
        .join()
        .map_err(|_| "Sender thread panicked".to_string())??;
    let received = receiver_handle
        .join()
        .map_err(|_| "Receiver thread panicked".to_string())??;

    let expected: Vec<_> = (0..THREAD_COUNT).collect();
    assert_eq_res!(received, expected);
    Ok(())
}

// Test using Barrier for synchronizing threads
pub fn test_barrier() -> Result<(), Box<dyn std::error::Error>> {
    let barrier = Arc::new(Barrier::new(THREAD_COUNT));
    let mut handles = vec![];

    for _ in 0..THREAD_COUNT {
        let barrier = Arc::clone(&barrier);
        let handle = thread::spawn(move || {
            // Do some work
            thread::sleep(std::time::Duration::from_millis(10));
            barrier.wait(); // Wait for others to reach this point
            Ok::<_, String>(())
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().map_err(|_| "Thread panicked".to_string())??;
    }

    Ok(())
}

// Test OnceLock initialization in separate trhead
pub fn test_scoped_oncelock() -> Result<(), Box<dyn std::error::Error>> {
    let lock: OnceLock<i32> = OnceLock::new();
    thread::scope(|s| {
        s.spawn(|| {
            // Initialize the lock
            lock.get_or_init(|| 42);
        })
        .join()
        .map_err(|_| "Thread panicked".to_string())
    })?;

    // Check if initialization was successful
    assert_eq_res!(lock.get(), Some(&42));
    Ok(())
}

// Test thread-local storage
pub fn test_thread_local_storage() -> Result<(), Box<dyn std::error::Error>> {
    use std::cell::RefCell;

    // Define thread-local storage
    thread_local! {
        static THREAD_LOCAL: RefCell<usize> = RefCell::new(0);
    }

    let mut handles = vec![];

    for i in 0..THREAD_COUNT {
        let handle = thread::spawn(move || {
            THREAD_LOCAL.with(|local| {
                *local.borrow_mut() = i + 20;
                std::thread::sleep(Duration::from_millis(10));
                std::hint::black_box(&local);
                // Check the value in this thread did not change
                assert_eq_res!(*local.borrow(), i + 20);
                Ok::<_, String>(())
            })?;
            Ok::<_, String>(())
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().map_err(|_| "Thread panicked".to_string())??;
    }

    THREAD_LOCAL.with(|local| {
        let final_value = *local.borrow();
        assert_eq_res!(final_value, 0); // Each thread has its own value
        Ok::<_, String>(())
    })?;

    Ok(())
}
