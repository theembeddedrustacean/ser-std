/*
Simplified Embedded Rust: ESP Standard Library Edition
Progamming IoT & Networking Services - Simple HTTP Client Application Example
*/

use esp_idf_svc::eventloop::EspSystemEventLoop;
use esp_idf_svc::hal::peripherals::Peripherals;
use esp_idf_svc::http::client::{Configuration as HttpConfig, EspHttpConnection};
use esp_idf_svc::http::Method;
use esp_idf_svc::nvs::EspDefaultNvsPartition;
use esp_idf_svc::wifi::{AuthMethod, BlockingWifi, ClientConfiguration, Configuration, EspWifi};

fn main() -> anyhow::Result<()> {
    esp_idf_svc::sys::link_patches();

    // Configure Wifi
    let peripherals = Peripherals::take().unwrap();
    let sysloop = EspSystemEventLoop::take()?;
    let nvs = EspDefaultNvsPartition::take()?;

    let mut wifi = BlockingWifi::wrap(
        EspWifi::new(peripherals.modem, sysloop.clone(), Some(nvs))?,
        sysloop,
    )?;
    //comment
    wifi.set_configuration(&Configuration::Client(ClientConfiguration {
        ssid: "Wokwi-GUEST".try_into().unwrap(),
        bssid: None,
        auth_method: AuthMethod::None,
        password: "".try_into().unwrap(),
        channel: None,
    }))?;

    // Start Wifi
    wifi.start()?;

    // Connect Wifi
    wifi.connect()?;

    // Wait until the network interface is up
    wifi.wait_netif_up()?;

    // Print Out Wifi Connection Configuration
    while !wifi.is_connected().unwrap() {
        // Get and print connection configuration
        let config = wifi.get_configuration().unwrap();
        println!("Waiting for station {:?}", config);
    }

    println!("Wifi Connected, Intiatlizing HTTP");

    // HTTP Configuration
    // Create HTTPS Connection Handle
    let mut httpconnection = EspHttpConnection::new(&HttpConfig {
        use_global_ca_store: true,
        crt_bundle_attach: Some(esp_idf_svc::sys::esp_crt_bundle_attach),
        ..Default::default()
    })?;

    // HTTP Request Submission
    // Define URL
    let url = "https://httpbin.org/get";

    // Log URL and type of request
    println!("-> GET {}", url);

    // Initiate Request
    let _connection = httpconnection.initiate_request(Method::Get, url, &[])?;

    // Initiate Response
    let _response = httpconnection.initiate_response()?;

    // HTTP Response Processing
    let status = httpconnection.status();
    println!("<- {}", status);

    match httpconnection.header("Content-Length") {
        Some(data) => {
            println!("Content-Length: {}", data);
        }
        None => {
            println!("No Content-Length Header");
        }
    }
    match httpconnection.header("Date") {
        Some(data) => {
            println!("Date: {}", data);
        }
        None => {
            println!("No Date Header");
        }
    }

    Ok(())
}
