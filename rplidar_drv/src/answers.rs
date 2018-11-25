pub const RPLIDAR_ANS_TYPE_DEVINFO : u8 = 0x4;
pub const RPLIDAR_ANS_TYPE_DEVHEALTH : u8 = 0x6;

pub const RPLIDAR_ANS_TYPE_MEASUREMENT : u8 = 0x81;
// Added in FW ver 1.17
pub const RPLIDAR_ANS_TYPE_MEASUREMENT_CAPSULED : u8 = 0x82;
pub const RPLIDAR_ANS_TYPE_MEASUREMENT_HQ : u8 = 0x83;


// Added in FW ver 1.17
// pub const RPLIDAR_ANS_TYPE_SAMPLE_RATE : u8 = 0x15;
//added in FW ver 1.23alpha
pub const RPLIDAR_ANS_TYPE_MEASUREMENT_CAPSULED_ULTRA : u8 = 0x84;
//added in FW ver 1.24
pub const RPLIDAR_ANS_TYPE_GET_LIDAR_CONF : u8 = 0x20;
// pub const RPLIDAR_ANS_TYPE_SET_LIDAR_CONF : u8 = 0x21;

// pub const RPLIDAR_ANS_TYPE_ACC_BOARD_FLAG : u8 = 0xFF;

// pub const RPLIDAR_RESP_ACC_BOARD_FLAG_MOTOR_CTRL_SUPPORT_MASK : u8 = (0x1);

pub const RPLIDAR_RESP_MEASUREMENT_SYNCBIT : u8 = 1;
pub const RPLIDAR_RESP_MEASUREMENT_QUALITY_SHIFT : usize = 2;
pub const RPLIDAR_RESP_HQ_FLAG_SYNCBIT : u8 = 1;
pub const RPLIDAR_RESP_MEASUREMENT_CHECKBIT : u8 = 1;
pub const RPLIDAR_RESP_MEASUREMENT_ANGLE_SHIFT : usize = 1;
pub const RPLIDAR_RESP_MEASUREMENT_EXP_SYNC_1 : u8 = 0xA;
pub const RPLIDAR_RESP_MEASUREMENT_EXP_SYNC_2 : u8 = 0x5;

/// Rplidar device info data strcture
#[derive(Debug, Clone, PartialEq)]
#[repr(packed)]
#[repr(C)]
pub struct RplidarResponseDeviceInfo {
    pub model: u8,
    pub firmware_version: u16,
    pub hardware_version: u8,
    pub serialnum: [u8;16]
}

#[derive(Debug, Clone, PartialEq)]
#[repr(packed)]
#[repr(C)]
pub struct RplidarResponseMeasurementNode {
    pub sync_quality: u8,
    pub angle_q6_checkbit: u16,
    pub distance_q2: u16,
}

#[derive(Debug, Clone, PartialEq)]
#[repr(packed)]
#[repr(C)]
pub struct RplidarResponseCabinNodes {
    pub distance_angle_1: u16,
    pub distance_angle_2: u16,
    pub offset_angles_q3: u8
}

#[derive(Debug, Clone, PartialEq)]
#[repr(packed)]
#[repr(C)]
pub struct RplidarResponseCapsuleMeasurementNodes {
    pub s_checksum_1: u8,
    pub s_checksum_2: u8,
    pub start_angle_sync_q6: u16,
    pub cabins: [RplidarResponseCabinNodes;16],
}

#[derive(Debug, Clone, PartialEq)]
#[repr(packed)]
#[repr(C)]
pub struct RplidarResponseUltraCapsuleMeasurementNodes {
    pub s_checksum_1: u8,
    pub s_checksum_2: u8,
    pub start_angle_sync_q6: u16,
    pub ultra_cabins: [u32;32],
}

#[derive(Debug, Clone, PartialEq)]
#[repr(packed)]
#[repr(C)]
pub struct RplidarResponseMeasurementNodeHq {
    pub angle_z_q14: u16,
    pub dist_mm_q2: u32,
    pub quality: u8,
    pub flag: u8,
}
