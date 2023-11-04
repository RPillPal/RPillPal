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
    pub expiration: String,
    pub end_date: String,
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
