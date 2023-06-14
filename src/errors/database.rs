#[derive(Debug)]
pub enum DBError {
    CouldNotOpenWebSocket,
    CouldNotConnectToNameSpace,
    #[allow(unused)]
    TableCreateFailed,
    UserCreateFailed,
    UserSelectFailed,
    AuthFailed,
}
