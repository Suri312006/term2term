use crate::grpc::{LoginReq, RegisterReq, SendMsgReq};

pub enum Command {
    Register(RegisterReq),
    Login(LoginReq),
    SendMessage(SendMsgReq),
}
