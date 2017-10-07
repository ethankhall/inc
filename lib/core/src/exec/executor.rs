use slog::Logger;
use exec::{Execution};

pub struct Executor {
    logger: Logger
}

impl Executor {

    pub fn new(logger: &Logger) -> Self {
        Executor { logger: logger.new(o!()) }
    }

    pub fn execute<T>(&self, execution: &Execution<T>, args: &Vec<String>) -> Result<T, String> {
        slog_debug!(self.logger, "Beginning execution");
        return execution.execute(args)
    }
}