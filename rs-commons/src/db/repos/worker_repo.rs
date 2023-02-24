use crate::adapters::db::client::PgClient;
use crate::adapters::models::common_error::ErrorDefinition;
use crate::db::models::task_worker_db::TaskWorkerDb;
use chrono::{DateTime, Utc};
use deadpool_postgres::tokio_postgres::Row;
use deadpool_postgres::Transaction;
use serde_json::json;
use crate::adapters::models::worker::task_worker::TaskWorker;

#[derive(Clone)]
pub struct WorkerRepo;

impl WorkerRepo {
    pub fn new() -> Self {
        WorkerRepo {}
    }

    pub async fn fetch_workers(
        &self,
        lock_key: uuid::Uuid,
        now: DateTime<Utc>,
        lock_by: DateTime<Utc>,
        count: i64,
        db: &PgClient,
    ) -> Result<Vec<TaskWorkerDb>, ErrorDefinition> {
        if let Err(err) = self.unlock(now, db).await {
            return Err(err);
        }
        let query = "update pc_task_worker set runner_key=$1, locked_by=$2
                            where id in (select ptw.id from pc_task_worker ptw where ptw.runner_key is null and ptw.locked_by is null and (ptw.run_after<=$3 or ptw.run_after is null) limit $4);";

        if let Err(err) = db
            .get_connection()
            .await
            .query(query, &[&lock_key, &lock_by, &now, &count])
            .await
        {
            return Err(ErrorDefinition::with_reason(
                "Couldn't lock workers".to_string(),
                json!({ "error": format!("{:?}", err) }),
            ));
        }

        let query = "select ptw from pc_task_worker ptw where runner_key=$1;";
        return match db.get_connection().await.query(query, &[&lock_key]).await {
            Ok(rows) => {
                let workers = rows
                    .iter()
                    .map(|x| {
                        let worker_db: TaskWorkerDb = x.get(0);
                        worker_db
                    })
                    .collect();
                Ok(workers)
            }
            Err(err) => Err(ErrorDefinition::with_reason(
                "Couldn't fetch locked workers".to_string(),
                json!({ "error": format!("{:?}", err) }),
            )),
        };
    }

    pub async fn get_worker(&self, uuid: uuid::Uuid, db: &PgClient) -> Result<TaskWorker, ErrorDefinition> {
        let query = "select ptw from pc_task_worker ptw where id=$1;";
        return match db.get_connection().await.query(query, &[&uuid]).await {
            Ok(rows) => {
                match rows.first() {
                    None => {
                        Err(ErrorDefinition::empty("Worker not found".to_string()))
                    }
                    Some(row) => {
                        let worker: TaskWorkerDb = row.get(0);
                        Ok(TaskWorker::from_db(&worker))
                    }
                }
            }
            Err(err) => Err(ErrorDefinition::with_reason(
                "Couldn't fetch locked workers".to_string(),
                json!({ "error": format!("{:?}", err) }),
            )),
        };
    }

    async fn unlock(&self, now: DateTime<Utc>, db: &PgClient) -> Result<(), ErrorDefinition> {
        let query = "update pc_task_worker set locked_by=null, runner_key=null where locked_by<$1 or locked_by is null;";
        if let Err(err) = db.get_connection().await.query(query, &[&now]).await {
            return Err(ErrorDefinition::with_reason(
                "Couldn't unlock workers".to_string(),
                json!({ "error": format!("{:?}", err) }),
            ));
        }
        Ok(())
    }

    pub async fn delete(
        &self,
        worker_id: uuid::Uuid,
        tr: &Transaction<'_>,
    ) -> Result<(), ErrorDefinition> {
        let query = "delete from pc_task_worker where id = $1;";
        if let Err(err) = tr.query(query, &[&worker_id]).await {
            return Err(ErrorDefinition::with_reason(
                "Couldn't delete worker".to_string(),
                json!({ "error": format!("{:?}", err) }),
            ));
        }
        Ok(())
    }
}
