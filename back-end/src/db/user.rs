use chrono::prelude::*;
use mongodb::bson::oid::ObjectId;
use serde::{Deserialize, Serialize};
use serde_repr::Serialize_repr;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum ContactInfo {
    Email(String),
    Phone(String),
    Address(String),
}

#[derive(Debug, Clone, Copy, Default, Serialize_repr, Deserialize)]
#[serde(rename_all = "camelCase")]
#[repr(u8)]
pub enum Frequency {
    TwiceDaily,
    #[default]
    Daily,
    BiWeekly,
    Weekly,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Prescription {
    pub name: String,
    pub num_pills: u32,
    pub dosage: String,
    pub frequency: Frequency,
    pub end_date: u32,
    pub expiration: u32,
    pub last_taken: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
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

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct EmbeddedUser {
    pub name: String,
    pub pin: u32,
    pub prescription_name: String,
    pub num_pills: u32,
    pub last_taken: u32,
    pub dosage: String,
    pub frequency: Frequency,
    pub can_take_pill: bool,
    pub is_expired: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UpdateRequest {
    pub name: String,
    pub num_pills: u32,
    pub time_dispensed: Option<u32>,
}

impl From<User> for EmbeddedUser {
    fn from(val: User) -> Self {
        From::from(&val)
    }
}

impl From<&User> for EmbeddedUser {
    fn from(val: &User) -> Self {
        let mut user = EmbeddedUser {
            name: val.name.clone(),
            pin: val.pin,
            prescription_name: val.prescription[0].name.clone(),
            num_pills: val.prescription[0].num_pills,
            last_taken: val.prescription[0].last_taken,
            dosage: val.prescription[0].dosage.clone(),
            frequency: val.prescription[0].frequency,

            ..Default::default()
        };

        // unix timestamp of last_taken vs now, compare with frequency
        let now = TimeZone::from_utc_datetime(&Utc, &Utc::now().naive_utc());

        let naive_last =
            NaiveDateTime::from_timestamp_opt(val.prescription[0].last_taken as i64, 0).unwrap();
        let last = TimeZone::from_utc_datetime(&Utc, &naive_last);

        let naive_expired =
            NaiveDateTime::from_timestamp_opt(val.prescription[0].expiration as i64, 0).unwrap();
        let expired = TimeZone::from_utc_datetime(&Utc, &naive_expired);

        user.can_take_pill = match user.frequency {
            Frequency::TwiceDaily => (now - last).num_hours() >= 12,
            Frequency::Daily => (now - last).num_days() >= 1,
            Frequency::BiWeekly => (now - last).num_days() >= 3,
            Frequency::Weekly => (now - last).num_weeks() >= 1,
        };

        user.is_expired = (now - expired).num_days() >= 0;

        user
    }
}
