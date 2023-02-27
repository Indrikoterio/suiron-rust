//! Functions for limiting execution time.
//!
//! This module contains a global variable, SUIRON_STOP_QUERY,
//! and therefore has 'unsafe' code.

use std::time::Duration;
use thread_timer::ThreadTimer;

use super::logic_var::*;

static mut SUIRON_STOP_QUERY: bool = false;

/// Create a timer with a timeout in milliseconds.
///
/// When the timer times out, it sets the SUIRON_STOP_QUERY
/// flag to true, effectively halting a query.
///
/// # Arguments
/// * `timeout` - in milliseconds
/// # Return
/// * `timer` - ThreadTimer
/// # Usage
/// ```
/// use suiron::*;
///
/// let timer = start_query_timer(300);
/// ```
pub fn start_query_timer(milliseconds: u64) -> ThreadTimer {
    unsafe { SUIRON_STOP_QUERY = false; }
    let timer = ThreadTimer::new();
    timer.start(Duration::from_millis(milliseconds),
                move || { stop_query(); }).unwrap();
    return timer;
} // start_query_timer()

/// Cancels query timer. Ignores any issues.
///
/// # Argument
/// * `timer` - ThreadTimer
/// # Usage
/// ```
/// use suiron::*;
///
/// let timer = start_query_timer(30);
/// cancel_timer(timer);
/// ```
pub fn cancel_timer(timer: ThreadTimer) {
    match timer.cancel() {
        Ok(_) => {},
        Err(_) => {},
    }
} // cancel_timer()

/// Sets the SUIRON_STOP_QUERY flag to false and LOGIC_VAR_ID to 0.
///
/// The SUIRON_STOP_QUERY is checked in count_rules(), in knowledgebase.rs.
/// Setting it true effectively stops the search for a solution.
///
/// In order to keep the substitution set small, the LOGIC_VAR_ID is
/// reset to 0 at the start of every query.
pub fn start_query() {
    unsafe { SUIRON_STOP_QUERY = false; }
    clear_id();
}

/// Sets the SUIRON_STOP_QUERY flag to true.
///
/// The SUIRON_STOP_QUERY is checked in count_rules(), in knowledgebase.rs.
/// Setting it `true` effectively stops the search for a solution.
pub fn stop_query() {
    unsafe { SUIRON_STOP_QUERY = true; }
}

/// Returns value of SUIRON_STOP_QUERY.
///
/// If the SUIRON_STOP_QUERY is true, it means that the query timed out.
/// # Return
/// * `SUIRON_STOP_QUERY` - bool
pub fn query_stopped() -> bool {
    unsafe { SUIRON_STOP_QUERY }
}

#[cfg(test)]
mod test {

    use super::*;
    use std::thread;
    use std::time::Duration;
    use serial_test::serial;

    // Test timer.
    // The function sleeps for 40 milliseconds, but the
    // timer times out after 30 milliseconds.
    #[test]
    #[serial]
    fn test_query_timer() {
        let timer = start_query_timer(30);
        let flag = query_stopped();
        assert_eq!(false, flag, "SUIRON_STOP_QUERY should be false.");
        let delay = Duration::from_millis(40);
        thread::sleep(delay); // Rust does sleep, Neil Young.
        cancel_timer(timer);
        let flag = query_stopped();
        assert_eq!(true, flag, "SUIRON_STOP_QUERY should be true.");
    } // test_query_timer()

} // test
