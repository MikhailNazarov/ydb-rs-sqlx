use std::borrow::Cow;
use std::time::Instant;
use sqlx_core::connection::Connection;
use sqlx_core::executor::Executor;
use sqlx_core::query_as::query_as;
use futures::future::{ok, BoxFuture};
use sqlx_core::migrate::{AppliedMigration, Migrate, MigrateDatabase, MigrateError, Migration};
use sqlx_core::query::query;
use ydb::Bytes;
use crate::connection::YdbConnection;
use crate::database::Ydb;

impl Migrate for YdbConnection {
    fn ensure_migrations_table(&mut self) -> BoxFuture<'_, Result<(), MigrateError>> {
        Box::pin(async move {
            
            query(r#"
                CREATE TABLE _sqlx_migrations (
                    version Int64 NOT NULL,
                    description Utf8 NOT NULL,
                    checksum String NOT NULL,
                    installed_on Timestamp NOT NULL,
                    success Bool NOT NULL,
                    execution_time Int64 NOT NULL,
                    PRIMARY KEY (version)
                );
            "#).execute(self.schema()).await?;

            Ok(())
        })
    }

    fn dirty_version(&mut self) -> BoxFuture<'_, Result<Option<i64>,MigrateError>> {
        Box::pin(async move {
            let row: Option<(i64,)> = query_as(
                "SELECT version FROM _sqlx_migrations WHERE success = false ORDER BY version LIMIT 1",
            )
            .fetch_optional(self)
            .await?;

            Ok(row.map(|r| r.0))
        })  
    }

    fn list_applied_migrations(
        &mut self,
    ) -> BoxFuture<'_, Result<Vec<sqlx_core::migrate::AppliedMigration>,MigrateError>> {
        Box::pin(async move {
            let rows: Vec<(i64, Bytes)> =
                query_as("SELECT version, checksum FROM _sqlx_migrations ORDER BY version")
                    .fetch_all(self)
                    .await?;

            let migrations = rows
                .into_iter()
                .map(|(version, checksum)| AppliedMigration {
                    version,
                    checksum: Cow::Owned(Vec::from( checksum)) ,
                })
                .collect();

            Ok(migrations)
        })
    }

    fn lock(&mut self) -> BoxFuture<'_, Result<(),MigrateError>> {
        Box::pin(ok(()))
    }

    fn unlock(&mut self) -> BoxFuture<'_, Result<(),MigrateError>> {
        Box::pin(ok(()))
    }

    fn apply<'e: 'm, 'm>(
        &'e mut self,
        migration: &'m Migration,
    ) -> BoxFuture<'m, Result<std::time::Duration,MigrateError>> {

        Box::pin(async move {

            let start = Instant::now();
            
            let mut tx = self.begin().await?;
            

            // Use a single transaction for the actual migration script and the essential bookeeping so we never
            // execute migrations twice. See https://github.com/launchbadge/sqlx/issues/1966.
            // The `execution_time` however can only be measured for the whole transaction. This value _only_ exists for
            // data lineage and debugging reasons, so it is not super important if it is lost. So we initialize it to -1
            // and update it once the actual transaction completed.
            let _ = tx.schema().execute(&*migration.sql).await?;

            let _ = query::<Ydb>(
                r#"
                $installed_on = CurrentUtcTimestamp(0);

                UPSERT INTO _sqlx_migrations ( version, description, success, checksum, execution_time, installed_on )
                VALUES ( $arg_1, $arg_2, TRUE, $arg_3, -1, $installed_on )
                "#,
            )
            .bind(migration.version)
            .bind(migration.description.clone().into_owned())
            .bind(Bytes::from(Vec::from(&*migration.checksum)))
            .execute(&mut *tx)
            .await?;

            tx.commit().await?;
        
        let elapsed = start.elapsed();
            // Update `elapsed_time`.
            // NOTE: The process may disconnect/die at this point, so the elapsed time value might be lost. We accept
            //       this small risk since this value is not super important.
        
            
            let nanos = elapsed.as_nanos() as i64;
            
            
            let _ = query(
                r#"
                
                    UPDATE _sqlx_migrations
                    SET execution_time = $arg_1,
                        description = description,
                        checksum = checksum,
                        installed_on = installed_on,
                        success = success
                    WHERE version = $arg_2
                "#,
            )
            .bind(nanos)
            .bind(migration.version)
            .execute(self)
            .await?;
            

            Ok(elapsed)
        })
       
    }

    fn revert<'e: 'm, 'm>(
        &'e mut self,
        migration: &'m Migration,
    ) -> BoxFuture<'m, Result<std::time::Duration,MigrateError>> {
        Box::pin(async move {
            // Use a single transaction for the actual migration script and the essential bookeeping so we never
            // execute migrations twice. See https://github.com/launchbadge/sqlx/issues/1966.
            let mut tx = self.begin().await?;
            let start = Instant::now();

            let _ = tx.execute(&*migration.sql).await?;

            let _ = query(r#"DELETE FROM _sqlx_migrations WHERE version = $arg_1"#)
                .bind(migration.version)
                .execute(&mut *tx)
                .await?;

            tx.commit().await?;

            let elapsed = start.elapsed();

            Ok(elapsed)
        })
    }
}


impl MigrateDatabase for Ydb{
    fn create_database(_url: &str) -> BoxFuture<'_, Result<(), sqlx_core::Error>> {
        todo!()
    }

    fn database_exists(_url: &str) -> BoxFuture<'_, Result<bool, sqlx_core::Error>> {
        todo!()
    }

    fn drop_database(_url: &str) -> BoxFuture<'_, Result<(), sqlx_core::Error>> {
        todo!()
    }
}