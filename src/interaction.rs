#![cfg_attr(coverage_nightly, feature(coverage_attribute))]

use chrono::Local;
use dialoguer::{theme::ColorfulTheme, Confirm, Input, Select};
use mockall::automock;

#[automock]
pub trait InteractionTrait {
    fn input_common(&self, prompt: &str) -> String;
    fn input_id(&self) -> String;
    fn input_job_id(&self) -> String;
    fn confirm_delete(&self) -> bool;

    fn input_common_default(&self, prompt: &str, default: &str, text: &str) -> String;
    fn input_name(&self, text: &str) -> String;
    fn input_start_at(&self, text: &str) -> String;
    fn input_interval(&self, text: &str) -> String;
    fn input_cron_expr(&self, text: &str) -> String;
    fn input_timezone(&self, text: &str) -> String;
    fn input_args(&self, text: &str) -> String;
    fn input_timeout(&self, text: &str) -> String;
    fn input_queues(&self, text: &str) -> String;

    fn input_page(&self, text: &str) -> String;
    fn input_page_size(&self, text: &str) -> String;

    fn select_common(&self, prompt: &str, selections: Vec<String>, default: usize) -> String;
    fn select_type(&self) -> String;
    fn select_func_name(&self, selections: Vec<String>) -> String;
}

pub struct Interaction;

impl InteractionTrait for Interaction {
    #[cfg_attr(coverage_nightly, coverage(off))]
    fn input_common(&self, prompt: &str) -> String {
        Input::with_theme(&ColorfulTheme::default())
            .with_prompt(prompt)
            .interact_text()
            .unwrap()
    }

    #[cfg_attr(coverage_nightly, coverage(off))]
    fn input_id(&self) -> String {
        self.input_common("ID").trim().to_string()
    }

    #[cfg_attr(coverage_nightly, coverage(off))]
    fn input_job_id(&self) -> String {
        self.input_common("JobId").trim().to_string()
    }

    #[cfg_attr(coverage_nightly, coverage(off))]
    fn confirm_delete(&self) -> bool {
        Confirm::with_theme(&ColorfulTheme::default())
            .with_prompt("Do you really really want to delete?")
            .default(false)
            .show_default(true)
            .wait_for_newline(true)
            .interact()
            .unwrap()
    }

    #[cfg_attr(coverage_nightly, coverage(off))]
    fn input_common_default(&self, prompt: &str, default: &str, text: &str) -> String {
        let mut initial_text = default.to_string();
        if !text.is_empty() {
            initial_text = text.to_string();
        }

        Input::with_theme(&ColorfulTheme::default())
            .with_prompt(prompt)
            .default(default.to_string())
            .with_initial_text(initial_text)
            .interact_text()
            .unwrap()
    }

    #[cfg_attr(coverage_nightly, coverage(off))]
    fn input_name(&self, text: &str) -> String {
        self.input_common_default("Name", "myJob", text)
            .trim()
            .to_string()
    }

    #[cfg_attr(coverage_nightly, coverage(off))]
    fn input_start_at(&self, text: &str) -> String {
        let local_datetime = Local::now().format("%Y-%m-%d %H:%M:%S").to_string();
        self.input_common_default("StartAt", &local_datetime, text)
            .trim()
            .to_string()
    }

    #[cfg_attr(coverage_nightly, coverage(off))]
    fn input_interval(&self, text: &str) -> String {
        self.input_common_default("Interval", "60s", text)
            .trim()
            .to_string()
    }

    #[cfg_attr(coverage_nightly, coverage(off))]
    fn input_cron_expr(&self, text: &str) -> String {
        self.input_common_default("CronExpr", "*/1 * * * *", text)
            .trim()
            .to_string()
    }

    #[cfg_attr(coverage_nightly, coverage(off))]
    fn input_timezone(&self, text: &str) -> String {
        self.input_common_default("Timezone", "UTC", text)
            .trim()
            .to_string()
    }

    #[cfg_attr(coverage_nightly, coverage(off))]
    fn input_args(&self, text: &str) -> String {
        self.input_common_default("Args", "{}", text)
            .trim()
            .to_string()
    }

    #[cfg_attr(coverage_nightly, coverage(off))]
    fn input_timeout(&self, text: &str) -> String {
        self.input_common_default("Timeout", "1h", text)
            .trim()
            .to_string()
    }

    #[cfg_attr(coverage_nightly, coverage(off))]
    fn input_queues(&self, text: &str) -> String {
        self.input_common_default("Queues", "[]", text)
            .trim()
            .to_string()
    }

    #[cfg_attr(coverage_nightly, coverage(off))]
    fn input_page(&self, text: &str) -> String {
        self.input_common_default("Page", "1", text)
            .trim()
            .to_string()
    }

    #[cfg_attr(coverage_nightly, coverage(off))]
    fn input_page_size(&self, text: &str) -> String {
        self.input_common_default("PageSize", "10", text)
            .trim()
            .to_string()
    }

    #[cfg_attr(coverage_nightly, coverage(off))]
    fn select_common(&self, prompt: &str, selections: Vec<String>, default: usize) -> String {
        let selection = Select::with_theme(&ColorfulTheme::default())
            .with_prompt(prompt)
            .default(default)
            .max_length(8)
            .items(&selections[..])
            .interact()
            .unwrap();

        selections[selection].to_string()
    }

    #[cfg_attr(coverage_nightly, coverage(off))]
    fn select_type(&self) -> String {
        let selections = vec![
            "Datetime".to_string(),
            "Interval".to_string(),
            "Cron".to_string(),
        ];
        self.select_common("Select Type", selections, 1)
    }

    #[cfg_attr(coverage_nightly, coverage(off))]
    fn select_func_name(&self, selections: Vec<String>) -> String {
        self.select_common("Select FuncName", selections, 0)
    }
}
