const PROTOCOL_PTP_JOINT_PARAMS: u8 = 80;
const PROTOCOL_FUNCTION_QUEUED_CMD_BASE: u8 = 240;


#[repr(u8)]
pub enum ProtocolID {
    ProtocolPTPJointParams = PROTOCOL_PTP_JOINT_PARAMS + 0,
    ProtocolPTPCoordinateParams = PROTOCOL_PTP_JOINT_PARAMS + 1,
    ProtocolPTPJumpParams = PROTOCOL_PTP_JOINT_PARAMS + 2,
    ProtocolPTPCommonParams = PROTOCOL_PTP_JOINT_PARAMS + 3,
    ProtocolPTPCmd = PROTOCOL_PTP_JOINT_PARAMS + 4,

    ProtocolPTPLParams = PROTOCOL_PTP_JOINT_PARAMS + 5,
    ProtocolPTPWithLCmd = PROTOCOL_PTP_JOINT_PARAMS + 6,
    ProtocolPTPJump2Params = PROTOCOL_PTP_JOINT_PARAMS + 7,

    ProtocolPTPPOCmd = PROTOCOL_PTP_JOINT_PARAMS + 8,
    ProtocolPTPPOWithLCmd = PROTOCOL_PTP_JOINT_PARAMS + 9,

    ProtocolQueuedCmdStartExec = PROTOCOL_FUNCTION_QUEUED_CMD_BASE + 0,
    ProtocolQueuedCmdStopExec = PROTOCOL_FUNCTION_QUEUED_CMD_BASE + 1,
    ProtocolQueuedCmdForceStopExec = PROTOCOL_FUNCTION_QUEUED_CMD_BASE + 2,
    ProtocolQueuedCmdStartDownload = PROTOCOL_FUNCTION_QUEUED_CMD_BASE + 3,
    ProtocolQueuedCmdStopDownload = PROTOCOL_FUNCTION_QUEUED_CMD_BASE + 4,
    ProtocolQueuedCmdClear = PROTOCOL_FUNCTION_QUEUED_CMD_BASE + 5,
    ProtocolQueuedCmdCurrentIndex = PROTOCOL_FUNCTION_QUEUED_CMD_BASE + 6,
    ProtocolQueuedCmdLeftSpace = PROTOCOL_FUNCTION_QUEUED_CMD_BASE + 7,
}