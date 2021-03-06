/*
 * 
 *
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * The version of the OpenAPI document: 1.0.0
 * 
 * Generated by: https://openapi-generator.tech
 */


/// 
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum PatcherNotificationId {
    #[serde(rename = "UnspecifiedError")]
    UnspecifiedError,
    #[serde(rename = "ConnectionError")]
    ConnectionError,
    #[serde(rename = "MissingFilesError")]
    MissingFilesError,
    #[serde(rename = "FailedToWriteError")]
    FailedToWriteError,
    #[serde(rename = "WillRestoreClientBackupOnRestart")]
    WillRestoreClientBackupOnRestart,
    #[serde(rename = "DidRestoreClientBackup")]
    DidRestoreClientBackup,
    #[serde(rename = "NotEnoughDiskSpace")]
    NotEnoughDiskSpace,
    #[serde(rename = "BrokenPermissions")]
    BrokenPermissions,

}




