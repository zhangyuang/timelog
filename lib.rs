use std::collections::HashMap;
use std::time::{Instant, Duration};
use std::sync::{Arc, Mutex};

pub struct Timer {
    timers: Arc<Mutex<HashMap<String, Instant>>>,
}

impl Timer {
    pub fn new() -> Self {
        Timer {
            timers: Arc::new(Mutex::new(HashMap::new())),
        }
    }

    pub fn time(&self, label: &str) {
        let mut timers = self.timers.lock().unwrap();
        timers.insert(label.to_string(), Instant::now());
    }

    pub fn time_log(&self, label: &str, message: Option<&str>) {
        let timers = self.timers.lock().unwrap();
        if let Some(start_time) = timers.get(label) {
            let duration = start_time.elapsed();
            match message {
                Some(msg) => println!("{}: {:.3}ms - {}", label, Self::duration_to_ms(duration), msg),
                None => println!("{}: {:.3}ms", label, Self::duration_to_ms(duration)),
            }
        } else {
            eprintln!("Timer '{}' does not exist", label);
        }
    }

    pub fn time_end(&self, label: &str) {
        let mut timers = self.timers.lock().unwrap();
        if let Some(start_time) = timers.remove(label) {
            let duration = start_time.elapsed();
            println!("{}: {:.3}ms", label, Self::duration_to_ms(duration));
        } else {
            eprintln!("Timer '{}' does not exist", label);
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
