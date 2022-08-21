//! api.rs --- API typedefs

//! - 'op' module contains client <--> server messages
//! - 'c2' module contains server <--> agent messages
//!
//! The two additional traits
pub mod c2;
pub mod op;

use c2::{Ack, AgentRegistered, Job, JobPayload, JobResult, RegisterAgent, Syn, UpdateJobResult};
use op::{Message, OpCode, Val};

use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub enum Response<T: Serialize> {
  Data(T),
  Error(String),
}

impl<T: Serialize> Response<T> {
  pub fn ok(data: T) -> Response<T> {
    return Response::Data(data);
  }
  pub fn err<E: std::error::Error>(err: E) -> Response<T> {
    return Response::Error(err.to_string());
  }
}

pub trait AgentExt {
  /// return a Syn.
  fn syn() -> Syn {
    Syn
  }
  /// collect seen packet #s and return an Ack
  fn ack(acks: &[u16]) -> Ack {
    Ack(acks.to_vec())
  }
  /// build a RegisterAgent response
  fn register_agent(&self) -> RegisterAgent;
  /// get the payload from a job
  fn get_payload(&self, job: Job) -> JobPayload;
  /// return an UpdateJobResult
  fn update_job_result(&self) -> UpdateJobResult;
}

pub trait ServerExt {
  fn syn() -> Syn {
    Syn
  }
  fn ack(acks: &[u16]) -> Ack {
    Ack(acks.to_vec())
  }
  /// turn a payload into a Job
  fn job(&self, payload: JobPayload) -> Job;
  /// accept an agent registration
  fn get_registered(&self, agent: RegisterAgent) -> AgentRegistered;
  /// push an update to a borrowed JobResult
  fn push_job_result(&self, result: &mut JobResult, update: UpdateJobResult);
}

pub trait ClientExt {
  /// generate a Message.
  fn message(&self, top: OpCode, val: Val) -> Message;
}
