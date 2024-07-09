
use log::{info,error};
use gitlab::Gitlab;
use gitlab::api::{raw, self, projects, Client, Endpoint, Query, RestClient};
use serde::{Deserialize};
use serde_json::Result;
use serde_json::value;
use std::collections::HashMap;

#[derive(Debug, Deserialize)]
struct Project {
    id: u16,
    name: String,
    _links: HashMap<String,String>,
}

#[derive(Debug,Deserialize)]
struct Languages {
    Go: Option<f32>,
    Java: Option<f32>,
    Kotlin: Option<f32>,
    MakeFile: Option<f32>,
    Dockerfile: Option<f32>,
    TypeScript: Option<f32>,
    CSS: Option<f32>,
    HTML: Option<f32>,
    Shell:Option<f32>,
    Python:Option<f32>,
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

    info!("Using Gitlab: {}",&gitlab_host);

    let client =Gitlab::new(gitlab_host.clone(),gitlab_token.clone()).unwrap();

    let endpoint =projects::Projects::builder()
        .build()
        .unwrap();
    let first_200 :Vec<Project> = api::paged(endpoint, api::Pagination::Limit(200))
        .query(&client)
        .unwrap();

    let mut output : HashMap<String,Languages> = HashMap::default();
    info!("Found {} projects",first_200.len());
    for e in first_200 {
        info!("Pulling language info: {} [{}]",e.name,e.id);
        let url = format!("https://{}/api/v4/projects/{}/languages",gitlab_host,e.id);
        // info!("Calling url: {}",url);
        let client = reqwest::blocking::Client::new();
        let response = client
            .get(url)
            .header("Authorization", gitlab_token.clone())
            .header("PRIVATE-TOKEN",gitlab_token.clone())
            .send();
        match response {
            Ok(r) => {
                
                let body = r.text().unwrap();
                // info!("Got a response: {}",&body);
                let obj : Languages = serde_json::from_str(body.as_str()).unwrap();
                // dbg!(&obj);
                output.insert(e.name, obj);
            }
            Err(e) => {
                error!("Could not get response: {}",e);
            }
        }
    }
    // Output to CSV
    println!("Project,Go,Java,Kotlin,MakeFile,Dockerfile,TypeScript,CSS,HTML,Schell,Python");
    for (name,lang) in output {
        println!("{},{},{},{},{},{},{},{},{},{},{}",
            name,
            lang.Go.unwrap_or_default(),
            lang.Java.unwrap_or_default(),
            lang.Kotlin.unwrap_or_default(),
            lang.MakeFile.unwrap_or_default(),
            lang.Dockerfile.unwrap_or_default(),
            lang.TypeScript.unwrap_or_default(),
            lang.CSS.unwrap_or_default(),
            lang.HTML.unwrap_or_default(),
            lang.Shell.unwrap_or_default(),
            lang.Python.unwrap_or_default(),
        );
    };
}
