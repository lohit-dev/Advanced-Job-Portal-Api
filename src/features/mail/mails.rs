use crate::features::mail::send::send_email;
use std::{env, path::PathBuf};

pub async fn send_verification_email(
    to_email: &str,
    username: &str,
    token: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    let subject = "Email Verification";
    let base_path = get_base_template_path().expect("unable to get base path in the mails.rs");
    let template_path = format!(
        "{:?}/src/features/mail/templates/verify-mail.html",
        base_path
    );

    let base_url = "https://e-commerce-backend-rs.onrender.com/api/auth/verify";
    let verification_link = create_verification_link(base_url, token);
    let placeholders = vec![
        ("{{username}}".to_string(), username.to_string()),
        ("{{verification_link}}".to_string(), verification_link),
    ];

    send_email(to_email, subject, template_path, &placeholders).await
}

fn create_verification_link(base_url: &str, token: &str) -> String {
    format!("{}?token={}", base_url, token)
}

pub async fn send_welcome_email(
    to_email: &str,
    username: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    let subject = "Welcome to Application";
    let base_path = get_base_template_path().expect("unable to get base path in the mails.rs");
    let template_path = format!(
        "{:?}/src/features/mail/templates/on-boarding.html",
        base_path
    );
    let placeholders = vec![("{{username}}".to_string(), username.to_string())];

    send_email(to_email, subject, template_path, &placeholders).await
}

pub async fn send_forgot_password_email(
    to_email: &str,
    reset_link: &str,
    username: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    let subject = "Rest your Password";

    let base_path = get_base_template_path().expect("unable to get base path in the mails.rs");
    let template_path = format!(
        "{:?}/src/features/mail/templates/reset-pass.html",
        base_path
    );
    let placeholders = vec![
        ("{{username}}".to_string(), username.to_string()),
        ("{{rest_link}}".to_string(), reset_link.to_string()),
    ];

    send_email(to_email, subject, template_path, &placeholders).await
}

pub fn get_base_template_path() -> Option<PathBuf> {
    match env::current_dir() {
        Ok(p) => Some(p),
        Err(e) => {
            eprintln!("Error getting current dir: {}", e);
            None
        }
    }
}
