task_handler!(StartingHandler, |_task_worker: TaskWorker, _dbs: &DbServices, _app: &App, _args: Option<Vec<TaskVariable>>, _tr: &Transaction<'_>| -> Result<TaskWorkerResult, ErrorDefinition> {
    Ok(TaskWorkerResult::finish())
});
