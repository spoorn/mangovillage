#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub enum ClientState {
    JoiningServer,
    Running
}