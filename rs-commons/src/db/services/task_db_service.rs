use crate::db::repos::task_repo::TasksDbRepo;

#[derive(Clone)]
pub struct TasksDbService {
    repo: TasksDbRepo
}

impl TasksDbService {
    pub fn new() -> Self {
        TasksDbService {
            repo: TasksDbRepo::new()
        }
    }
}