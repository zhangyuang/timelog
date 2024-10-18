use std::collections::HashMap;
use std::time::{Instant, Duration};

pub struct Timer {
    timers: HashMap<String, Instant>,
}

impl Timer {
    pub fn new() -> Self {
        Timer {
            timers: HashMap::new(),
        }
    }

    pub fn time(&mut self, label: &str) {
        self.timers.insert(label.to_string(), Instant::now());
    }

    pub fn time_log(&self, label: &str, message: Option<&str>) -> f64 {
        if let Some(start_time) = self.timers.get(label) {
            let duration = start_time.elapsed();
            let ms = Self::duration_to_ms(duration);
            match message {
                Some(msg) => println!("{}: {:.3}ms - {}", label, ms, msg),
                None => println!("{}: {:.3}ms", label, ms),
            }
            ms
        } else {
            eprintln!("Timer '{}' does not exist", label);
            0.0
        }
    }

    pub fn time_end(&mut self, label: &str) -> f64 {
        if let Some(start_time) = self.timers.remove(label) {
            let duration = start_time.elapsed();
            let ms = Self::duration_to_ms(duration);
            println!("{}: {:.3}ms", label, ms);
            ms
        } else {
            eprintln!("Timer '{}' does not exist", label);
            0.0
        }
    }

    fn duration_to_ms(duration: Duration) -> f64 {
        duration.as_secs() as f64 * 1000.0 + duration.subsec_nanos() as f64 / 1_000_000.0
    }
}

impl Default for Timer {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::thread::sleep;
    use std::time::Duration;

    #[test]
    fn test_timer_new() {
        let timer = Timer::default();
        assert!(timer.timers.is_empty());
    }

    #[test]
    fn test_timer_time() {
        let mut timer = Timer::new();
        timer.time("test");
        assert!(timer.timers.contains_key("test"));
    }

    #[test]
    fn test_timer_time_log() {
        let mut timer = Timer::new();
        timer.time("test_time_log");
        sleep(Duration::from_millis(10));
        let ms=  timer.time_log("test_time_log", Some("测试消息"));
        assert!(ms > 10.0 && ms < 15.0);
    }

    #[test]
    fn test_timer_time_end() {
        let mut timer = Timer::new();
        timer.time("test_time_end");
        sleep(Duration::from_millis(10));
        timer.time_end("test_time_end");
        assert!(!timer.timers.contains_key("test"));
    }

    #[test]
    fn test_duration_to_ms() {
        let duration = Duration::from_millis(1234);
        assert_eq!(Timer::duration_to_ms(duration), 1234.0);
    }
}

