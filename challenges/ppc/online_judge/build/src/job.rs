use chrono::prelude::*;
use serde::{Deserialize, Serialize};
use std::{
    io,
    env,
    fs::{self, write, File, OpenOptions},
    path::Path,
    process::{Command, Stdio},
    time::Duration,
};
use std::string::ToString;
use wait_timeout::ChildExt;

use crate::config::Language;
use crate::{config, AppError, CaseResult, Response, RunResult, State};

const TEMP_DIR_PREFIX: &str = "/tmp";

// the struct represent the json content from the post job http request
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct JobInfo {
    pub source_code: String,
    pub language: String,
    pub problem_id: usize,
}

// use this struct to run a job and get a response
pub struct Job {
    pub job_id: u32,
    pub info: JobInfo,
    pub created_time: DateTime<Utc>,
    pub updated_time: DateTime<Utc>,
    pub state: State,
    pub result: RunResult,
    pub case_res: Vec<CaseResult>,
    pub flag: String,
}

impl Job {
    pub fn new(job_id: u32, info: &JobInfo) -> Self {
        Self {
            job_id,
            info: info.clone(),
            created_time: DateTime::default(),
            updated_time: DateTime::default(),
            state: State::Queueing,
            result: RunResult::Waiting,
            case_res: Vec::new(),
            flag: String::new(),
        }
    }

    // 初始化目录，重建任务对应缓存文件夹
    fn init(&mut self) -> Result<(), io::Error> {
        let path = format!("{}/job_{}", TEMP_DIR_PREFIX, self.job_id);
        if Path::new(&path).is_dir() {
            fs::remove_dir_all(&path)?;
            fs::create_dir(&path)?;
        } else {
            fs::create_dir(&path)?;
        }

        self.created_time = Utc::now();
        self.updated_time = Utc::now();
        self.state = State::Queueing;
        self.result = RunResult::Waiting;
        self.case_res.clear();

        Ok(())
    }

    fn compile_source_code(&mut self, language: &Language) -> Result<RunResult, io::Error> {
        write(self.path(&language.file_name), &self.info.source_code)?;

        let mut process = Command::new(&language.command[0])
            .args(&language.command[1..])
            .spawn()?;
        let exitstatus = process.wait()?;
        if exitstatus.success() {
            Ok(RunResult::CompilationSuccess)
        } else {
            Ok(RunResult::CompilationError)
        }
    }

    // 运行单个测试用例
    fn run_one_case(&mut self, case: &config::Case) -> Result<RunResult, io::Error> {
        // 打开输入输出文件
        let input = File::open(&case.input_file)?;
        let output = OpenOptions::new()
            .read(true)
            .write(true)
            .truncate(true)
            .create(true)
            .open(self.path("output"))?;
        // 创建进程
        let mut process = Command::new(self.path("a.out"))
            .stdin(input)
            .stdout(Stdio::from(output))
            .spawn()?;
        // 超时时间内等待结果
        let res = process.wait_timeout(Duration::from_micros(case.time_limit as u64))?;
        match res {
            Some(exit) => {
                if exit.success() {
                    let ans = fs::read_to_string(&case.answer_file)?;
                    let out = fs::read_to_string(self.path("output"))?;
                    if ans == out {
                        Ok(RunResult::Accepted)
                    } else {
                        Ok(RunResult::WrongAnswer)
                    }
                } else {
                    Ok(RunResult::RuntimeError)
                }
            }
            None => {
                process.kill()?;
                Ok(RunResult::TimeLimitExceeded)
            }
        }
    }

    pub fn run(&mut self, config: &config::Config) -> Result<Response, AppError> {
        // 从配置文件获取指定 id 的问题
        let problem = config
            .problems
            .iter()
            .find(|prob| prob.id == self.info.problem_id)
            .ok_or(AppError::ERR_NOT_FOUND)?;

        // 从配置文件获取指定语言
        let mut language = config
            .languages
            .iter()
            .find(|lang| lang.name == self.info.language)
            .ok_or(AppError::ERR_NOT_FOUND)?
            .clone();
        language.replace("%OUTPUT%", &self.path("a.out"));
        language.replace("%INPUT%", &self.path(&language.file_name));

        // 初始化
        match self.init() {
            Err(err) => {
                log::info!(target: "Job::init", "System io error {}", err);
                self.state = State::Finished;
                self.result = RunResult::SystemError;
                return Ok(self.response());
            }
            Ok(_) => {
                self.result = RunResult::Running;
                self.state = State::Running;
            }
        }

        // 编译
        match self.compile_source_code(&language) {
            Err(err) => {
                log::info!(target: "Job::compile_source_code", "System io error {}", err);
                self.state = State::Finished;
                self.result = RunResult::SystemError;
                self.case_res.push(CaseResult::new(RunResult::SystemError));
                return Ok(self.response());
            }
            Ok(res) => match res {
                RunResult::CompilationSuccess => {
                    self.result = res;
                    self.case_res
                        .push(CaseResult::new(RunResult::CompilationSuccess));
                }
                RunResult::CompilationError => {
                    self.state = State::Finished;
                    self.result = RunResult::CompilationError;
                    self.case_res
                        .push(CaseResult::new(RunResult::CompilationError));
                    return Ok(self.response());
                }
                _ => unreachable!(),
            },
        }

        // 运行测试用例
        for case in problem.cases.iter() {
            match self.run_one_case(case) {
                Ok(res) if res == RunResult::Accepted => {
                    self.result = res;
                    self.case_res.push(CaseResult::new(res));
                }
                Ok(res) => {
                    self.state = State::Finished;
                    self.result = res;
                    self.case_res.push(CaseResult::new(res));
                    return Ok(self.response());
                }
                Err(err) => {
                    log::info!(target: "Job::run_one_case", "System io error {}", err);
                    self.state = State::Finished;
                    self.result = RunResult::SystemError;
                    self.case_res.push(CaseResult::new(RunResult::SystemError));
                    return Ok(self.response());
                }
            }
        }

        // 返回部分 FLAG
        let flag: String = env::var("GZCTF_FLAG").unwrap_or("flag{test_flag}".to_string());
        let flag_length = flag.len();

        let problem_count = config.problems.len();
        let flag_chunk_size = flag_length / problem_count;
        let start = problem.id * flag_chunk_size;
        let mut end = start + flag_chunk_size;
        if problem.id == problem_count - 1 {
            end += flag_length % problem_count;
        }
        self.flag = flag[start..end].to_string();

        self.state = State::Finished;
        Ok(self.response())
    }

    pub fn response(&self) -> Response {
        Response {
            id: self.job_id,
            created_time: self
                .created_time
                .format("%Y-%m-%dT%H:%M:%S%.3fZ")
                .to_string(),
            updated_time: self
                .updated_time
                .format("%Y-%m-%dT%H:%M:%S%.3fZ")
                .to_string(),
            submission: self.info.clone(),
            state: self.state,
            result: self.result,
            cases: self.case_res.clone(),
            flag: self.flag.clone(),
        }
    }

    fn path(&self, filename: &str) -> String {
        format!("{}/job_{}/{}", TEMP_DIR_PREFIX, &self.job_id, filename)
    }
}
