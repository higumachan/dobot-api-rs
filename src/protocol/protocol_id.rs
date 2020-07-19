const PROTOCOL_FUNCTION_DEVICE_INFO_BASE: u8 = 0;
const PROTOCOL_FUNCTION_POSE_BASE: u8 = 10;
const PROTOCOL_FUNCTIONAL_ARM_BASE: u8 = 20;
const PROTOCOL_FUNCTION_HOME_BASE: u8 = 30;
const PROTOCOL_FUNCTION_HHT_BASE: u8 = 40;
const PROTOCOL_FUNCTION_ARM_ORIENTATION_BASE: u8 = 50;
const PROTOCOL_FUNCTION_END_EFFECTOR_BASE: u8 = 60;
const PROTOCOL_FUNCTION_JOG_BASE: u8 = 70;
const PROTOCOL_FUNCTION_PTP_BASE: u8 = 80;
const PROTOCOL_FUNCTION_CP_BASE: u8 = 90;
const PROTOCOL_FUNCTION_ARC_BASE: u8 = 100;
const PROTOCOL_FUNCTION_WAIT_BASE: u8 = 110;
const PROTOCOL_FUNCTION_TRIG_BASE: u8 = 120;
const PROTOCOL_FUNCTION_EIO_BASE: u8 = 130;
const PROTOCOL_FUNCTION_CAL_BASE: u8 = 140;
const PROTOCOL_FUNCTION_WIFI_BASE: u8 = 150;
const PROTOCOL_FUNCTION_FIRMWARE_BASE: u8 = 160;
const PROTOCOL_FUNCTION_LOST_STEP_BASE: u8 = 170;
const PROTOCOL_FUNCTION_CHECK_MODEL_BASE: u8 = 180;
const PROTOCOL_FUNCTION_PULSE_MODE_BASE: u8 = 190;
const PROTOCOL_TEST_BASE: u8 = 220;

const PROTOCOL_FUNCTION_QUEUED_CMD_BASE: u8 = 240;

#[repr(u8)]
pub enum ProtocolID {
    // Device information
    ProtocolDeviceSN = PROTOCOL_FUNCTION_DEVICE_INFO_BASE + 0,
    ProtocolDeviceName = PROTOCOL_FUNCTION_DEVICE_INFO_BASE + 1,
    ProtocolDeviceVersion = PROTOCOL_FUNCTION_DEVICE_INFO_BASE + 2,
    ProtocolDeviceWithL = PROTOCOL_FUNCTION_DEVICE_INFO_BASE + 3,
    ProtocolDeviceTime = PROTOCOL_FUNCTION_DEVICE_INFO_BASE + 4,

    // Pose
    ProtocolGetPose = PROTOCOL_FUNCTION_POSE_BASE + 0,
    ProtocolResetPose = PROTOCOL_FUNCTION_POSE_BASE + 1,
    ProtocolGetKinematics = PROTOCOL_FUNCTION_POSE_BASE + 2,
    ProtocolGetPoseL = PROTOCOL_FUNCTION_POSE_BASE + 3,

    // Alarm
    ProtocolAlarmsState = PROTOCOL_FUNCTIONAL_ARM_BASE + 0,

    // HOME
    ProtocolHOMEParams = PROTOCOL_FUNCTION_HOME_BASE + 0,
    ProtocolHOMECmd = PROTOCOL_FUNCTION_HOME_BASE + 1,
    ProtocolAutoLeveling = PROTOCOL_FUNCTION_HOME_BASE + 2,

    // HHT
    ProtocolHHTTrigMode = PROTOCOL_FUNCTION_HHT_BASE + 0,
    ProtocolHHTTrigOutputEnabled = PROTOCOL_FUNCTION_HHT_BASE + 1,
    ProtocolHHTTrigOutput = PROTOCOL_FUNCTION_HHT_BASE + 2,

    // Function-Arm Orientation
    ProtocolArmOrientation = PROTOCOL_FUNCTION_ARM_ORIENTATION_BASE + 0,

    // End effector
    ProtocolEndEffectorParams = PROTOCOL_FUNCTION_END_EFFECTOR_BASE + 0,
    ProtocolEndEffectorLaser = PROTOCOL_FUNCTION_END_EFFECTOR_BASE + 1,
    ProtocolEndEffectorSuctionCup = PROTOCOL_FUNCTION_END_EFFECTOR_BASE + 2,
    ProtocolEndEffectorGripper = PROTOCOL_FUNCTION_END_EFFECTOR_BASE + 3,

    // Function-JOG
    ProtocolJOGJointParams = PROTOCOL_FUNCTION_JOG_BASE + 0,
    ProtocolJOGCoordinateParams = PROTOCOL_FUNCTION_JOG_BASE + 1,
    ProtocolJOGCommonParams = PROTOCOL_FUNCTION_JOG_BASE + 2,
    ProtocolJOGCmd = PROTOCOL_FUNCTION_JOG_BASE + 3,

    ProtocolJOGLParams = PROTOCOL_FUNCTION_JOG_BASE + 4,

    // Function-PTP

    ProtocolPTPJointParams = PROTOCOL_FUNCTION_PTP_BASE + 0,
    ProtocolPTPCoordinateParams = PROTOCOL_FUNCTION_PTP_BASE + 1,
    ProtocolPTPJumpParams = PROTOCOL_FUNCTION_PTP_BASE + 2,
    ProtocolPTPCommonParams = PROTOCOL_FUNCTION_PTP_BASE + 3,
    ProtocolPTPCmd = PROTOCOL_FUNCTION_PTP_BASE + 4,

    ProtocolPTPLParams = PROTOCOL_FUNCTION_PTP_BASE + 5,
    ProtocolPTPWithLCmd = PROTOCOL_FUNCTION_PTP_BASE + 6,
    ProtocolPTPJump2Params = PROTOCOL_FUNCTION_PTP_BASE + 7,

    ProtocolPTPPOCmd = PROTOCOL_FUNCTION_PTP_BASE + 8,
    ProtocolPTPPOWithLCmd = PROTOCOL_FUNCTION_PTP_BASE + 9,

    // Function-CP

    ProtocolCPParams = PROTOCOL_FUNCTION_CP_BASE + 0,
    ProtocolCPCmd = PROTOCOL_FUNCTION_CP_BASE + 1,
    ProtocolCPLECmd = PROTOCOL_FUNCTION_CP_BASE + 2,
    ProtocolCPRHoldEnable = PROTOCOL_FUNCTION_CP_BASE + 3,
    ProtocolCPCommonParams = PROTOCOL_FUNCTION_CP_BASE + 4,

    // Function-ARC
    ProtocolARCParams = PROTOCOL_FUNCTION_ARC_BASE + 0,
    ProtocolARCCmd = PROTOCOL_FUNCTION_ARC_BASE + 1,
    ProtocolCircleCmd = PROTOCOL_FUNCTION_ARC_BASE + 2,
    ProtocolARCCommonParams = PROTOCOL_FUNCTION_ARC_BASE + 3,

    // Function-WAIT
    ProtocolWAITCmd = PROTOCOL_FUNCTION_WAIT_BASE + 0,

    // Function-TRIG
    ProtocolTRIGCmd = PROTOCOL_FUNCTION_TRIG_BASE + 0,

    // Function-EIO

    ProtocolIOMultiplexing = PROTOCOL_FUNCTION_EIO_BASE + 0,
    ProtocolIODO = PROTOCOL_FUNCTION_EIO_BASE + 1,
    ProtocolIOPWM = PROTOCOL_FUNCTION_EIO_BASE + 2,
    ProtocolIODI = PROTOCOL_FUNCTION_EIO_BASE + 3,
    ProtocolIOADC = PROTOCOL_FUNCTION_EIO_BASE + 4,
    ProtocolEMotor = PROTOCOL_FUNCTION_EIO_BASE + 5,
    ProtocolEMotorS = PROTOCOL_FUNCTION_EIO_BASE + 6,
    ProtocolColorSensor = PROTOCOL_FUNCTION_EIO_BASE + 7,
    ProtocolIRSwitch = PROTOCOL_FUNCTION_EIO_BASE + 8,

    // Function-CAL
    ProtocolAngleSensorStaticError = PROTOCOL_FUNCTION_CAL_BASE + 0,
    ProtocolAngleSensorCoef = PROTOCOL_FUNCTION_CAL_BASE + 1,
    ProtocolBaseDecoderStaticError = PROTOCOL_FUNCTION_CAL_BASE + 2,
    ProtocolLRHandCalibrateValue = PROTOCOL_FUNCTION_CAL_BASE + 3,

    // Function-WIFI
    ProtocolWIFIConfigMode = PROTOCOL_FUNCTION_WIFI_BASE + 0,
    ProtocolWIFISSID = PROTOCOL_FUNCTION_WIFI_BASE + 1,
    ProtocolWIFIPassword = PROTOCOL_FUNCTION_WIFI_BASE + 2,
    ProtocolWIFIIPAddress = PROTOCOL_FUNCTION_WIFI_BASE + 3,
    ProtocolWIFINetmask = PROTOCOL_FUNCTION_WIFI_BASE + 4,
    ProtocolWIFIGateway = PROTOCOL_FUNCTION_WIFI_BASE + 5,
    ProtocolWIFIDNS = PROTOCOL_FUNCTION_WIFI_BASE + 6,
    ProtocolWIFIConnectStatus = PROTOCOL_FUNCTION_WIFI_BASE + 7,

    // Function-Firmware
    ProtocolFirmwareSwitch = PROTOCOL_FUNCTION_FIRMWARE_BASE + 0,
    ProtocolFirmwareMode = PROTOCOL_FUNCTION_FIRMWARE_BASE + 1,

    // Function-LostStep
    ProtocolLostStepSet = PROTOCOL_FUNCTION_LOST_STEP_BASE + 0,
    ProtocolLostStepDetect = PROTOCOL_FUNCTION_LOST_STEP_BASE + 1,

    //UART4 Peripherals
    ProtocolCheckUART4PeripheralsModel = PROTOCOL_FUNCTION_CHECK_MODEL_BASE + 1,
    ProtocolUART4PeripheralsEnabled = PROTOCOL_FUNCTION_CHECK_MODEL_BASE + 2,

    //Function Pluse Mode
    ProtocolFunctionPulseMode = PROTOCOL_FUNCTION_PULSE_MODE_BASE + 1,

    // Function-TEST
    ProtocolUserParams = PROTOCOL_TEST_BASE + 0,
    ProtocolPTPTime = PROTOCOL_TEST_BASE + 1,
    ProtocolServoPIDParams = PROTOCOL_TEST_BASE + 2,
    ProtocolServoControlLoop = PROTOCOL_TEST_BASE + 3,
    ProtocolSaveServoPIDParams = PROTOCOL_TEST_BASE + 4,

    // Function-QueuedCmd
    ProtocolQueuedCmdStartExec = PROTOCOL_FUNCTION_QUEUED_CMD_BASE + 0,
    ProtocolQueuedCmdStopExec = PROTOCOL_FUNCTION_QUEUED_CMD_BASE  + 1,
    ProtocolQueuedCmdForceStopExec = PROTOCOL_FUNCTION_QUEUED_CMD_BASE + 2,
    ProtocolQueuedCmdStartDownload = PROTOCOL_FUNCTION_QUEUED_CMD_BASE + 3,
    ProtocolQueuedCmdStopDownload = PROTOCOL_FUNCTION_QUEUED_CMD_BASE  + 4,
    ProtocolQueuedCmdClear = PROTOCOL_FUNCTION_QUEUED_CMD_BASE + 5,
    ProtocolQueuedCmdCurrentIndex = PROTOCOL_FUNCTION_QUEUED_CMD_BASE  + 6,
    ProtocolQueuedCmdLeftSpace = PROTOCOL_FUNCTION_QUEUED_CMD_BASE + 7,
}
