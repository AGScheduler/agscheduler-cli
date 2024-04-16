use chrono::Local;
use dialoguer::{theme::ColorfulTheme, Confirm, Input, Select};
use mockall::automock;

#[automock]
pub trait InteractionTrait {
    fn input_id(&self) -> String;
    fn confirm_delete(&self) -> bool;

    fn input_common(&self, prompt: &str, default: &str, text: &str) -> String;
    fn input_name(&self, text: &str) -> String;
    fn input_start_at(&self, text: &str) -> String;
    fn input_interval(&self, text: &str) -> String;
    fn input_cron_expr(&self, text: &str) -> String;
    fn input_timezone(&self, text: &str) -> String;
    fn input_args(&self, text: &str) -> String;
    fn input_timeout(&self, text: &str) -> String;
    fn input_queues(&self, text: &str) -> String;

    fn select_common(&self, prompt: &str, selections: Vec<String>, default: usize) -> String;
    fn select_type(&self) -> String;
    fn select_func_name(&self, selections: Vec<String>) -> String;
}

pub struct Interaction;

impl InteractionTrait for Interaction {
    fn input_id(&self) -> String {
        Input::with_theme(&ColorfulTheme::default())
            .with_prompt("ID")
            .interact_text()
            .unwrap()
    }

    fn confirm_delete(&self) -> bool {
        Confirm::with_theme(&ColorfulTheme::default())
            .with_prompt("Do you really really want to delete?")
            .default(false)
            .show_default(true)
            .wait_for_newline(true)
            .interact()
            .unwrap()
    }

    fn input_common(&self, prompt: &str, default: &str, text: &str) -> String {
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

    fn input_name(&self, text: &str) -> String {
        self.input_common("Name", "myJob", text)
    }

    fn input_start_at(&self, text: &str) -> String {
        let local_datetime = Local::now().format("%Y-%m-%d %H:%M:%S").to_string();
        self.input_common("StartAt", &local_datetime, text)
    }

    fn input_interval(&self, text: &str) -> String {
        self.input_common("Interval", "60s", text)
    }

    fn input_cron_expr(&self, text: &str) -> String {
        self.input_common("CronExpr", "*/1 * * * *", text)
    }

    fn input_timezone(&self, text: &str) -> String {
        self.input_common("Timezone", "UTC", text)
    }

    fn input_args(&self, text: &str) -> String {
        self.input_common("Args", "{}", text)
    }

    fn input_timeout(&self, text: &str) -> String {
        self.input_common("Timeout", "1h", text)
    }

    fn input_queues(&self, text: &str) -> String {
        self.input_common("Queues", "[\"default\"]", text)
    }

    fn select_common(&self, prompt: &str, selections: Vec<String>, default: usize) -> String {
        let selection = Select::with_theme(&ColorfulTheme::default())
            .with_prompt(prompt)
            .default(default)
            .max_length(4)
            .items(&selections[..])
            .interact()
            .unwrap();

        selections[selection].to_string()
    }

    fn select_type(&self) -> String {
        let selections = vec![
            "Datetime".to_string(),
            "Interval".to_string(),
            "Cron".to_string(),
        ];
        self.select_common("Select Type", selections, 1)
    }

    fn select_func_name(&self, selections: Vec<String>) -> String {
        self.select_common("Select FuncName", selections, 0)
    }
}
