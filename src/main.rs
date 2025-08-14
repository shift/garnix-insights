use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::env;

// Actix-web imports
use actix_web::{post, web, App, HttpResponse, HttpServer, Responder};

#[derive(Debug, Deserialize, Serialize)]
struct Summary {
    repo_owner: String,
    repo_name: String,
    repo_is_public: bool,
    git_commit: String,
    branch: String,
    req_user: String,
    start_time: String,
    succeeded: u32,
    failed: u32,
    pending: u32,
    cancelled: u32,
}

#[derive(Debug, Deserialize, Serialize)]
struct Build {
    id: String,
    repo_user: String,
    repo_name: String,
    branch: String,
    repo_is_public: bool,
    git_commit: String,
    package: String,
    package_type: String,
    system: Option<String>,
    req_user: String,
    status: String,
    start_time: String,
    end_time: String,
    drv_path: Option<String>,
    output_paths: Option<HashMap<String, String>>,
    github_run_id: u64,
    wants_incrementalism: bool,
    eval_host: String,
    uploaded_to_cache: bool,
}

#[derive(Debug, Deserialize, Serialize)]
struct LogEntry {
    timestamp: String,
    log_message: String,
}

#[derive(Debug, Deserialize, Serialize)]
struct LogResponse {
    finished: bool,
    logs: Vec<LogEntry>,
}

#[derive(Debug, Deserialize, Serialize)]
struct GarnixResponse {
    summary: Summary,
    builds: Vec<Build>,
    runs: Vec<serde_json::Value>,
}

// Request struct for the build status endpoint
#[derive(Deserialize)]
struct BuildStatusRequest {
    jwt_token: String,
    commit_id: String,
}

// Actix-web handler for /build-status
#[post("/build-status")]
async fn build_status_handler(req: web::Json<BuildStatusRequest>) -> impl Responder {
    let jwt_token = &req.jwt_token;
    let commit_id = &req.commit_id;

    match get_garnix_data(jwt_token, commit_id).await {
        Ok(response) => HttpResponse::Ok().json(response),
        Err(e) => HttpResponse::InternalServerError().body(format!("Error: {}", e)),
    }
}

// Function to fetch and parse Garnix data
async fn get_garnix_data(jwt_token: &str, commit_id: &str) -> Result<GarnixResponse, Box<dyn std::error::Error>> {
    let client = reqwest::Client::new(); // Create client inside function
    let url = format!("https://garnix.io/api/commits/{}", commit_id);
    let cookie = format!("__stripe_mid=119e351f-f0e8-4943-abae-6e207d8b6aac548adf; JWT-Cookie={}; NO-XSRF-TOKEN=", jwt_token);

    let response = client.get(&url)
        .header("accept", "*/*")
        .header("accept-language", "en-GB,en-US;q=0.9,en;q=0.8,de;q=0.7")
        .header("priority", "u=1, i")
        .header("sec-ch-ua", r#""Not)A;Brand";v="8", "Chromium";v="138", "Google Chrome";v="138""#)
        .header("sec-ch-ua-mobile", "?0")
        .header("sec-ch-ua-platform", "\"Linux\"")
        .header("sec-fetch-dest", "empty")
        .header("sec-fetch-mode", "cors")
        .header("sec-fetch-site", "same-origin")
        .header("referer", format!("https://garnix.io/commit/{}", commit_id))
        .header("user-agent", "Mozilla/5.0 (X11; Linux x86_64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/138.0.0.0 Safari/537.36")
        .header("cookie", cookie.clone()) // Clone cookie for multiple requests
        .send()
        .await?
        .json::<GarnixResponse>()
        .await?;

    Ok(response)
}

fn main() -> Result<(), Box<dyn std::error::Error>> { // main is now synchronous
    let args: Vec<String> = env::args().collect();

    // Check if running as server
    if args.len() == 2 && args[1] == "--server" {
        return tokio::runtime::Runtime::new().unwrap().block_on(async {
            HttpServer::new(|| {
                App::new().service(build_status_handler)
            })
            .bind(("127.0.0.1", 8080))?.run().await
        })
    } else {
        // Run as CLI tool
        return tokio::runtime::Runtime::new().unwrap().block_on(async {
            if args.len() < 3 { // Expect program name, JWT, and Commit ID
                eprintln!("Usage: {} <JWT_TOKEN> <COMMIT_ID> [--json-output] [PACKAGE_NAME...]", args[0]);
                std::process::exit(1);
            }

            let jwt_token = &args[1];
            let commit_id = &args[2];
            let mut package_names: Vec<String> = Vec::new();
            let mut json_output = false;

            // Parse optional arguments for CLI
            let mut i = 3;
            while i < args.len() {
                if args[i] == "--json-output" {
                    json_output = true;
                }
                i += 1;
            }

            let response = get_garnix_data(jwt_token, commit_id).await?;

            if json_output {
                // Output as JSON
                println!("{}", serde_json::to_string_pretty(&response)?);
            } else {
                // --- Summary Report (Human-readable) ---
                println!("\n--- Garnix Build Report ---");
                println!("Repository: {}/{}", response.summary.repo_owner, response.summary.repo_name);
                println!("Commit ID: {}", response.summary.git_commit);
                println!("Branch: {}", response.summary.branch);
                println!("Requested by: {}", response.summary.req_user);
                println!("Start Time: {}", response.summary.start_time);
                println!("Builds Succeeded: {}", response.summary.succeeded);
                println!("Builds Failed: {}", response.summary.failed);
                println!("Builds Pending: {}", response.summary.pending);
                println!("Builds Cancelled: {}", response.summary.cancelled);

                // --- All Builds Status (Table Format) ---
                println!("\n--- All Builds Status ---");
                println!("{:<30} {:<10}", "Package", "Status");
                println!("{:-<30} {:-<10}", "", "");

                let mut failed_builds: Vec<&Build> = Vec::new();

                let builds_to_display: Vec<&Build> = if package_names.is_empty() {
                    response.builds.iter().collect()
                } else {
                    response.builds.iter()
                        .filter(|build| package_names.contains(&build.package))
                        .collect()
                };

                if builds_to_display.is_empty() {
                    println!("No builds found for the specified packages or commit.");
                } else {
                    for build in builds_to_display {
                        let status_emoji = if build.status == "Success" { "✅" } else { "❌" };
                        println!("{:<30} {:<10}",
                                 build.package,
                                 status_emoji);

                        if build.status != "Success" {
                            failed_builds.push(build);
                        }
                    }
                }

                // --- Summary of Failed Builds ---
                println!("\n--- Summary of Failed Builds ---");
                if failed_builds.is_empty() {
                    println!("No failed builds.");
                } else {
                    for build in failed_builds {
                        println!("- Package: {} (Status: {})", build.package, build.status);
                        if let Some(drv_path) = &build.drv_path {
                            println!("  DRV Path: {}", drv_path);
                        }
                        if let Some(output_paths) = &build.output_paths {
                            println!("  Output Paths:");
                            for (key, value) in output_paths {
                                println!("    {}: {}", key, value);
                            }
                        }
                        // Fetch and display logs
                        let log_url = format!("https://garnix.io/api/build/{}/logs", build.id);
                        let log_response = reqwest::Client::new().get(&log_url)
                            .header("accept", "*/*")
                            .header("accept-language", "en-GB,en-US;q=0.9,en;q=0.8,de;q=0.7")
                            .header("priority", "u=1, i")
                            .header("sec-ch-ua", r#""Not)A;Brand";v="8", "Chromium";v="138", "Google Chrome";v="138""#)
                            .header("sec-ch-ua-mobile", "?0")
                            .header("sec-ch-ua-platform", "\"Linux\"")
                            .header("sec-fetch-dest", "empty")
                            .header("sec-fetch-mode", "cors")
                            .header("sec-fetch-site", "same-origin")
                            .header("referer", format!("https://garnix.io/build/{}", build.id))
                            .header("user-agent", "Mozilla/5.0 (X11; Linux x86_64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/138.0.0.0 Safari/537.36")
                            .header("cookie", format!("__stripe_mid=119e3511f-f0e8-4943-abae-6e207d8b6aac548adf; JWT-Cookie={}; NO-XSRF-TOKEN=", jwt_token)) // Use jwt_token directly
                            .send()
                            .await?;

                        if log_response.status().is_success() {
                            let log_data = log_response.json::<LogResponse>().await?;
                            println!("  Logs:");
                            for entry in log_data.logs {
                                println!("    {}", entry.log_message);
                            }
                        } else {
                            println!("  Failed to fetch logs: Status {}", log_response.status());
                        }
                    }
                }
            }

            println!("\n--- Report End ---");
        })
    }

    Ok(())
}