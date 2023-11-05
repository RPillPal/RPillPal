use mongodb::bson::oid::ObjectId;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum ContactInfo {
    Email(String),
    Phone(String),
    Address(String),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Frequency {
    TwiceDaily,
    Daily,
    BiWeekly,
    Weekly,
    Custom(String),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Prescription {
    pub name: String,
    pub num_pills: u32,
    pub dosage: String,
    pub frequency: Frequency,
    pub end_date: u64,
    pub expiration: u64,
    pub last_taken: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Doctor {
    pub name: String,
    pub contact_info: Vec<ContactInfo>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct User {
    #[serde(rename = "_id")]
    pub id: ObjectId,
    pub name: String,
    pub prescription: Vec<Prescription>,
    pub pin: u32,
    pub contact_info: Vec<ContactInfo>,
    pub doctor: Doctor,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EmbeddedUser {
    pub name: String,
    pub pin: u32,
    pub prescription_name: String,
    pub num_pills: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateRequest {
    pub name: String,
    pub num_pills: u32,
    pub time_dispensed: String,
}

impl From<User> for EmbeddedUser {
    fn from(val: User) -> Self {
        EmbeddedUser {
            name: val.name,
            pin: val.pin,
            prescription_name: val.prescription[0].name.clone(),
            num_pills: val.prescription[0].num_pills,
        }
    }
}

impl From<&User> for EmbeddedUser {
    fn from(val: &User) -> Self {
        EmbeddedUser {
            name: val.name.clone(),
            pin: val.pin,
            prescription_name: val.prescription[0].name.clone(),
            num_pills: val.prescription[0].num_pills,
        }
    }
}
