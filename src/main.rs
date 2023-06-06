use lettre::{
    message::header::ContentType, transport::smtp::authentication::Credentials, Message,
    SmtpTransport, Transport,
};
use serde::Deserialize;
use std::time::Duration;

const TEMP_THRESH: f32 = 10.0;

// #[tokio::main]
fn main() -> anyhow::Result<()> {
    // Prepare api url to Ambient
    let request_url = format!(
        "http://ambidata.io/api/v2/channels/{}/data?&readKey={}&n={}",
        std::env::var("AMBIENT_CHANNEL_ID")?,
        std::env::var("AMBIENT_READ_KEY")?,
        1
    );

    // Set up mailer
    let sender = std::env::var("SENDER")?;
    let recipient = std::env::var("RECIPIENT")?;
    let password = std::env::var("GMAIL_APP_PASSWORD")?;

    let creds = Credentials::new(sender.clone(), password.clone());

    let mailer = SmtpTransport::relay("smtp.gmail.com")?
        .credentials(creds)
        .build();

    loop {
        println!("Sending request...");
        let response = reqwest::blocking::get(&request_url)?;

        if let Some(data) = response.json::<Vec<M5StackData>>()?.pop() {
            if data.d1 > TEMP_THRESH {
                println!("Sending email...");

                let email = Message::builder()
                    .from(format!("Me <{}>", &sender).parse()?)
                    .to(format!("Also Me <{}>", &recipient).parse()?)
                    .subject("🔥High Temperature Alert🔥")
                    .header(ContentType::TEXT_PLAIN)
                    .body(format!(
                        "Too hot💀 \n\n
                        Temperature: {} deg. \n
                        Humidity: {} % \n
                        Timestamp: {}
                        ",
                        data.d1, data.d2, data.created
                    ))?;

                match mailer.send(&email) {
                    Ok(_) => println!("Email sent successfully"),
                    Err(e) => eprintln!("Could not send email: {:?}", e),
                }
            }
        }

        std::thread::sleep(Duration::from_secs(60));
    }
}

#[derive(Debug, Deserialize)]
struct M5StackData {
    d1: f32,         // temperature
    d2: f32,         // humidity
    created: String, // timestamp
}
