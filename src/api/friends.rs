use napi::bindgen_prelude::BigInt;
use napi_derive::napi;
use steamworks::SteamId;

#[napi]
pub mod friends {
    use super::*;

    /// Returns the Steam display name for any player whose persona data Steam has cached.
    /// After `downloadScores` (leaderboards), Steam caches names for the returned entries,
    /// so this will work for leaderboard players even if they are not on the friend list.
    /// Returns an empty string if the data is not yet available.
    #[napi]
    pub fn get_friend_persona_name(steam_id64: BigInt) -> String {
        let client = crate::client::get_client();
        let steam_id = SteamId::from_raw(steam_id64.get_u64().1);
        client.friends().get_friend(steam_id).name()
    }
}
