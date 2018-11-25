# Slamtec RPLIDAR Public SDK for Rust

## Introduction

Slamtec RPLIDAR(https://www.slamtec.com/lidar/a3) series is a set of high-performance and low-cost LIDAR(https://en.wikipedia.org/wiki/Lidar) sensors, which is the perfect sensor of 2D SLAM, 3D reconstruction, multi-touch, and safety applications.

This is the public SDK of RPLIDAR products in Rust, and open-sourced under GPLv3 license.

If you are using ROS (Robot Operating System), please use our open-source ROS node directly: https://github.com/slamtec/rplidar_ros .

If you are just evaluating RPLIDAR, you can use Slamtec RoboStudio(https://www.slamtec.com/robostudio) (currently only support Windows) to do the evaulation.

## Release Notes

* [v0.1.0](https://github.com/slamtec/rplidar_sdk/blob/master/docs/ReleaseNote.v0.1.0.md)

## Supported Platforms

RPLIDAR SDK supports Windows, macOS and Linux by using Visual Studio 2010 projects and Makefile.

| LIDAR Model \ Platform | Windows | macOS | Linux   |
| ---------------------- | ------- | ----- | ------- |
| A1                     | Yes     | Yes   | Yes     |
| A2                     | Yes     | Yes   | Yes     |
| A3                     | Partial | No    | Partial |

## Quick Start

To use RPLIDAR Rust SDK is quite simple:

```rust
extern crate rplidar_drv;
extern crate serialport;

use rplidar_drv::RplidarDevice;

let mut serial_port = serialport::open("COM3").unwrap();
let mut rplidar = RplidarDevice::with_stream(serial_port);

let device_info = rplidar.get_device_info().unwrap();
rplidar.start_scan().unwrap();

while true {
    let scan_point = rplidar.grab_scan_point().unwrap();

    // use the scan point data
}
```
