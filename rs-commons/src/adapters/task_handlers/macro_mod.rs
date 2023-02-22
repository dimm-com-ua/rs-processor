macro_rules! task_handler {
    ($struct_name: ident, $process_method: expr) => {
        use async_trait::async_trait;
        use crate::adapters::task_handlers::TaskHandlerTrait;
        use crate::adapters::models::worker::task_worker::TaskWorker;
        use crate::db::services::{App, DbServices};
        use crate::adapters::models::worker::task_variable::TaskVariable;
        use crate::adapters::models::common_error::ErrorDefinition;
        use deadpool_postgres::Transaction;
        use crate::adapters::models::worker::task_worker_result::TaskWorkerResult;

        struct $struct_name;

        #[async_trait]
        impl TaskHandlerTrait for $struct_name {
            async fn process(&self, task_worker: TaskWorker, dbs: &DbServices, app: &App, args: Option<Vec<TaskVariable>>, tr: &Transaction<'_>) -> Result<TaskWorkerResult, ErrorDefinition> {
                $process_method(task_worker, dbs, app, args, tr)
            }
        }
    }
}