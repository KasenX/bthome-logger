mod parser;

use std::path::PathBuf;

use bluer::{
    DeviceEvent, DeviceProperty,
    monitor::{Monitor, MonitorEvent, Pattern},
};
use clap::Parser;
use futures::StreamExt;
use uuid::Uuid;

use crate::{config, db};

// https://bthome.io/format
const SERVICE_DATA_UUID_16: u8 = 0x16;
const START_POSITION: u8 = 0x00;
const BTHOME_PATTERN: [u8; 2] = [0xd2, 0xfc];

// BTHome Data format UUID: 0xFCD2
// Bluetooth Base UUID: xxxxxxxx-0000-1000-8000-00805F9B34FB
const BTHOME_UUID: Uuid = Uuid::from_u128(0x0000_fcd2_0000_1000_8000_0080_5f9b_34fb);

#[derive(Debug, clap::Parser)]
#[command(
    name = "BTHome Logger",
    about,
    long_about = None,
    version = concat!("git:", env!("VERGEN_GIT_SHA")),
    max_term_width = 120)]
struct Cli {
    #[arg(long)]
    config: PathBuf,
}

pub async fn run_main() -> anyhow::Result<()> {
    let cli = Cli::parse();
    let config = config::read(&cli.config).expect("error parsing config");

    let db = db::Db::connect(config.database_url.as_str()).await?;

    let pattern = Pattern {
        data_type: SERVICE_DATA_UUID_16,
        start_position: START_POSITION,
        content: BTHOME_PATTERN.to_vec(),
    };

    let session = bluer::Session::new().await?;
    let adapter = session.default_adapter().await?;
    println!(
        "Starting scan on Bluetooth adapter {} with pattern {:?}",
        adapter.name(),
        pattern
    );

    adapter.set_powered(true).await?;

    let monitor = adapter.monitor().await?;
    let mut monitor_handle = monitor
        .register(Monitor {
            monitor_type: bluer::monitor::Type::OrPatterns,
            patterns: Some(vec![pattern]),
            ..Default::default()
        })
        .await?;

    while let Some(monitor_event) = &monitor_handle.next().await {
        // MonitorEvent::DeviceFound is triggered every time the device advertises again (e.g. service data or RSSI changes)
        if let MonitorEvent::DeviceFound(device_id) = monitor_event {
            let dev = adapter.device(device_id.device)?;
            let mut events = dev.events().await?;
            if let Some(DeviceEvent::PropertyChanged(DeviceProperty::ServiceData(service_data))) =
                events.next().await
            {
                let bthome_data = service_data
                    .get(&BTHOME_UUID)
                    .ok_or_else(|| anyhow::anyhow!("No BTHome service data found"))?;

                if let Some(sample) = parser::parse_bthome_service_data(bthome_data) {
                    db::queries::insert_sample(
                        db.pool(),
                        &dev.address().to_string(),
                        sample.packet_counter,
                        sample.temperature,
                        sample.humidity,
                        sample.battery,
                    )
                    .await?;

                    println!("On device {:?}, received event {:?}", dev, sample,);
                }
            }
        }
    }

    Ok(())
}
