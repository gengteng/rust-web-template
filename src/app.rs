//! Application context
//! 

use std::time::SystemTime;

use time::UtcOffset;

/// Package related constants
pub mod package {

    /// Package name
    pub const NAME: &'static str = env!("CARGO_PKG_NAME");

    /// Package version
    pub const VERSION: &'static str = env!("CARGO_PKG_VERSION");
}

/// Application context
pub struct Application {
    /// Package name
    pub name: &'static str,
    /// Package version
    pub version: &'static str,
    /// Process ID
    pub pid: u32,
    /// Timezone
    pub timezone: UtcOffset,
    /// Launch timestamp
    pub launch_time: SystemTime,
}

impl Default for Application {
    fn default() -> Self {
        Self {
            name: package::NAME,
            version: package::VERSION,
            pid: std::process::id(),
            timezone: UtcOffset::current_local_offset().expect("current local offset"),
            launch_time: SystemTime::now(),
        }
    }
}

impl Application {
    /// Print application information
    pub fn hi() -> Self {
        let app = Self::default();
        println!("Application: {} v{}", app.name, app.version);
        println!("Pid: {}, LocalOffset: {}", app.pid, app.timezone);
        app
    }
}

impl Drop for Application {
    fn drop(&mut self) {
        println!("Application has exited, Bye. (duration: {:?})", self.launch_time.elapsed().expect("elapsed"));
    }
}