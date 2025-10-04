use rspotify::model::{PlayHistory, SimplifiedPlaylist};

// Msg from gui to backend
pub enum BackendMessage {
    RequestLoadUserProfile,
}

pub struct UserProfile {
    pub name: String,
    pub profile_img: String,
}

// Msg from backend to gui
pub enum GuiMessage {
    UserProfileLoaded(UserProfile),
    UserPlaylistsLoaded(Vec<SimplifiedPlaylist>),
    UserRecentlyPlayed(Vec<PlayHistory>),
}
