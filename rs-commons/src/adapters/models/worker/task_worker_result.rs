use chrono::{DateTime, Utc};
use crate::adapters::models::worker::task_variable::TaskVariable;

pub enum WorkerResult { Done, Fail, Finishing }

pub struct TaskWorkerResult {
    pub result: WorkerResult,
    pub wait_until: Option<DateTime<Utc>>,
    pub out_args: Vec<TaskVariable>
}

impl TaskWorkerResult {
    pub fn ok() -> Self {
        TaskWorkerResult {
            result: WorkerResult::Done,
            wait_until: None,
            out_args: vec![],
        }
    }

    pub fn ok_with_args(args: Vec<TaskVariable>) -> Self {
        TaskWorkerResult {
            result: WorkerResult::Done,
            wait_until: None,
            out_args: args,
        }
    }

    pub fn finish() -> Self {
        TaskWorkerResult {
            result: WorkerResult::Finishing,
            wait_until: None,
            out_args: vec![],
        }
    }
}