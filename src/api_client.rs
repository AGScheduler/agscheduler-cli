use std::collections::HashMap;

use comfy_table::Table;
use reqwest::Method;
use serde_json::{json, Value};

use crate::interaction::InteractionTrait;
use crate::{datetime, http};

pub struct AGScheduler {
    pub endpoint: String,
}

impl AGScheduler {
    pub async fn get_info(&self) {
        http::fetch_show_json(
            format!("{}{}", &self.endpoint, "/info"),
            http::Options::default(),
        )
        .await;
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

    async fn _edit_job(
        &self,
        data: HashMap<&str, String>,
        method: Method,
        interaction: &dyn InteractionTrait,
    ) {
        let name = interaction.input_name(data.get("name").unwrap());
        let _type = interaction.select_type().to_lowercase();

        let mut start_at = String::new();
        let mut interval = String::new();
        let mut cron_expr = String::new();
        match _type.as_str() {
            "datetime" => {
                start_at = interaction.input_start_at(data.get("start_at").unwrap());
            }
            "interval" => {
                interval = interaction.input_interval(data.get("interval").unwrap());
            }
            "cron" => {
                cron_expr = interaction.input_cron_expr(data.get("cron_expr").unwrap());
            }
            _ => {}
        }

        let mut tz = iana_time_zone::get_timezone().unwrap();
        if !data.get("timezone").unwrap().is_empty() {
            tz = data.get("timezone").unwrap().to_string();
        }
        let timezone = interaction.input_timezone(&tz);

        let mut fn_selections: Vec<String> = vec![];
        match http::fetch(
            format!("{}{}", &self.endpoint, "/funcs"),
            http::Options::default(),
        )
        .await
        {
            Ok(result) => {
                if let Value::Array(list) = result {
                    for f in list {
                        let f_name = f["name"].as_str().unwrap().to_string();
                        fn_selections.push(f_name);
                    }
                }
            }
            Err(err) => {
                println!("Error: {}", err)
            }
        }
        let func_name = interaction.select_func_name(fn_selections);

        let args = interaction.input_args(data.get("args").unwrap());
        let timeout = interaction.input_timeout(data.get("timeout").unwrap());
        let queues = interaction.input_queues(data.get("queues").unwrap());

        let args_value: Value = serde_json::from_str(&args).unwrap();
        let queues_value: Value = serde_json::from_str(&queues).unwrap();
        let body = json!(
            {
                "id": data.get("id").unwrap(),
                "name": name,
                "type": _type,
                "start_at": start_at,
                "interval": interval,
                "cron_expr": cron_expr,
                "timezone": timezone,
                "func_name": func_name,
                "args": args_value,
                "timeout": timeout,
                "queues": queues_value,
            }
        );
        http::fetch_show_json(
            format!("{}{}", &self.endpoint, "/scheduler/job"),
            http::Options {
                method,
                body: body.to_string(),
                ..Default::default()
            },
        )
        .await;
    }

    pub async fn add_job(&self, interaction: &dyn InteractionTrait) {
        let mut data = HashMap::new();
        for key in [
            "id",
            "name",
            "type",
            "start_at",
            "interval",
            "cron_expr",
            "timezone",
            "func_name",
            "args",
            "timeout",
            "queues",
        ] {
            data.insert(key, "".to_string());
        }

        self._edit_job(data, Method::POST, interaction).await;
    }

    pub async fn update_job(&self, interaction: &dyn InteractionTrait) {
        let id = interaction.input_id();

        let mut data = HashMap::new();
        match http::fetch(
            format!("{}{}/{}", &self.endpoint, "/scheduler/job", id),
            http::Options::default(),
        )
        .await
        {
            Ok(result) => {
                data.insert("id", id);
                data.insert("args", result["args"].to_string());
                data.insert("queues", result["queues"].to_string());

                for key in [
                    "name",
                    "type",
                    "start_at",
                    "interval",
                    "cron_expr",
                    "timezone",
                    "func_name",
                    "timeout",
                ] {
                    data.insert(key, result[key].as_str().unwrap().to_string());
                }
            }
            Err(err) => {
                println!("Error: {}", err)
            }
        }

        self._edit_job(data, Method::PUT, interaction).await;
    }

    pub async fn get_job(&self, interaction: &dyn InteractionTrait) {
        let id = interaction.input_id();

        http::fetch_show_json(
            format!("{}{}/{}", &self.endpoint, "/scheduler/job", id),
            http::Options::default(),
        )
        .await;
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
                    "TypeValue",
                    "LastRunTime",
                    "NextRunTime",
                    "Status",
                ]);

                if let Value::Array(list) = result {
                    for j in list {
                        let _type = j["type"].as_str().unwrap();
                        let mut type_value = "";
                        match _type {
                            "datetime" => {
                                type_value = j["start_at"].as_str().unwrap();
                            }
                            "interval" => {
                                type_value = j["interval"].as_str().unwrap();
                            }
                            "cron" => {
                                type_value = j["cron_expr"].as_str().unwrap();
                            }
                            _ => {}
                        }
                        let last_run_time =
                            datetime::parse_iso8601_to_local(j["last_run_time"].as_str().unwrap())
                                .unwrap()
                                .format("%Y-%m-%d %H:%M:%S")
                                .to_string();
                        let next_run_time =
                            datetime::parse_iso8601_to_local(j["next_run_time"].as_str().unwrap())
                                .unwrap()
                                .format("%Y-%m-%d %H:%M:%S")
                                .to_string();
                        table.add_row(vec![
                            j["id"].as_str().unwrap(),
                            j["name"].as_str().unwrap(),
                            _type,
                            type_value,
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
        let id = interaction.input_id();

        if !interaction.confirm_delete() {
            return;
        }

        http::fetch_show_ok(
            format!("{}{}/{id}", &self.endpoint, "/scheduler/job"),
            http::Options {
                method: Method::DELETE,
                ..Default::default()
            },
        )
        .await;
    }

    pub async fn delete_all_jobs(&self, interaction: &dyn InteractionTrait) {
        if !interaction.confirm_delete() {
            return;
        }

        http::fetch_show_ok(
            format!("{}{}", &self.endpoint, "/scheduler/jobs"),
            http::Options {
                method: Method::DELETE,
                ..Default::default()
            },
        )
        .await;
    }

    pub async fn pause_or_resume_job(&self, action: &str, interaction: &dyn InteractionTrait) {
        let id = interaction.input_id();

        http::fetch_show_ok(
            format!("{}{}/{}/{}", &self.endpoint, "/scheduler/job", id, action),
            http::Options {
                method: Method::POST,
                ..Default::default()
            },
        )
        .await;
    }

    pub async fn start_or_stop(&self, action: &str) {
        http::fetch_show_ok(
            format!("{}{}/{}", &self.endpoint, "/scheduler", action),
            http::Options {
                method: Method::POST,
                ..Default::default()
            },
        )
        .await;
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
                        let register_time =
                            datetime::parse_iso8601_to_local(n["register_time"].as_str().unwrap())
                                .unwrap()
                                .format("%Y-%m-%d %H:%M:%S")
                                .to_string();
                        let last_heartbeat_time = datetime::parse_iso8601_to_local(
                            n["last_heartbeat_time"].as_str().unwrap(),
                        )
                        .unwrap()
                        .format("%Y-%m-%d %H:%M:%S")
                        .to_string();
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
    use super::*;
    use serde_json::json;

    use crate::interaction::MockInteractionTrait;

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
            .mock("POST", "/scheduler/job")
            .with_status(200)
            .with_body(&empty_data)
            .create_async()
            .await;
        server
            .mock("PUT", "/scheduler/job")
            .with_status(200)
            .with_body(&empty_data)
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
        mock.expect_input_id().return_const(id);
        mock.expect_confirm_delete().return_const(true);
        mock.expect_input_name().return_const("myJob");
        mock.expect_input_start_at()
            .return_const("2024-04-16 15:23:51");
        mock.expect_input_interval().return_const("60s");
        mock.expect_input_cron_expr().return_const("*/1 * * * *");
        mock.expect_input_timezone().return_const("UTC");
        mock.expect_input_args().return_const("{}");
        mock.expect_input_timeout().return_const("1h");
        mock.expect_input_queues().return_const("[\"default\"]");
        mock.expect_select_type().return_const("Interval");
        mock.expect_select_func_name()
            .return_const("github.com/agscheduler/agscheduler/examples.PrintMsg");

        let ags = AGScheduler { endpoint: url };

        ags.get_info().await;
        ags.get_funcs().await;
        ags.get_job(&mock).await;
        ags.get_all_jobs().await;
        ags.add_job(&mock).await;
        ags.update_job(&mock).await;
        ags.delete_job(&mock).await;
        ags.delete_all_jobs(&mock).await;
        ags.pause_or_resume_job("pause", &mock).await;
        ags.pause_or_resume_job("resume", &mock).await;
        ags.start_or_stop("start").await;
        ags.start_or_stop("stop").await;
        ags.get_cluster_nodes().await;
    }
}
