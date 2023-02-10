pub mod flow_repo;
pub mod process_repo;
pub mod task_repo;

#[derive(Debug)]
pub enum DbRepoError {
    NotFound,
    QueryError(String)
}