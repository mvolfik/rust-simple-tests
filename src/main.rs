use std::io::Write;

mod fs;
mod thread;

macro_rules! tests {
    [$($name:expr),* $(,)?] => {
        [$((
            Box::new(|| {
                ($name()).map_err(|e| Box::<dyn std::error::Error>::from(e))
            }) as Box<dyn Fn() -> _>,
            stringify!($name),
        )),*]
    };
}

fn main() {
    let temp_dir = std::env::temp_dir();
    let temp_dir_path = temp_dir.join("rust_file_tests");
    println!("Using temporary directory: {:?}", temp_dir_path);
    if temp_dir_path.exists() {
        println!("Cleaning up previous test files...");
        std::fs::remove_dir_all(&temp_dir_path).expect("Failed to remove previous test files");
    }
    std::fs::create_dir_all(&temp_dir_path).expect("Failed to create temporary directory");
    std::env::set_current_dir(&temp_dir_path).expect("Failed to chdir to temporary directory");

    let tests = tests![
        fs::test_create_file,
        fs::test_write_and_read_file,
        fs::test_file_exists,
        fs::test_remove_file,
        fs::test_create_directory,
        fs::test_directory_exists,
        fs::test_remove_directory,
        fs::test_create_file_with_path,
        fs::test_file_write_multiple_times,
        fs::test_file_read_as_string,
        fs::test_read_file_with_bufreader,
        fs::test_read_file_with_file_open,
        fs::test_write_and_read_large_file,
        fs::test_create_nested_directories,
        fs::test_list_directory,
        fs::test_copy_file,
        fs::test_move_file,
        fs::test_empty_directory,
        fs::test_read_empty_file,
        fs::test_directory_listing_after_removal,
        fs::test_file_replacement,
        thread::test_create_thread,
        thread::test_mutex_counter,
        thread::test_scheduling,
        thread::test_condvar,
        thread::test_thread_join,
        thread::test_thread_sleep,
        thread::test_rwlock,
        thread::test_channel,
        thread::test_scoped_oncelock,
        thread::test_barrier,
        thread::test_thread_local_storage,
    ];

    let mut failed = 0;
    for (func, name) in tests {
        print!("Running {name}...");
        let _ = std::io::stdout().flush();
        match func() {
            Ok(_) => println!(" OK"),
            Err(e) => {
                failed += 1;
                println!(" FAILED: {}", e);
            }
        }
    }

    if failed == 0 {
        println!("All tests passed!");
    } else {
        println!("{failed} tests failed.");
        std::process::exit(1);
    }
}

#[macro_export]
macro_rules! assert_res {
    ($cond:expr $(,)?) => {
        if !$cond {
            return Err(concat!("Assertion failed: `", stringify!($cond), "` is false").into());
        }
    };
}

#[macro_export]
macro_rules! assert_eq_res {
    ($left:expr, $right:expr $(,)?) => {
        match (&$left, &$right) {
            (left_val, right_val) => {
                if !(*left_val == *right_val) {
                    return Err(format!(
                            "Assertion failed: `{}` != `{}`. Left: {:?}, Right: {:?}",
                            stringify!($left),
                            stringify!($right),
                            left_val,
                            right_val,
                        ).into(),
                    );
                }
            }
        }
    };
}
