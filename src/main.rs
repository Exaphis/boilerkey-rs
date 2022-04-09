use serde_derive::{Deserialize, Serialize};
use std::fs::File;
use std::io::Read;
use std::io::Write;

#[derive(Serialize, Deserialize)]
struct HOTPData {
    secret: String,
    counter: u64,
    pin: String,
}

#[derive(Debug, Deserialize)]
#[allow(dead_code)]
struct DuoActivationResponse {
    akey: String,
    app_status: i32,
    current_app_version: String,
    current_os_version: String,
    customer_name: String,
    force_disable_analytics: bool,
    has_backup_restore: bool,
    has_bluetooth_approve: bool,
    has_device_insight: bool,
    has_trusted_endpoints: bool,
    has_trusted_endpoints_permission_flow: bool,
    hotp_secret: String,
    instant_restore_status: String,
    os_status: i32,
    pkey: String,
    reactivation_token: String,
    requires_fips_android: bool,
    requires_mdm: i32,
    security_checkup_enabled: bool,
    urg_secret: String,
}

#[derive(Debug, Deserialize)]
#[allow(dead_code)]
struct DuoActivationResponseWrapper {
    response: DuoActivationResponse,
    stat: String,
}

// https://old.reddit.com/r/rust/comments/ikmufi/idiomatically_handle_multiple_error_types/g3lsq3h/
// https://doc.rust-lang.org/std/keyword.impl.html
#[derive(Debug)]
enum ActivationError {
    ReqwestError(reqwest::Error),
    SerdeJsonError(serde_json::Error),
}

impl From<reqwest::Error> for ActivationError {
    fn from(e: reqwest::Error) -> Self {
        ActivationError::ReqwestError(e)
    }
}

impl From<serde_json::Error> for ActivationError {
    fn from(e: serde_json::Error) -> Self {
        ActivationError::SerdeJsonError(e)
    }
}

async fn activate() -> Result<HOTPData, ActivationError> {
    print!("Enter activation code: ");
    std::io::stdout().flush().unwrap();

    let mut activation_code = String::new();
    std::io::stdin().read_line(&mut activation_code).unwrap();

    println!("Requesting activation data...");
    let client = reqwest::Client::new();
    let res_text = client
        .post(format!(
            "{}{}",
            "https://api-1b9bef70.duosecurity.com/push/v2/activation/", activation_code
        ))
        .header("User-Agent", "okhttp/3.11.0")
        .query(&[
            ("app_id", "com.duosecurity.duomobile.app.DMApplication"),
            ("app_version", "3.37.1"),
            ("app_build_number", "326002"),
            ("full_disk_encryption", "false"),
            ("manufacturer", "Google"),
            ("model", "Pixel4"),
            ("platform", "Android"),
            ("jailbroken", "false"),
            ("version", "10.0"),
            ("language", "EN"),
            ("customer_protocol", "1"),
        ])
        .send()
        .await?
        .text()
        .await?;
    println!("Response: {}", res_text);
    let res = serde_json::from_str::<DuoActivationResponseWrapper>(&res_text)?;

    print!("Enter BoilerKey PIN: ");
    std::io::stdout().flush().unwrap();

    let mut pin = String::new();
    std::io::stdin().read_line(&mut pin).unwrap();

    let hotp_data = HOTPData {
        secret: res.response.hotp_secret,
        counter: 0,
        pin: pin.trim().to_string(),
    };

    return Ok(hotp_data);
}

#[tokio::main]
async fn main() {
    // if hotp_data.json exists, read data from the file
    // otherwise, request activation code and save data to hotp_data.json
    let mut hotp_data: HOTPData = match File::open("hotp_data.json") {
        Ok(mut file) => {
            println!("Reading hotp_data.json...");
            let mut contents = String::new();
            file.read_to_string(&mut contents).unwrap();
            serde_json::from_str::<HOTPData>(&contents).unwrap()
        }
        Err(_) => {
            let hotp_data = match activate().await {
                Ok(hotp_data) => hotp_data,
                Err(e) => panic!("Activation error: {:?}", e),
            };
            hotp_data
        }
    };

    let key = hotp_data.secret.to_owned();
    let hotp = libreauth::oath::HOTPBuilder::new()
        .ascii_key(&key)
        .counter(hotp_data.counter)
        .finalize()
        .unwrap();

    let code = hotp.generate();
    println!("{},{}", hotp_data.pin, code);

    // update and write hotp_data to file
    hotp_data.counter += 1;

    let mut file = File::create("hotp_data.json").unwrap();
    serde_json::to_writer(&mut file, &hotp_data).unwrap();
}
