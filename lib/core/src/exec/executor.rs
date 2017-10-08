use exec::Execution;

pub struct Executor {
}

impl Executor {
    pub fn new() -> Self {
        Executor { }
    }

    pub fn execute<T>(&self, execution: &Execution<T>, args: &Vec<String>) -> Result<T, String> {
        debug!("Beginning execution");
        return execution.execute(args);
    }
}
