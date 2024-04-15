use dialoguer::{theme::ColorfulTheme, Confirm, Input};
use mockall::automock;
use serde_json::Value;

pub fn show_json(result: Value) {
    let formatted_json = serde_json::to_string_pretty(&result)
        .unwrap_or_else(|err| format!("Format json error: {err}"));

    println!("{formatted_json}");
}

#[automock]
pub trait InteractionTrait {
    fn input_job_id(&self) -> String;
    fn confirm_delete(&self) -> bool;
}

pub struct Interaction;

impl InteractionTrait for Interaction {
    fn input_job_id(&self) -> String {
        let id = Input::with_theme(&ColorfulTheme::default())
            .with_prompt("Job ID")
            .interact_text()
            .unwrap();

        id
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
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn it_show_json() {
        let json_value = json!({"case": "test"});
        show_json(json_value);
    }
}
