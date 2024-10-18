use std::collections::HashMap;
use std::time::{Instant, Duration};

/// A struct for timing and logging time durations.
///
/// `Timer` uses a `HashMap` to store multiple named timers, each associated with a label.
pub struct Timer {
    /// HashMap storing timers, where keys are labels and values are start times.
    timers: HashMap<String, Instant>,
}

impl Timer {
    /// Creates a new `Timer` instance.
    ///
    /// # Returns
    ///
    /// Returns a new `Timer` instance with an empty timer HashMap.
    pub fn new() -> Self {
        Timer {
            timers: HashMap::new(),
        }
    }

    /// Starts a new timer.
    ///
    /// # Arguments
    ///
    /// * `label` - The label for the timer.
    pub fn time(&mut self, label: &str) {
        self.timers.insert(label.to_string(), Instant::now());
    }

    /// Logs and prints the current time of a timer without stopping it.
    ///
    /// # Arguments
    ///
    /// * `label` - The label of the timer.
    /// * `silent` - Whether to suppress printing the message.
    ///
    /// # Returns
    ///
    /// Returns the number of milliseconds the timer has been running, or 0.0 if the timer doesn't exist.
    pub fn time_log(&self, label: &str, silent: bool) -> f64 {
        if let Some(start_time) = self.timers.get(label) {
            let duration = start_time.elapsed();
            let ms = Self::duration_to_ms(duration);
            if !silent {
                println!("{}: {:.3}ms", label, ms);
            }
            ms
        } else {
            eprintln!("Timer '{}' does not exist", label);
            0.0
        }
    }

    /// Ends a timer and prints its runtime.
    ///
    /// # Arguments
    ///
    /// * `label` - The label of the timer.
    /// * `silent` - Whether to suppress printing the message.
    ///
    /// # Returns
    ///
    /// Returns the number of milliseconds the timer has been running, or 0.0 if the timer doesn't exist.
    pub fn time_end(&mut self, label: &str, silent: bool) -> f64 {
        if let Some(start_time) = self.timers.remove(label) {
            let duration = start_time.elapsed();
            let ms = Self::duration_to_ms(duration);
            if !silent {
                println!("{}: {:.3}ms", label, ms);
            }
            ms
        } else {
            eprintln!("Timer '{}' does not exist", label);
            0.0
        }
    }

    /// Converts a Duration to milliseconds.
    ///
    /// # Arguments
    ///
    /// * `duration` - The Duration to convert.
    ///
    /// # Returns
    ///
    /// Returns the converted milliseconds as a floating-point number.
    fn duration_to_ms(duration: Duration) -> f64 {
        duration.as_secs() as f64 * 1000.0 + duration.subsec_nanos() as f64 / 1_000_000.0
    }
}

/// Implements the `Default` trait for `Timer`.
impl Default for Timer {
    /// Creates a default `Timer` instance.
    ///
    /// # Returns
    ///
    /// Returns a new `Timer` instance.
    fn default() -> Self {
        Self::new()
    }
}

/// Test module
#[cfg(test)]
mod tests {
    use super::*;
    use std::thread::sleep;
    use std::time::Duration;

    /// Tests Timer::new() and Timer::default()
    #[test]
    fn test_timer_new() {
        let timer = Timer::default();
        assert!(timer.timers.is_empty());
    }

    /// Tests Timer::time() method
    #[test]
    fn test_timer_time() {
        let mut timer = Timer::new();
        timer.time("test");
        assert!(timer.timers.contains_key("test"));
    }

    /// Tests Timer::time_log() method
    #[test]
    fn test_timer_time_log() {
        let mut timer = Timer::new();
        timer.time("test_time_log");
        sleep(Duration::from_millis(10));
        let ms = timer.time_log("test_time_log", false);
        assert!(ms > 10.0 && ms < 15.0);
    }

    /// Tests Timer::time_end() method
    #[test]
    fn test_timer_time_end() {
        let mut timer = Timer::new();
        timer.time("test_time_end");
        sleep(Duration::from_millis(10));
        timer.time_end("test_time_end", false);
        assert!(!timer.timers.contains_key("test"));
    }

    /// Tests Timer::duration_to_ms() method
    #[test]
    fn test_duration_to_ms() {
        let duration = Duration::from_millis(1234);
        assert_eq!(Timer::duration_to_ms(duration), 1234.0);
    }
}
