use actix_web::{get, post, web, HttpResponse, Responder};

use crate::job::Job;
use crate::{config::Config, job::JobInfo, AppError, Response, JOBDATA};

#[post("/jobs")]
pub async fn post_jobs(
    info: web::Json<JobInfo>,
    config: web::Data<Config>,
) -> Result<HttpResponse, AppError> {
    let info = info.into_inner();
    let job_data = JOBDATA.clone();
    let mut job_data_inner = job_data.lock().expect("lock error");

    let res = job_data_inner.add_job(&info, &config)?;

    log::info!(target: "post_job_handler", "post job {}", res.id);
    Ok(HttpResponse::Ok().json(res))
}

// get the job list the a query
#[get("/jobs")]
pub async fn get_jobs() -> impl Responder {
    let job_data = JOBDATA.clone();
    let job_data_inner = job_data.lock().expect("lock error");
    let mut temp_job_list: Vec<&Job> = job_data_inner.job_list.iter().collect();
    temp_job_list.sort_by_key(|x| x.created_time);

    let res: Vec<Response> = temp_job_list.iter().map(|x| x.response()).collect();
    drop(job_data_inner);
    log::info!(target: "get jobs", "get jobs list");
    HttpResponse::Ok().json(res)
}

#[get("/")]
pub async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello AC...CTFer?")
}