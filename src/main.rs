use serde::Deserialize;
use std::collections::HashMap;
use std::env; // Import the env module

#[derive(Debug, Deserialize)]
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

#[derive(Debug, Deserialize)]
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

#[derive(Debug, Deserialize)]
struct LogEntry {
    timestamp: String,
    log_message: String,
}

#[derive(Debug, Deserialize)]
struct LogResponse {
    finished: bool,
    logs: Vec<LogEntry>,
}

#[derive(Debug, Deserialize)]
struct GarnixResponse {
    summary: Summary,
    builds: Vec<Build>,
    runs: Vec<serde_json::Value>,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = env::args().collect();

    if args.len() < 3 {
        eprintln!("Usage: {} <JWT_TOKEN> <COMMIT_ID> [PACKAGE_NAME...]", args[0]);
        std::process::exit(1);
    }

    let jwt_token = &args[1];
    let commit_id = &args[2];
    let package_names: Vec<String> = args[3..].to_vec(); // Collect package names again

    let client = reqwest::Client::new(); // Create client once

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

    // --- Summary Report ---
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
            let log_response = client.get(&log_url)
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
                .header("cookie", cookie.clone()) // Use cloned cookie
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

    println!("\n--- Report End ---");

    Ok(())
}
