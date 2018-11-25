// Commands without payload and response
pub const RPLIDAR_CMD_STOP : u8 = 0x25;
pub const RPLIDAR_CMD_SCAN : u8 = 0x20;
pub const RPLIDAR_CMD_FORCE_SCAN : u8 = 0x21;
pub const RPLIDAR_CMD_RESET : u8 = 0x40;


// Commands without payload but have response
pub const RPLIDAR_CMD_GET_DEVICE_INFO : u8 = 0x50;
pub const RPLIDAR_CMD_GET_DEVICE_HEALTH : u8 = 0x52;

pub const RPLIDAR_CMD_GET_SAMPLERATE : u8 = 0x59; //added in fw 1.17

pub const RPLIDAR_CMD_HQ_MOTOR_SPEED_CTRL : u8 = 0xA8;

// Commands with payload and have response
pub const RPLIDAR_CMD_EXPRESS_SCAN : u8 = 0x82; //added in fw 1.17;

#[repr(packed)]
#[repr(C)]
pub struct RplidarPayloadExpressScan {
    pub work_mode: u8,
    pub work_flags: u16,
    pub param: u16
}

pub const RPLIDAR_CMD_HQ_SCAN : u8 = 0x83; //added in fw 1.24;
pub const RPLIDAR_CMD_GET_LIDAR_CONF : u8 = 0x84; //added in fw 1.24;
pub const RPLIDAR_CMD_SET_LIDAR_CONF : u8 = 0x85; //added in fw 1.24;

//add for A2 to set RPLIDAR motor pwm when using accessory board
pub const RPLIDAR_CMD_SET_MOTOR_PWM : u8 = 0xF0;
pub const RPLIDAR_CMD_GET_ACC_BOARD_FLAG : u8 = 0xFF;

pub const RPLIDAR_CONF_SCAN_MODE_COUNT: u32 = 0x00000070;
pub const RPLIDAR_CONF_SCAN_MODE_US_PER_SAMPLE: u32 = 0x00000071;
pub const RPLIDAR_CONF_SCAN_MODE_MAX_DISTANCE: u32 = 0x00000074;
pub const RPLIDAR_CONF_SCAN_MODE_ANS_TYPE: u32 = 0x00000075;
pub const RPLIDAR_CONF_SCAN_MODE_TYPICAL: u32 = 0x0000007C;
pub const RPLIDAR_CONF_SCAN_MODE_NAME: u32 = 0x0000007F;
