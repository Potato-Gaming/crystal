/*
 * 
 *
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * The version of the OpenAPI document: 1.0.0
 * 
 * Generated by: https://openapi-generator.tech
 */




#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct LolChatFriendCountsResource {
    #[serde(rename = "numFriends", skip_serializing_if = "Option::is_none")]
    pub num_friends: Option<i32>,
    #[serde(rename = "numFriendsAvailable", skip_serializing_if = "Option::is_none")]
    pub num_friends_available: Option<i32>,
    #[serde(rename = "numFriendsAway", skip_serializing_if = "Option::is_none")]
    pub num_friends_away: Option<i32>,
    #[serde(rename = "numFriendsInChampSelect", skip_serializing_if = "Option::is_none")]
    pub num_friends_in_champ_select: Option<i32>,
    #[serde(rename = "numFriendsInGame", skip_serializing_if = "Option::is_none")]
    pub num_friends_in_game: Option<i32>,
    #[serde(rename = "numFriendsInQueue", skip_serializing_if = "Option::is_none")]
    pub num_friends_in_queue: Option<i32>,
    #[serde(rename = "numFriendsMobile", skip_serializing_if = "Option::is_none")]
    pub num_friends_mobile: Option<i32>,
    #[serde(rename = "numFriendsOnline", skip_serializing_if = "Option::is_none")]
    pub num_friends_online: Option<i32>,
}

impl LolChatFriendCountsResource {
    pub fn new() -> LolChatFriendCountsResource {
        LolChatFriendCountsResource {
            num_friends: None,
            num_friends_available: None,
            num_friends_away: None,
            num_friends_in_champ_select: None,
            num_friends_in_game: None,
            num_friends_in_queue: None,
            num_friends_mobile: None,
            num_friends_online: None,
        }
    }
}


