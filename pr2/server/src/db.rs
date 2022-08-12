use crate::Error;
use sqlx::postgres::*;
use std::net::SocketAddr;
use uuid::Uuid;
use chrono::Utc;
pub mod types;

#[derive(Clone, Debug)]
pub struct Pool {
  pub writer: sqlx::Pool<Postgres>,
  pub reader: sqlx::Pool<Postgres>,
}

impl Pool {
  pub async fn new(database_url: SocketAddr) -> Result<Pool, Error> {
  let conn_opts = PgConnectOptions::new()
    .host(database_url.ip().to_string().as_str())
    .port(database_url.port())
    .database("pr2.db");

  let writer_pool = PgPoolOptions::new()
    .max_connections(1)
    .connect_with(conn_opts.clone())
    .await?;

  let reader_pool = PgPoolOptions::new()
    .connect_with(conn_opts)
    .await?;

    Ok(
      Pool {
	writer: writer_pool,
	reader: reader_pool,
      }
    )
  }
}

pub async fn prepare_pg_pool(
    database_url: SocketAddr,
) -> Result<Pool, Error> {
  let pool = Pool::new(database_url).await?;
  create_extensions(&pool).await?;
  create_tables(&pool).await?;
  Ok(pool)
}

async fn create_extensions(pool: &Pool) -> Result<(), Error> {
  // better uuids
  sqlx::query("create extension if not exists \"uuid-ossp\";")
    .execute(&pool.writer)
    .await?;

  // insert updated_at and created_at columns select
  // trigger_updated_at('<table name>');
  sqlx::query("
create or replace function set_updated_at()
    returns trigger as
$$
begin
    NEW.updated_at = now();
    return NEW;
end;
$$ language plpgsql;
")
.execute(&pool.writer)
.await?;

  sqlx::query("
create or replace function trigger_updated_at(tablename regclass)
    returns void as
$$
begin
    execute format('CREATE TRIGGER set_updated_at
        BEFORE UPDATE
        ON %s
        FOR EACH ROW
        WHEN (OLD is distinct from NEW)
    EXECUTE FUNCTION set_updated_at();', tablename);
end;
$$ language plpgsql;")
.execute(&pool.writer)
.await?;
  // text collation that sorts text case-insensitively, useful for
  // `UNIQUE` indexes over things like usernames and emails, without
  // needing to remember to do case-conversion.
  sqlx::query("
create collation if not exists case_insensitive (provider = icu, locale = 'und-u-ks-level2', deterministic = false);
")
.execute(&pool.writer)
.await?;
Ok(())
}

async fn create_types(pool: &Pool) -> Result<(), Error> {
  sqlx::query("CREATE TYPE job_status AS ENUM ('SCHEDULED', 'PENDING', 'DONE', 'FAILED');")
    .execute(&pool.writer)
    .await?;

  sqlx::query("CREATE TYPE job_type AS ENUM ('TASK', 'EXIT');")
    .execute(&pool.writer)
    .await?;
  Ok(())
}

async fn create_tables(pool: &Pool) -> Result<(), Error> {
    sqlx::query(
        "
CREATE TABLE IF NOT EXISTS agent (
    id uuid PRIMARY KEY NOT NULL,
    public_prekey bytea NOT NULL,
    public_prekey_signature bytea NOT NULL,
    identity_public_key bytea NOT NULL,    
    created_at timestamptz NOT NULL,
    last_seen timestamptz NOT NULL
);
        ",
    )
    .execute(&pool.writer)
    .await?;

    sqlx::query(
        "
CREATE TABLE IF NOT EXISTS job (
    id uuid PRIMARY KEY NOT NULL,
    agent_id uuid NOT NULL,
    ephemeral_public_key bytea NOT NULL,
    nonce bytea NOT NULL,
    signature bytea NOT NULL,
    updated_at timestamptz,
    status job_status NOT NULL,
    type job_type NOT NULL,
    payload bytea NOT NULL
);
        ",
    )
    .execute(&pool.writer)
    .await?;

  sqlx::query("select trigger_updated_at('job');")
    .execute(&pool.writer)
    .await?;

  sqlx::query(
        "
CREATE TABLE IF NOT EXISTS job_result (
    result_id uuid PRIMARY KEY NOT NULL,
    job_id uuid NOT NULL,
    result text NOT NULL,
    created_at timestamptz NOT NULL,
    updated_at timestamptz
);")
    .execute(&pool.writer)
    .await?;

  sqlx::query("select trigger_updated_at('job_result');")
    .execute(&pool.writer)
    .await?;

    Ok(())
}

pub async fn drop_tables(pool: Pool) -> Result<(), Error> {
  sqlx::query("
select 'drop table if exists \"' || tablename || '\" cascade;' 
  from pg_tables
 where schemaname = 'public';"
  )
    .execute(&pool.writer)
    .await?;
  Ok(())
}

pub async fn insert_agent(agent: &types::Agent, pool: &Pool) -> Result<Uuid, Error> {
  let rec = sqlx::query!(r#"
insert into agent
values ($1, $2, $3, $4, $5, $6)
returning id;
"#,
			 agent.id,
			 agent.public_prekey,
			 agent.public_prekey_signature,
			 agent.identity_public_key,
			 agent.created_at,
			 agent.last_seen,
  )
    .fetch_one(&pool.writer)
    .await?;
  Ok(rec.id)
}

pub async fn list_agents(pool: &Pool) -> Result<Vec<types::Agent>, Error> {
  sqlx::query_as::<_, types::Agent>("select * from agent order by created_at")
    .fetch_all(&pool.reader)
    .await.map_err(|e| e.into())
}

pub async fn visit_agent_by_id(id: &Uuid, pool: &Pool) -> Result<(), Error> {
  sqlx::query!(r#"
update agent set last_seen = $1 where id = $2;
"#,
	       Utc::now(),
	       id
  ).fetch_one(&pool.writer)
    .await?;
  Ok(())
}
