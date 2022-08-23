use crate::{crypto, hex::encode, serialize, Error};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::convert::TryFrom;
use uuid::Uuid;

use prettytable::{row, Table};

#[derive(Debug, Serialize, Deserialize)]
pub struct Syn;

type A = u16;
type AV = Vec<A>;
#[derive(Debug, Serialize, Deserialize)]
pub struct Ack(pub AV);

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
pub struct JobsList(pub Vec<Job>);

impl From<Vec<Job>> for JobsList {
  fn from(v: Vec<Job>) -> Self {
    Self(v)
  }
}

impl std::fmt::Display for JobsList {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    let mut tbl = Table::new();
    tbl.set_format(*prettytable::format::consts::FORMAT_NO_LINESEP_WITH_TITLE);
    tbl.set_titles(row![
      "id",
      "agent_id",
      "encrypted_job",
      "ephemeral_public_key",
      "nonce",
      "signature",
      "encrypted_result",
      "result_ephemeral_public_key",
      "result_nonce",
      "result_signature"
    ]);
    for i in self.0.iter() {
      tbl.add_row(row![
        i.id,
        i.agent_id,
        encode(&i.encrypted_job),
        encode(i.ephemeral_public_key),
        encode(i.nonce),
        encode(&i.signature),
        if let Some(ref v) = i.encrypted_result {
          encode(v)
        } else {
          "nil".to_string()
        },
        if let Some(v) = i.result_ephemeral_public_key {
          encode(v)
        } else {
          "nil".to_string()
        },
        if let Some(v) = i.result_nonce {
          encode(v)
        } else {
          "nil".to_string()
        },
        if let Some(ref v) = i.result_signature {
          encode(v)
        } else {
          "nil".to_string()
        }
      ]);
    }
    f.write_str(&tbl.to_string())
  }
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
pub struct AgentsList(pub Vec<Agent>);

impl From<Vec<Agent>> for AgentsList {
  fn from(v: Vec<Agent>) -> Self {
    Self(v)
  }
}

impl std::fmt::Display for AgentsList {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    let mut tbl = Table::new();
    tbl.set_format(*prettytable::format::consts::FORMAT_NO_LINESEP_WITH_TITLE);
    tbl.set_titles(row![
      "id",
      "created_at",
      "last_seen",
      "identity_public_key",
      "public_prekey",
      "public_prekey_signature"
    ]);
    for i in self.0.iter() {
      let row = row![
        i.id.to_string(),
        i.created_at,
        i.last_seen,
        encode(i.identity_public_key).get(..16).unwrap_or(""),
        encode(i.public_prekey).get(..16).unwrap_or(""),
        encode(&i.public_prekey_signature).get(..16).unwrap_or(""),
      ];
      tbl.add_row(row);
    }
    f.write_str(&tbl.to_string())
  }
}

#[derive(Deserialize, Serialize, Clone)]
pub struct EncryptedData {
  pub public_key: crypto::PublicKey,
  pub data: crypto::Encrypted,
}

#[derive(Deserialize, Serialize, Clone)]
pub struct Packet {
  pub len: u16,
  pub chk: u32,
  pub val: EncryptedData,
}

impl Packet {
  pub fn len(&self) -> u16 {
    self.len
  }
  pub fn chk(&self) -> u32 {
    self.chk
  }
  pub fn to_bytes(&self) -> Result<Vec<u8>, Error> {
    let len_slice = u16::to_le_bytes(self.len());
    let chk_slice = u32::to_le_bytes(self.chk());
    // reserve space in buffer.
    let mut bytes = Vec::with_capacity(6 + self.len() as usize);

    bytes.extend_from_slice(&len_slice);
    bytes.extend_from_slice(&chk_slice);
    bytes.extend_from_slice(&serialize(&self.val)?);
    Ok(bytes)
  }
}
