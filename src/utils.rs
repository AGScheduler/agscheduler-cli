use dialoguer::{theme::ColorfulTheme, Confirm, Input};
use serde_json::Value;

pub fn show_json(result: Value) {
    let formatted_json = serde_json::to_string_pretty(&result).unwrap_or_else(|err| {
        return format!("Format json error: {err}");
    });

    println!("{formatted_json}");
}

pub fn input_job_id() -> String {
    let id = Input::with_theme(&ColorfulTheme::default())
        .with_prompt("Job ID")
        .interact_text()
        .unwrap();

    id
}

pub fn confirm_delete() -> bool {
    if Confirm::with_theme(&ColorfulTheme::default())
        .with_prompt("Do you really really want to delete?")
        .default(false)
        .show_default(true)
        .wait_for_newline(true)
        .interact()
        .unwrap()
    {
        return true;
    } else {
        return false;
    }
}
