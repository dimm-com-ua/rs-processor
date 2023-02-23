use crate::adapters::db::client::PgClient;
use std::ops::DerefMut;

mod embedded {
    use refinery::embed_migrations;
    embed_migrations!("./migrations");
}

pub async fn run_migrations(db: &PgClient) {
    let mut conn = db.get_connection().await;
    let client = conn.deref_mut().deref_mut();
    match embedded::migrations::runner().run_async(client).await {
        Ok(_) => log::info!("Migrations successfully applied!"),
        Err(err) => panic!("No migrations took place. The error is: {:?}", err),
    }
}
