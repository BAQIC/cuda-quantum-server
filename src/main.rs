use axum::{
    extract::Request,
    http::{header, StatusCode},
    routing, Form, Json, RequestExt, Router,
};
use serde::Deserialize;
use serde_json::{json, Value};
use std::{
    fs::{self, File},
    io::{self, Write},
    process::{Command, Output},
};
use uuid::Uuid;

#[derive(Deserialize)]
pub struct SubmitMessage {
    pub code: String,
}

async fn compile(source: &str, target: &str) -> io::Result<Output> {
    Command::new("nvq++")
        .arg("--target")
        .arg("emulate")
        .arg("--emulate-url")
        .arg("http://127.0.0.1:3000")
        .arg("--disable-qubit-mapping")
        .arg("-o")
        .arg(target)
        .arg(source)
        .output()
}

async fn run_compiled_file(target: &str) -> io::Result<Output> {
    Command::new(target).output()
}

async fn save_source_file(code: &str, source: &str) -> io::Result<()> {
    let mut file = File::create(source)?;
    file.write_all(code.as_bytes())?;
    Ok(())
}

async fn remove_source_target_files(source: &str, target: &str) -> io::Result<()> {
    fs::remove_file(source)?;
    fs::remove_file(target)?;
    Ok(())
}

pub async fn consume_task(Form(submit_message): Form<SubmitMessage>) -> (StatusCode, Json<Value>) {
    let name = Uuid::new_v4().to_string();
    let source = "/tmp/".to_string() + &name + ".cpp";
    let target = "/tmp/".to_string() + &name + ".x";

    match save_source_file(&submit_message.code, &source).await {
        Ok(_) => match compile(&source, &target).await {
            Ok(compile_output) if compile_output.status.code() == Some(0) => {
                match run_compiled_file(&target).await {
                    Ok(execute_output) if execute_output.status.code() == Some(0) => {
                        match remove_source_target_files(&source, &target).await {
                            Ok(_) => {}
                            Err(err) => {
                                println!("{}", err);
                            }
                        };

                        (
                            StatusCode::OK,
                            Json(
                                json!({"Output": String::from_utf8_lossy(&execute_output.stdout).to_string()}),
                            ),
                        )
                    }
                    Ok(execute_output) => match execute_output.status.code() {
                        Some(status) => (
                            StatusCode::INTERNAL_SERVER_ERROR,
                            Json(
                                json!({"Error": format!("Got error {:?} with {:?} when execute compiled file", String::from_utf8_lossy(&compile_output.stderr), status)}),
                            ),
                        ),
                        None => (
                            StatusCode::INTERNAL_SERVER_ERROR,
                            Json(json!({"Error": "Can't get signal from execute process"})),
                        ),
                    },
                    Err(err) => (
                        StatusCode::INTERNAL_SERVER_ERROR,
                        Json(
                            json!({"Error": format!("Got error {:?} when run compiled file", err)}),
                        ),
                    ),
                }
            }
            Ok(compile_output) => match compile_output.status.code() {
                Some(status) => (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    Json(
                        json!({"Error": format!("Got error {:?} with status {:?} when compiled source file", String::from_utf8_lossy(&compile_output.stderr), status)}),
                    ),
                ),
                None => (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    Json(json!({"Error": "Can't get signal from compile process"})),
                ),
            },
            Err(err) => (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({"Error": format!("Got error {:?} when compile source file", err)})),
            ),
        },
        Err(err) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({"Error": format!("Got error {:?} when saved source file", err)})),
        ),
    }
}

pub async fn submit(request: Request) -> (StatusCode, Json<Value>) {
    match request.headers().get(header::CONTENT_TYPE) {
        Some(content_type) => match content_type.to_str().unwrap() {
            "application/x-www-form-urlencoded" => {
                let Form(submit_message) = request.extract().await.unwrap();
                consume_task(Form(submit_message)).await
            }
            _ => (
                StatusCode::BAD_REQUEST,
                Json(json!({"Error": format!("content type {:?} not support", content_type)})),
            ),
        },
        _ => (
            StatusCode::BAD_REQUEST,
            Json(json!({"Error": format!("content type not specified")})),
        ),
    }
}

#[tokio::main]
async fn main() {
    let cudaq_router = Router::new().route("/submit", routing::post(submit));
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3001").await.unwrap();
    axum::serve(listener, cudaq_router).await.unwrap();
    // println!("{}", format!("-o {}.out", Uuid::new_v4()))
}
