mod datetime;
mod http;
mod utils;

use comfy_table::Table;
use reqwest::Method;
use serde_json::Value;

pub struct AGScheduler {
    pub endpoint: String,
}

impl AGScheduler {
    pub async fn get_info(&self) {
        match http::fetch(
            format!("{}{}", &self.endpoint, "/info"),
            http::Options::default(),
        )
        .await
        {
            Ok(result) => {
                utils::show_json(result);
            }
            Err(err) => {
                println!("Error: {}", err)
            }
        }
    }

    pub async fn get_funcs(&self) {
        match http::fetch(
            format!("{}{}", &self.endpoint, "/funcs"),
            http::Options::default(),
        )
        .await
        {
            Ok(result) => {
                let mut table = Table::new();
                table.set_header(vec!["name", "info"]);

                if let Value::Array(list) = result {
                    for f in list {
                        table.add_row(vec![
                            f["name"].as_str().unwrap(),
                            f["info"].as_str().unwrap(),
                        ]);
                    }

                    println!("{table}");
                }
            }
            Err(err) => {
                println!("Error: {}", err)
            }
        }
    }

    pub async fn get_job(&self) {
        let id: String = utils::input_job_id();

        match http::fetch(
            format!("{}{}/{}", &self.endpoint, "/scheduler/job", id),
            http::Options::default(),
        )
        .await
        {
            Ok(result) => {
                utils::show_json(result);
            }
            Err(err) => {
                println!("Error: {}", err)
            }
        }
    }

    pub async fn get_all_jobs(&self) {
        match http::fetch(
            format!("{}{}", &self.endpoint, "/scheduler/jobs"),
            http::Options::default(),
        )
        .await
        {
            Ok(result) => {
                let mut table = Table::new();
                table.set_header(vec![
                    "ID",
                    "Name",
                    "Type",
                    "LastRunTime",
                    "NextRunTime",
                    "Status",
                ]);

                if let Value::Array(list) = result {
                    for j in list {
                        let last_run_time = format!(
                            "{}",
                            datetime::parse_iso8601_to_local(j["last_run_time"].as_str().unwrap())
                                .unwrap()
                                .format("%Y-%m-%d %H:%M:%S")
                        );
                        let next_run_time = format!(
                            "{}",
                            datetime::parse_iso8601_to_local(j["next_run_time"].as_str().unwrap())
                                .unwrap()
                                .format("%Y-%m-%d %H:%M:%S")
                        );
                        table.add_row(vec![
                            j["id"].as_str().unwrap(),
                            j["name"].as_str().unwrap(),
                            j["type"].as_str().unwrap(),
                            &last_run_time[..],
                            &next_run_time[..],
                            j["status"].as_str().unwrap(),
                        ]);
                    }

                    println!("{table}");
                }
            }
            Err(err) => {
                println!("Error: {}", err)
            }
        }
    }

    pub async fn delete_job(&self) {
        let id: String = utils::input_job_id();

        if !utils::confirm_delete() {
            return;
        }

        let mut options = http::Options::default();
        options.method = Method::DELETE;
        match http::fetch(
            format!("{}{}/{id}", &self.endpoint, "/scheduler/job"),
            options,
        )
        .await
        {
            Ok(_) => {
                println!("Ok")
            }
            Err(err) => {
                println!("Error: {}", err)
            }
        }
    }

    pub async fn delete_all_jobs(&self) {
        if !utils::confirm_delete() {
            return;
        }

        let mut options = http::Options::default();
        options.method = Method::DELETE;
        match http::fetch(format!("{}{}", &self.endpoint, "/scheduler/jobs"), options).await {
            Ok(_) => {
                println!("Ok")
            }
            Err(err) => {
                println!("Error: {}", err)
            }
        }
    }

    pub async fn pause_or_resume_job(&self, action: &str) {
        let id: String = utils::input_job_id();

        let mut options = http::Options::default();
        options.method = Method::POST;
        match http::fetch(
            format!("{}{}/{}/{}", &self.endpoint, "/scheduler/job", id, action),
            options,
        )
        .await
        {
            Ok(_) => {
                println!("Ok")
            }
            Err(err) => {
                println!("Error: {}", err)
            }
        }
    }

    pub async fn start_or_stop(&self, action: &str) {
        let mut options = http::Options::default();
        options.method = Method::POST;
        match http::fetch(
            format!("{}{}/{}", &self.endpoint, "/scheduler", action),
            options,
        )
        .await
        {
            Ok(_) => {
                println!("Ok")
            }
            Err(err) => {
                println!("Error: {}", err)
            }
        }
    }

    pub async fn get_cluster_nodes(&self) {
        match http::fetch(
            format!("{}{}", &self.endpoint, "/cluster/nodes"),
            http::Options::default(),
        )
        .await
        {
            Ok(result) => {
                let mut table = Table::new();
                table.set_header(vec![
                    "Endpoint",
                    "EndpointGRPC",
                    "EndpointHTTP",
                    "EndpointMain",
                    "Queue",
                    "Mode",
                    "Version",
                    "Health",
                    "RegisterTime",
                    "LastHeartbeatTime",
                ]);

                if let Value::Object(map) = result {
                    for (_, n) in map.iter() {
                        let register_time = format!(
                            "{}",
                            datetime::parse_iso8601_to_local(n["register_time"].as_str().unwrap())
                                .unwrap()
                                .format("%Y-%m-%d %H:%M:%S")
                        );
                        let last_heartbeat_time = format!(
                            "{}",
                            datetime::parse_iso8601_to_local(
                                n["last_heartbeat_time"].as_str().unwrap()
                            )
                            .unwrap()
                            .format("%Y-%m-%d %H:%M:%S")
                        );
                        table.add_row(vec![
                            n["endpoint"].as_str().unwrap(),
                            n["endpoint_grpc"].as_str().unwrap(),
                            n["endpoint_http"].as_str().unwrap(),
                            n["endpoint_main"].as_str().unwrap(),
                            n["queue"].as_str().unwrap(),
                            n["mode"].as_str().unwrap(),
                            n["version"].as_str().unwrap(),
                            &n["health"].as_bool().unwrap().to_string()[..],
                            &register_time[..],
                            &last_heartbeat_time[..],
                        ]);
                    }

                    println!("{table}");
                }
            }
            Err(err) => {
                println!("Error: {}", err)
            }
        }
    }
}
