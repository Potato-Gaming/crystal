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
pub struct SeriesMediaDto {
    #[serde(rename = "accentColor", skip_serializing_if = "Option::is_none")]
    pub accent_color: Option<String>,
    #[serde(rename = "backgroundImageLargeUrl", skip_serializing_if = "Option::is_none")]
    pub background_image_large_url: Option<String>,
    #[serde(rename = "backgroundImageSmallUrl", skip_serializing_if = "Option::is_none")]
    pub background_image_small_url: Option<String>,
    #[serde(rename = "backgroundUrl", skip_serializing_if = "Option::is_none")]
    pub background_url: Option<String>,
    #[serde(rename = "trackerIconUrl", skip_serializing_if = "Option::is_none")]
    pub tracker_icon_url: Option<String>,
}

impl SeriesMediaDto {
    pub fn new() -> SeriesMediaDto {
        SeriesMediaDto {
            accent_color: None,
            background_image_large_url: None,
            background_image_small_url: None,
            background_url: None,
            tracker_icon_url: None,
        }
    }
}


