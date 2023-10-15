pub mod api;
pub mod config;
pub mod job;

use actix_web::{
    body::BoxBody, http::StatusCode, HttpResponse, HttpResponseBuilder, ResponseError,
};
use clap::Parser;
use derive_more::{Display, Error};
use lazy_static::lazy_static;
use serde::{Deserialize, Serialize};
use std::{fs::File, io::Read, sync::Arc, sync::Mutex};

use config::Config;
use job::{Job, JobInfo};

// 用作数据库的替代
lazy_static! {
    pub static ref JOBDATA: Arc<Mutex<JobData>> = Arc::default();
}

#[derive(Debug, Display, Error)]
#[allow(non_camel_case_types)]
pub enum AppError {
    ERR_INVALID_ARGUMENT,
    ERR_INVALID_STATE,
    ERR_NOT_FOUND,
    ERR_RATE_LIMIT,
    ERR_EXTERNAL,
    ERR_INTERNAL,
}

impl AppError {
    fn to_response(&self) -> ErrorResponse {
        match self {
            AppError::ERR_INVALID_ARGUMENT => ErrorResponse::new(1, &self.to_string()),
            AppError::ERR_INVALID_STATE => ErrorResponse::new(2, &self.to_string()),
            AppError::ERR_NOT_FOUND => ErrorResponse::new(3, &self.to_string()),
            AppError::ERR_RATE_LIMIT => ErrorResponse::new(4, &self.to_string()),
            AppError::ERR_EXTERNAL => ErrorResponse::new(5, &self.to_string()),
            AppError::ERR_INTERNAL => ErrorResponse::new(6, &self.to_string()),
        }
    }
}

impl ResponseError for AppError {
    // map the enum to HttpResponse StatusCode
    fn status_code(&self) -> StatusCode {
        match self {
            AppError::ERR_INVALID_ARGUMENT => StatusCode::BAD_REQUEST,
            AppError::ERR_INVALID_STATE => StatusCode::BAD_REQUEST,
            AppError::ERR_NOT_FOUND => StatusCode::NOT_FOUND,
            AppError::ERR_RATE_LIMIT => StatusCode::BAD_REQUEST,
            AppError::ERR_EXTERNAL => StatusCode::INTERNAL_SERVER_ERROR,
            AppError::ERR_INTERNAL => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }
    fn error_response(&self) -> HttpResponse<BoxBody> {
        HttpResponseBuilder::new(self.status_code()).json(self.to_response())
    }
}

pub struct JobData {
    job_list: Vec<Job>,
    total_jobs: u32,
}

impl JobData {
    // add job to the job list
    // first check if it is valid
    // if valid add it to the list and return the run response
    // otherwise return error
    pub fn add_job(&mut self, info: &JobInfo, config: &Config) -> Result<Response, AppError> {
        let id = self.total_jobs;

        let mut job = Job::new(id, info);

        // run the job and get the response
        job.run(config)?;
        let res = job.response();

        self.total_jobs += 1;
        self.job_list.push(job);
        Ok(res)
    }

    // find the job
    pub fn find_job_mut(&mut self, job_id: u32) -> Result<&mut Job, AppError> {
        let response = self.job_list.iter_mut().find(|x| x.job_id == job_id);
        response.map_or(Err(AppError::ERR_NOT_FOUND), Ok)
    }

    // find the job
    pub fn find_job(&self, job_id: u32) -> Result<&Job, AppError> {
        let response = self.job_list.iter().find(|x| x.job_id == job_id);
        response.map_or(Err(AppError::ERR_NOT_FOUND), Ok)
    }

    // get the job response
    pub fn get_job_response(&self, job_id: u32) -> Result<Response, AppError> {
        let response = self.find_job(job_id);
        response.map_or(Err(AppError::ERR_NOT_FOUND), |x| Ok(x.response()))
    }
}

impl Default for JobData {
    fn default() -> Self {
        let job_list: Vec<Job> = Vec::new();
        let total_jobs = 0;
        Self {
            job_list,
            total_jobs,
        }
    }
}

#[derive(Debug, Serialize, Clone)]
pub struct CaseResult {
    pub result: RunResult,
    time: u32,
    memory: u32,
}

impl CaseResult {
    pub fn new(result: RunResult) -> Self {
        Self {
            result,
            time: 0,
            memory: 0,
        }
    }
}

// error_response
#[derive(Debug, Serialize)]
pub struct ErrorResponse {
    code: u32,
    reason: String,
}

impl ErrorResponse {
    pub fn new(code: u32, reason: &str) -> Self {
        Self {
            code,
            reason: reason.to_string(),
        }
    }
}

// job state
#[derive(Debug, PartialEq, Serialize, Deserialize, Clone, Copy)]
pub enum State {
    Queueing,
    Running,
    Finished,
    Canceled,
}

// job result
#[derive(Debug, Serialize, Deserialize, PartialEq, Clone, Copy)]
pub enum RunResult {
    Waiting,
    Running,
    Accepted,
    #[serde(rename(serialize = "Compilation Error", deserialize = "Compilation Error"))]
    CompilationError,
    #[serde(rename(serialize = "Compilation Success", deserialize = "Compilation Success"))]
    CompilationSuccess,
    #[serde(rename(serialize = "Wrong Answer", deserialize = "Wrong Answer"))]
    WrongAnswer,
    #[serde(rename(serialize = "Runtime Error", deserialize = "Runtime Error"))]
    RuntimeError,
    #[serde(rename(serialize = "Time Limit Exceeded", deserialize = "Time Limit Exceeded"))]
    TimeLimitExceeded,
    #[serde(rename(
        serialize = "Memory Limit Exceeded",
        deserialize = "Memory Limit Exceeded"
    ))]
    MemoryLimitExceeded,
    #[serde(rename(serialize = "System Error", deserialize = "System Error"))]
    SystemError,
    #[serde(rename(serialize = "SPJ Error", deserialize = "SPJ Error"))]
    SpjError,
    Skipped,
}

// the job response
#[derive(Debug, Serialize)]
pub struct Response {
    pub id: u32,
    pub created_time: String,
    pub updated_time: String,
    pub submission: JobInfo,
    pub state: State,
    pub result: RunResult,
    pub cases: Vec<CaseResult>,
    pub flag: String,
}

#[derive(Parser)]
#[command(name = "oj")]
#[command(author = "13m0n4de")]
#[command(version = "0.1")]
#[command(about = "Attention is all you need", long_about = None)]
pub struct Cli {
    /// config json
    #[arg(short, long)]
    pub config: String,
    #[arg(short = 'f', long = "flush-data")]
    pub flush_data: bool,
}
impl Cli {
    pub fn init_config(&self) -> std::io::Result<Config> {
        let mut file = File::open(&self.config)?;
        let mut contents = String::new();
        file.read_to_string(&mut contents)?;
        let config: Config = serde_json::from_str(&contents)?;
        Ok(config)
    }
}
