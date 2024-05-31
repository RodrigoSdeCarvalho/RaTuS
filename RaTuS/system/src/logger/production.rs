use crate::logger::{ILogger, LoggerEssentials};

pub(super) struct ProductionLogger;

impl ILogger for ProductionLogger {}

impl LoggerEssentials for ProductionLogger {
    fn open() -> Self {
        todo!("Use Debug profile on .env file to use DebugLogger instead of ProductionLogger")
    }

    fn save(&self, _message: &String) {
        todo!("Use Debug profile on .env file to use DebugLogger instead of ProductionLogger")
    }
}
