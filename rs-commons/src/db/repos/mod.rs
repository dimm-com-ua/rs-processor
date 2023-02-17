pub mod core_repo;
pub mod flow_repo;
pub mod process_repo;
pub mod task_repo;
pub mod worker_repo;

#[derive(Debug)]
pub enum DbRepoError {
    NotFound,
    QueryError(String)
}