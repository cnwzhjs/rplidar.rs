# Roadmap for Slamtec RPLIDAR Public SDK for Rust

## Feature Status

| Feature                                | Availability |
| -------------------------------------- | ------------ |
| feature - get_all_supported_scan_modes | since 0.1.0  |
| feature - start_scan                   | since 0.1.0  |
| feature - get_health                   | No           |
| feature - get_device_info              | since 0.1.0  |
| feature - set_motor_pwm                | since 0.1.0  |
| feature - stop_motor                   | No           |
| feature - start_motor                  | No           |
| feature - check_motor_ctrl_support     | No           |
| feature - stop                         | since 0.1.0  |
| feature - grab_scan                    | No           |
| feature - grab_scan_point              | since 0.1.0  |
| feature - sort_scan                    | No           |
| protocol - measurement_nodes           | since 0.1.0  |
| protocol - capsuled_nodes              | since 0.1.0  |
| protocol - ultra_capsuled_nodes        | No           |
| protocol - hq_nodes                    | since 0.1.0  |
| back compatibility - start_scan        | No           |
| back compatibility - scan_modes        | No           |

## Roadmap

### v0.2.0

* **Scheduled at:** 2 Dec 2018
* feature - get_health
* feature - stop_motor
* feature - start_motor
* feature - grab_scan

### v0.3.0

* **Scheduled at:** 9 Dec 2018
* protocol - hq_nodes

### v0.4.0

* **Scheduled at:** 16 Dec 2018
* feature - check_motor_ctrl_support
* back compatibility - start_scan
* back compatibility - scan_modes

### v0.5.0

* **Scheduled at:** 23 Dec 2018
* feature - sort_scan
