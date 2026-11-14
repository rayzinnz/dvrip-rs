use dvrip_rs::{Authentication, Connection, DVRIPCam, Monitoring};
use std::time::Duration;
use tokio::fs::File;
use tokio::io::AsyncWriteExt;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = std::env::args().collect();
    if args.len() < 5 {
        println!("Usage: {} <IP> <Username> <Password> <output_filepath>", args[0]);
        return Ok(());
    }

    let ip = &args[1];
    let user = &args[2];
    let pass = &args[3];
	let filename = &args[4];

    let mut cam = DVRIPCam::new(ip);

    cam.connect(Duration::from_secs(5)).await?;
    cam.login(user, pass).await?;

	// let timestamp = chrono::Local::now().format("%Y%m%d_%H%M%S");
	// let filename = format!("snapshot_{}.jpg", timestamp);
	match cam.snapshot(0).await {
		Ok(image_bytes) => {
			let mut file = File::create(&filename).await?;
			file.write_all(&image_bytes).await?;
			println!(
				"  Successfully saved {} ({} bytes)",
				filename,
				image_bytes.len()
			);
		}
		Err(e) => eprintln!("  Failed to take snapshot: {}", e),
	}

    cam.close().await?;
    Ok(())
}
