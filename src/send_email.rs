use lettre::Transport;

#[derive(serde::Deserialize, Clone)]
pub enum EmailPresets {
    Submit,
    Removed,
    ChangedState,
}

#[derive(serde::Deserialize, Clone)]
pub struct EmailConfig {
    pub email: String,
    pub password: String,
    submit_subject: String,
    submit: String,
    removed_subject: String,
    removed: String,
    changed_state_subject: String,
    changed_state: String,
}

/// Sends an email to a pre configured email address
pub fn send_email(email: String, config: &EmailConfig, preset: EmailPresets) {
    let subject = match preset {
        EmailPresets::Submit => &config.submit_subject,
        EmailPresets::ChangedState => &config.changed_state_subject,
        EmailPresets::Removed => &config.removed_subject,
    };

    let email_content = match preset {
        EmailPresets::Submit => &config.submit,
        EmailPresets::ChangedState => &config.changed_state,
        EmailPresets::Removed => &config.removed,
    };
    // Create the email
    let lettre_email = lettre::Message::builder()
        .from(config.email.parse().unwrap())
        .to(email.parse().unwrap())
        .subject(subject)
        .body(email_content.clone())
        .unwrap();

    let creds = lettre::transport::smtp::authentication::Credentials::new(
        config.email.clone(),
        config.password.clone(),
    );

    // Create email mailer
    let mailer = lettre::SmtpTransport::relay("smtp.web.de")
        .unwrap()
        .credentials(creds)
        .build();

    // Send email to given email adress
    match mailer.send(&lettre_email) {
        Ok(_) => println!("Email sent successfully!"),
        Err(e) => println!("Could not send email: {}", e),
    }
}
