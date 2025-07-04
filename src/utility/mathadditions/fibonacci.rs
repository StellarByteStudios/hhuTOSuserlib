use core::sync::atomic::{AtomicUsize, Ordering};

static CALLS: AtomicUsize = AtomicUsize::new(0);
pub fn calculate_fibonacci_rec(x: u64) -> u64 {
    let times_called = CALLS.fetch_add(1, Ordering::SeqCst);

    //return 42;
    if x <= 2 {
        return 1;
    }
    return calculate_fibonacci_rec(x - 1) + calculate_fibonacci_rec(x - 2);
}
