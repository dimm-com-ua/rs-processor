use async_trait::async_trait;

use crate::adapters::task_handlers::TaskHandlerTrait;

pub struct StartingHandler {}
impl StartingHandler {
    pub fn _new() -> Self { StartingHandler{} }
}

#[async_trait]
impl TaskHandlerTrait for StartingHandler {

}