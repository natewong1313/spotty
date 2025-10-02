// Msg from gui to backend
pub enum BackendMessage {
    RequestLoadUserProfile,
}

// Msg from backend to gui
pub enum GuiMessage {
    UserProfileLoaded,
}
