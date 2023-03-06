#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub enum ServerState {
    LoadWorld,
    LoadEntities,
    Running
}