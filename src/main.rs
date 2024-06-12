
use log::{info,error};
use gitlab::Gitlab;
use gitlab::api::{self,projects, Query};


fn main() {
    env_logger::init();

    let ver = env!("CARGO_PKG_VERSION");
    let pkg = env!("CARGO_PKG_NAME");

    let gitlab_host = match std::env::var("GITLAB_HOST") {
        Ok(h) => h,
        Err(e) => panic!("Cannot proceed without host"),
    };

    let gitlab_token = match std::env::var("GITLAB_TOKEN") {
        Ok(t) => t,
        Err(e) => panic!("Cannnot proceed without Token")
    };

    info!("Starting {} v{}",pkg,ver);

    info!("Using Gitlab: {}",gitlab_host);

    let client =Gitlab::new(gitlab_host, gitlab_token);

    let endpoint = projects::Projects::

    dbg!(endpoint);
}
