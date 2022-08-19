use crate::crypto;

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::{convert::TryFrom, error::Error};
use uuid::Uuid;

pub trait ClientCoder {
  fn tx_ack() -> Ack;
  fn tx_syn() -> Syn;
  fn tx_register() -> RegisterAgent;
  fn rx_job() -> JobPayload;
  fn tx_job_result() -> UpdateJobResult;
}

pub trait ServerCoder {
  fn tx_ack() -> Ack;
  fn tx_job() -> Job;
  fn rx_register() -> AgentRegistered;
  fn rx_job_result() -> JobResult;
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Syn(u16);
#[derive(Debug, Serialize, Deserialize)]
pub struct Ack(u16);


#[derive(Debug, Serialize, Deserialize)]
pub enum Response<T: Serialize> {
  Data(T),
  Error(String),
}

impl<T: Serialize> Response<T> {
  pub fn ok(data: T) -> Response<T> {
    return Response::Data(data);
  }
  pub fn err<E: Error>(err: E) -> Response<T> {
    return Response::Error(err.to_string());
  }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct RegisterAgent {
  pub identity_public_key: [u8; crypto::ED25519_PUBLIC_KEY_SIZE],
  pub public_prekey: [u8; crypto::X25519_PUBLIC_KEY_SIZE],
  // we use Vec<u8> to avoid serde ownership errors
  pub public_prekey_signature: Vec<u8>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct AgentRegistered {
  pub id: Uuid,
}

pub enum JobType {
  Task,
  Exit,
}

impl TryFrom<&str> for JobType {
  type Error = ();

  fn try_from(s: &str) -> Result<Self, Self::Error> {
    let s = s.to_ascii_lowercase();

    if s == "task" {
      Ok(JobType::Task)
    } else if s == "exit" {
      Ok(JobType::Exit)
    } else {
      Err(())
    }
  }
}

impl From<JobType> for &str {
  fn from(job: JobType) -> Self {
    match job {
      JobType::Task => "task",
      JobType::Exit => "exit",
    }
  }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct CreateJob {
  pub id: Uuid,
  pub agent_id: Uuid,
  pub encrypted_job: Vec<u8>,
  pub ephemeral_public_key: [u8; crypto::X25519_PUBLIC_KEY_SIZE],
  pub nonce: [u8; crypto::XCHACHA20_POLY1305_NONCE_SIZE],
  pub signature: Vec<u8>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Job {
  pub id: Uuid,
  pub agent_id: Uuid,
  pub encrypted_job: Vec<u8>,
  pub ephemeral_public_key: [u8; crypto::X25519_PUBLIC_KEY_SIZE],
  pub nonce: [u8; crypto::XCHACHA20_POLY1305_NONCE_SIZE],
  pub signature: Vec<u8>,
  pub encrypted_result: Option<Vec<u8>>,
  pub result_ephemeral_public_key: Option<[u8; crypto::ED25519_PUBLIC_KEY_SIZE]>,
  pub result_nonce: Option<[u8; crypto::XCHACHA20_POLY1305_NONCE_SIZE]>,
  pub result_signature: Option<Vec<u8>>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct JobPayload {
  pub command: String,
  pub args: Vec<String>,
  pub result_ephemeral_public_key: [u8; crypto::X25519_PUBLIC_KEY_SIZE],
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct UpdateJobResult {
  pub job_id: Uuid,
  pub encrypted_job_result: Vec<u8>,
  pub ephemeral_public_key: [u8; crypto::X25519_PUBLIC_KEY_SIZE],
  pub nonce: [u8; crypto::XCHACHA20_POLY1305_NONCE_SIZE],
  pub signature: Vec<u8>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct JobResult {
  pub output: String,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct AgentJob {
  pub id: Uuid,
  pub encrypted_job: Vec<u8>,
  pub ephemeral_public_key: [u8; crypto::X25519_PUBLIC_KEY_SIZE],
  pub nonce: [u8; crypto::XCHACHA20_POLY1305_NONCE_SIZE],
  pub signature: Vec<u8>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Agent {
  pub id: Uuid,
  pub created_at: DateTime<Utc>,
  pub last_seen: DateTime<Utc>,
  pub identity_public_key: [u8; crypto::ED25519_PUBLIC_KEY_SIZE],
  pub public_prekey: [u8; crypto::X25519_PUBLIC_KEY_SIZE],
  pub public_prekey_signature: Vec<u8>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct AgentsList {
  pub agents: Vec<Agent>,
}
