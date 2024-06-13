
use log::{info,error};
use gitlab::Gitlab;
use gitlab::api::{raw, self, projects, Client, Endpoint, Query, RestClient};
use serde::{Deserialize};
use std::collections::HashMap;

#[derive(Debug, Deserialize)]
struct Project {
    id: u16,
    name: String,
    _links: HashMap<String,String>,
}

fn main() {
    env_logger::init();

    let ver = env!("CARGO_PKG_VERSION");
    let pkg = env!("CARGO_PKG_NAME");

    let gitlab_host = match std::env::var("GITLAB_HOST") {
        Ok(h) => h,
        Err(_e) => panic!("Cannot proceed without host"),
    };

    let gitlab_token = match std::env::var("GITLAB_TOKEN") {
        Ok(t) => t,
        Err(_e) => panic!("Cannnot proceed without Token")
    };

    info!("Starting {} v{}",pkg,ver);

    info!("Using Gitlab: {}",gitlab_host);

    let client =Gitlab::new(gitlab_host, gitlab_token).unwrap();

    let endpoint =projects::Projects::builder()
        .build()
        .unwrap();
    let first_200 :Vec<Project> = api::paged(endpoint, api::Pagination::Limit(200))
        .query(&client)
        .unwrap();

    info!("Found {} projects",first_200.len());
    for e in first_200 {
        info!("Pulling language info: {} [{}]",e.name,e.id);
    }
}
