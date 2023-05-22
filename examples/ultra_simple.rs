extern crate hex_slice;
extern crate rplidar_drv;
extern crate rpos_drv;
extern crate serialport;

use hex_slice::AsHex;

use rplidar_drv::{Health, RplidarDevice, RplidarHostProtocol};
use rpos_drv::{Channel, RposError};
use std::time::Duration;

use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 || args.len() > 3 {
        println!("Usage: {} <serial_port> [baudrate]", args[0]);
        println!("    baudrate defaults to 115200");
        return;
    }

    let serial_port = &args[1];
    let baud_rate = args.get(2).unwrap_or(&String::from("115200"))
        .parse::<u32>()
        .expect("Invalid value for baudrate");

    let mut serial_port = serialport::new(serial_port, baud_rate)
    .data_bits(serialport::DataBits::Eight)
    .flow_control(serialport::FlowControl::None)
    .parity(serialport::Parity::None)
    .stop_bits(serialport::StopBits::One)
        .timeout(Duration::from_millis(100))
        .open()
        .expect("failed to open serial port");


    serial_port
        .write_data_terminal_ready(false)
        .expect("failed to clear DTR");

    let channel = Channel::<RplidarHostProtocol, dyn serialport::SerialPort>::new(
        RplidarHostProtocol::new(),
        serial_port,
    );

    let mut rplidar = RplidarDevice::new(channel);

    let device_info = rplidar
        .get_device_info()
        .expect("failed to get device info");

    println!("Connected to LIDAR: ");
    println!("    Model: {}", device_info.model);
    println!(
        "    Firmware Version: {}.{}",
        device_info.firmware_version >> 8,
        device_info.firmware_version & 0xff
    );
    println!("    Hardware Version: {}", device_info.hardware_version);
    println!(
        "    Serial Number: {:02X}",
        device_info.serialnum.plain_hex(false)
    );

    let device_health = rplidar
        .get_device_health()
        .expect("failed to get device health");

    match device_health {
        Health::Healthy => {
            println!("LIDAR is healthy.");
        }
        Health::Warning(error_code) => {
            println!("LIDAR is unhealthy, warn: {:04X}", error_code);
        }
        Health::Error(error_code) => {
            println!("LIDAR is unhealthy, error: {:04X}", error_code);
        }
    }

    let all_supported_scan_modes = rplidar
        .get_all_supported_scan_modes()
        .expect("failed to get all supported scan modes");

    println!("All supported scan modes:");
    for scan_mode in all_supported_scan_modes {
        println!(
            "    {:2} {:16}: Max Distance: {:6.2}m, Ans Type: {:02X}, Us per sample: {:.2}us",
            scan_mode.id,
            scan_mode.name,
            scan_mode.max_distance,
            scan_mode.ans_type,
            scan_mode.us_per_sample
        );
    }

    let typical_scan_mode = rplidar
        .get_typical_scan_mode()
        .expect("failed to get typical scan mode");

    println!("Typical scan mode: {}", typical_scan_mode);

    match rplidar.check_motor_ctrl_support() {
        Ok(support) if support == true => {
            println!("Accessory board is detected and support motor control, starting motor...");
            rplidar.set_motor_pwm(600).expect("failed to start motor");
        },
        Ok(_) => {
            println!("Accessory board is detected, but doesn't support motor control");
        },
        Err(_) => {
            println!("Accessory board isn't detected");
        }
    }

    println!("Starting LIDAR in typical mode...");

    let actual_mode = rplidar
        .start_scan()
        .expect("failed to start scan in standard mode");

    println!("Started scan in mode `{}`", actual_mode.name);

    let start_time = std::time::Instant::now();

    loop {
        match rplidar.grab_scan() {
            Ok(scan) => {
                println!("[{:6}s] {} points per scan", start_time.elapsed().as_secs(), scan.len());

                /*
                 for scan_point in scan {
                    println!(
                        "    Angle: {:5.2}, Distance: {:8.4}, Valid: {:5}, Sync: {:5}",
                        scan_point.angle(),
                        scan_point.distance(),
                        scan_point.is_valid(),
                        scan_point.is_sync()
                    )
                }*/
            }
            Err(err) => {
                if let Some(RposError::OperationTimeout) = err.downcast_ref::<RposError>() {
                    continue;
                } else {
                    println!("Error: {:?}", err);
                    break;
                }
            }
        }
    }
}
