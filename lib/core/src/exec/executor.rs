use slog::Logger;
use exec::{Execution};

pub struct Executor {
    pub logger: Logger
}

impl Executor {
    pub fn execute<T>(&self, execution: &Execution<T>, args: &Vec<String>) -> Result<T, String> {
        slog_debug!(self.logger, "Beginning execution");
        return execution.execute(args)
    }
}