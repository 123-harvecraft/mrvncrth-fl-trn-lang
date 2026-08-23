//global or general function for all codes
use warp::{http::StatusCode, reply::json, Reply};
use crate::domain::models::routine::srv::RespSrv;
use crate::WebResult;
use chrono::{Local, NaiveDate, Utc};
use uuid::Uuid;
use std::sync::atomic::{AtomicUsize, Ordering};
use chrono::offset::{TimeZone};
use log::{error, info};
//Utc

static COUNTER: AtomicUsize = AtomicUsize::new(1);

/**
* date and time
*/
pub fn current_time() -> String {
    let now = Local::now();
    format!("{}", now.to_rfc3339())
}

pub fn current_time_ymd() -> String {
    // let now = Local::now();
    // format!("{}", now.to_rfc3339())
    // format!("{:04}{:02}{:02}", now.year(), now.month(), now.day())
    Local::now().format("%Y%m%d").to_string()
}

pub fn maximum_days_of_month(month_input: u32, year_input: i32) -> i64 {
    info!("searching for maximum_days_of_month for year {} month {} ", year_input, month_input);

    let mut return_maximum_days = 0;
    let year = year_input;
    for (m, d) in (1..=12).map(|m| {
        (
            m,
            if m == 12 {
                NaiveDate::from_ymd(year + 1, 1, 1)
            } else {
                NaiveDate::from_ymd(year, m + 1, 1)
            }.signed_duration_since(NaiveDate::from_ymd(year, m, 1))
                .num_days(),
        )
    }) {
        println!("days {} in month {}", d, m);
        if(m==month_input) {

            info!("[SUCCESS] days {} in month {}", d, m);

            return_maximum_days = d;
        }else{

            error!("not available of year {} month {} ", year_input, month_input);

            0;
        }
    }

    return_maximum_days
}


pub fn maximum_days_of_between_date(month_input_start: u32,
                         month_input_end: u32,
                         year_input_start: i32,
                         year_input_end: i32,
                         day_input_start: u32,
                         day_input_end: u32) -> i64 {

    info!("[CALC] maximum_days_of_between_date {} {} {} until {} {} {} ",
        year_input_start,
        month_input_start,
        day_input_start,
        year_input_end,
        month_input_end,
        day_input_end);

    let a = Utc.with_ymd_and_hms(year_input_start, month_input_start, day_input_start, 0, 0, 0).unwrap();

    let b = Utc.with_ymd_and_hms(year_input_end, month_input_end, day_input_end, 0, 0, 0).unwrap();

    let diff = b - a;

    info!("diff: {}",diff);

    let mut return_maximum_days: i64 = 0;

    info!("[SUCCESS] maximum_days_of_between_date date result {} ", diff.num_days());

    return_maximum_days = diff.num_days();

    return_maximum_days
}


/**
* id and nuumbering
*/
pub fn uniqueIdUUID() -> String {
    format!("{}", Uuid::new_v4())
}

pub fn generateTime() -> String {
    Utc::now().format("%Y%m%d%H%M%S%f").to_string()
}

pub fn generateidcounter() -> String {//usize
    // use the datatable to save latest number seq
    format!("{}", COUNTER.fetch_add(1, Ordering::Relaxed))
}

/**
* response and service
*/
pub fn srv_response(message: String, status: StatusCode) -> WebResult<impl Reply> {
    let response = RespSrv {
        message: message.to_string(),
        status: status.as_u16(),
    };

    Ok(json(&response))
}


pub fn utils_embedding_vec_dim(text: &str, dim_input: usize) -> Vec<f32> {
    let dim = dim_input;
    let mut embedding = vec![0.0; dim];
    for (i, byte) in text.bytes().enumerate() {
        embedding[i % dim] += byte as f32 / 255.0;
    }
    embedding
}

pub fn utils_embedding_vec(text: &str) -> Vec<f32> {
    let dim = 3;
    let mut embedding = vec![0.0; dim];
    for (i, byte) in text.bytes().enumerate() {
        embedding[i % dim] += byte as f32 / 255.0;
    }
    embedding
}
