use comfy_table::Table;
use reqwest::Method;
use serde_json::Value;

use crate::utils::InteractionTrait;
use crate::{datetime, http, utils};

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

    pub async fn get_job(&self, interaction: &dyn InteractionTrait) {
        let id: String = interaction.input_job_id();

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

    pub async fn delete_job(&self, interaction: &dyn InteractionTrait) {
        let id: String = interaction.input_job_id();

        if !interaction.confirm_delete() {
            return;
        }

        match http::fetch(
            format!("{}{}/{id}", &self.endpoint, "/scheduler/job"),
            http::Options {
                method: Method::DELETE,
                ..Default::default()
            },
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

    pub async fn delete_all_jobs(&self, interaction: &dyn InteractionTrait) {
        if !interaction.confirm_delete() {
            return;
        }

        match http::fetch(
            format!("{}{}", &self.endpoint, "/scheduler/jobs"),
            http::Options {
                method: Method::DELETE,
                ..Default::default()
            },
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

    pub async fn pause_or_resume_job(&self, action: &str, interaction: &dyn InteractionTrait) {
        let id: String = interaction.input_job_id();

        match http::fetch(
            format!("{}{}/{}/{}", &self.endpoint, "/scheduler/job", id, action),
            http::Options {
                method: Method::POST,
                ..Default::default()
            },
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
        match http::fetch(
            format!("{}{}/{}", &self.endpoint, "/scheduler", action),
            http::Options {
                method: Method::POST,
                ..Default::default()
            },
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

#[cfg(test)]
mod tests {
    use crate::utils::MockInteractionTrait;

    use super::*;
    use serde_json::json;

    #[tokio::test]
    async fn it_api_client() {
        let mut server = mockito::Server::new_async().await;
        let url = server.url();

        let id = String::from("00227fbf671f4ed2");
        let empty_data = json!({"data": null, "error": ""}).to_string();

        server
            .mock("GET", "/info")
            .with_status(200)
            .with_body(
                json!({
                    "data": {
                        "cluster_main_node": {
                            "endpoint": "127.0.0.1:36380",
                            "endpoint_grpc": "127.0.0.1:36360",
                            "endpoint_http": "127.0.0.1:36370",
                            "endpoint_main": "127.0.0.1:36380",
                            "mode": ""
                        },
                        "is_cluster_mode": true,
                        "is_running": false,
                        "version": "0.6.1"
                    },
                    "error": ""
                })
                .to_string(),
            )
            .create_async()
            .await;
        server
            .mock("GET", "/funcs")
            .with_status(200)
            .with_body(
                json!({
                    "data": [
                        {
                            "info": "",
                            "name": "github.com/agscheduler/agscheduler/examples.PrintMsg"
                        }
                    ],
                    "error": ""
                })
                .to_string(),
            )
            .create_async()
            .await;
        server
            .mock("GET", "/scheduler/job/00227fbf671f4ed2")
            .with_status(200)
            .with_body(
                json!({
                    "data":  {
                        "args": {

                        },
                        "cron_expr": "",
                        "end_at": "",
                        "func_name": "github.com/agscheduler/agscheduler/examples.PrintMsg",
                        "id": "00227fbf671f4ed2",
                        "interval": "60s",
                        "last_run_time": "0001-01-01T00:00:00Z",
                        "name": "myJob",
                        "next_run_time": "2024-04-15T04:19:12Z",
                        "queues": [
                            "default"
                        ],
                        "start_at": "",
                        "status": "running",
                        "timeout": "1h",
                        "timezone": "UTC",
                        "type": "interval"
                    },
                    "error": ""
                })
                .to_string(),
            )
            .create_async()
            .await;
        server
            .mock("GET", "/scheduler/jobs")
            .with_status(200)
            .with_body(
                json!({
                    "data": [
                        {
                            "args": {

                            },
                            "cron_expr": "",
                            "end_at": "",
                            "func_name": "github.com/agscheduler/agscheduler/examples.PrintMsg",
                            "id": "00227fbf671f4ed2",
                            "interval": "60s",
                            "last_run_time": "0001-01-01T00:00:00Z",
                            "name": "myJob",
                            "next_run_time": "2024-04-15T04:19:12Z",
                            "queues": [
                                "default"
                            ],
                            "start_at": "",
                            "status": "running",
                            "timeout": "1h",
                            "timezone": "UTC",
                            "type": "interval"
                        }
                    ],
                    "error": ""
                })
                .to_string(),
            )
            .create_async()
            .await;
        server
            .mock("DELETE", "/scheduler/job/00227fbf671f4ed2")
            .with_status(200)
            .with_body(&empty_data)
            .create_async()
            .await;
        server
            .mock("DELETE", "/scheduler/jobs")
            .with_status(200)
            .with_body(&empty_data)
            .create_async()
            .await;
        server
            .mock("POST", "/scheduler/job/00227fbf671f4ed2/pause")
            .with_status(200)
            .with_body(&empty_data)
            .create_async()
            .await;
        server
            .mock("POST", "/scheduler/job/00227fbf671f4ed2/resume")
            .with_status(200)
            .with_body(&empty_data)
            .create_async()
            .await;
        server
            .mock("POST", "/scheduler/start")
            .with_status(200)
            .with_body(&empty_data)
            .create_async()
            .await;
        server
            .mock("POST", "/scheduler/stop")
            .with_status(200)
            .with_body(&empty_data)
            .create_async()
            .await;
        server
            .mock("GET", "/cluster/nodes")
            .with_status(200)
            .with_body(
                json!({
                    "data": {
                        "127.0.0.1:36380": {
                            "endpoint": "127.0.0.1:36380",
                            "endpoint_grpc": "127.0.0.1:36360",
                            "endpoint_http": "127.0.0.1:36370",
                            "endpoint_main": "127.0.0.1:36380",
                            "health": true,
                            "last_heartbeat_time": "2024-04-15T04:30:08.489043439Z",
                            "mode": "",
                            "queue": "default",
                            "register_time": "2024-04-15T04:08:10.438222846Z",
                            "version": "0.6.1"
                        }
                    },
                    "error": ""
                })
                .to_string(),
            )
            .create_async()
            .await;

        let mut mock = MockInteractionTrait::new();
        mock.expect_input_job_id().return_const(id);
        mock.expect_confirm_delete().return_const(true);

        let ags = AGScheduler { endpoint: url };

        ags.get_info().await;
        ags.get_funcs().await;
        ags.get_job(&mock).await;
        ags.get_all_jobs().await;
        ags.delete_job(&mock).await;
        ags.delete_all_jobs(&mock).await;
        ags.pause_or_resume_job("pause", &mock).await;
        ags.pause_or_resume_job("resume", &mock).await;
        ags.start_or_stop("start").await;
        ags.start_or_stop("stop").await;
        ags.get_cluster_nodes().await;
    }
}
