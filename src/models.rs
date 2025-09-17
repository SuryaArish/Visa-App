use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use chrono::NaiveDate;
use bigdecimal::BigDecimal;
use utoipa::ToSchema;

#[derive(Debug, Clone, sqlx::Type, Serialize, Deserialize, ToSchema)]
#[sqlx(type_name = "sex_enum", rename_all = "UPPERCASE")]
pub enum SexEnum {
    Male,
    Female,
    Other,
}

#[derive(Debug, Clone, sqlx::Type, Serialize, Deserialize, ToSchema)]
#[sqlx(type_name = "marital_status_enum", rename_all = "UPPERCASE")]
pub enum MaritalStatusEnum {
    Single,
    Married,
    Divorced,
    Widowed,
}

#[derive(Debug, Clone, sqlx::Type, Serialize, Deserialize, ToSchema)]
#[sqlx(type_name = "email_verification_enum", rename_all = "UPPERCASE")]
pub enum EmailVerificationEnum {
    Pending,
    Verified,
    Rejected,
}

#[derive(Debug, Clone, sqlx::Type, Serialize, Deserialize, ToSchema)]
#[sqlx(type_name = "h1b_status_enum", rename_all = "UPPERCASE")]
pub enum H1bStatusEnum {
    Pending,
    Active,
    Expired,
    Revoked,
}

#[derive(Debug, FromRow, Serialize)]
pub struct PersonalDetails {
    pub id: i64,
    pub first_name: String,
    pub last_name: String,
    pub dob: NaiveDate,
    pub sex: SexEnum,
    pub marital_status: MaritalStatusEnum,
    pub phone: String,
    pub email: String,
    pub email_verification: EmailVerificationEnum,
    pub emergency_contact_name: Option<String>,
    pub emergency_contact_phone: Option<String>,
    pub employment_start_date: Option<NaiveDate>,
}

#[derive(Debug, FromRow, Serialize)]
pub struct AddressDetails {
    pub id: i64,
    pub street_name: Option<String>,
    pub city: Option<String>,
    pub state: Option<String>,
    pub zip: Option<String>,
    pub address_start_date: Option<NaiveDate>,
    pub address_end_date: Option<NaiveDate>,
}

#[derive(Debug, FromRow, Serialize)]
pub struct CustomerAddress {
    pub id: i64,
    pub first_name: String,
    pub last_name: String,
    pub street_name: Option<String>,
    pub city: Option<String>,
    pub state: Option<String>,
    pub zip: Option<String>,
}

#[derive(Debug, FromRow, Serialize)]
pub struct H1bDetails {
    pub id: i64,
    pub client_name: Option<String>,
    pub client_street_name: Option<String>,
    pub client_city: Option<String>,
    pub client_state: Option<String>,
    pub client_zip: Option<String>,
    pub lca_title: Option<String>,
    pub lca_salary: Option<BigDecimal>,
    pub lca_code: Option<String>,
    pub receipt_number: Option<String>,
    pub h1b_start_date: Option<NaiveDate>,
    pub h1b_end_date: Option<NaiveDate>,
    pub h1b_status: H1bStatusEnum,
}

#[derive(Debug, Deserialize, ToSchema)]
pub struct CreatePersonalRequest {
    pub first_name: String,
    pub last_name: String,
    pub dob: NaiveDate,
    pub sex: SexEnum,
    pub marital_status: MaritalStatusEnum,
    pub phone: String,
    pub email: String,
    pub emergency_contact_name: Option<String>,
    pub emergency_contact_phone: Option<String>,
    pub employment_start_date: Option<NaiveDate>,
}

#[derive(Debug, Deserialize, ToSchema)]
pub struct UpdateAddressRequest {
    pub street_name: Option<String>,
    pub city: Option<String>,
    pub state: Option<String>,
    pub zip: Option<String>,
    pub address_start_date: Option<NaiveDate>,
    pub address_end_date: Option<NaiveDate>,
}

#[derive(Debug, Deserialize, ToSchema)]
pub struct UpdateH1bRequest {
    pub client_name: Option<String>,
    pub client_street_name: Option<String>,
    pub client_city: Option<String>,
    pub client_state: Option<String>,
    pub client_zip: Option<String>,
    pub lca_title: Option<String>,
    pub lca_salary: Option<BigDecimal>,
    pub lca_code: Option<String>,
    pub receipt_number: Option<String>,
    pub h1b_start_date: Option<NaiveDate>,
    pub h1b_end_date: Option<NaiveDate>,
    pub h1b_status: Option<H1bStatusEnum>,
}

#[derive(Debug, Serialize, ToSchema)]
pub struct CompleteCustomer {
    pub id: i64,
    pub first_name: String,
    pub last_name: String,
    pub dob: NaiveDate,
    pub sex: SexEnum,
    pub marital_status: MaritalStatusEnum,
    pub phone: String,
    pub email: String,
    pub email_verification: EmailVerificationEnum,
    pub emergency_contact_name: Option<String>,
    pub emergency_contact_phone: Option<String>,
    pub employment_start_date: Option<NaiveDate>,
    pub street_name: Option<String>,
    pub city: Option<String>,
    pub state: Option<String>,
    pub zip: Option<String>,
    pub address_start_date: Option<NaiveDate>,
    pub address_end_date: Option<NaiveDate>,
    pub client_name: Option<String>,
    pub client_street_name: Option<String>,
    pub client_city: Option<String>,
    pub client_state: Option<String>,
    pub client_zip: Option<String>,
    pub lca_title: Option<String>,
    pub lca_salary: Option<BigDecimal>,
    pub lca_code: Option<String>,
    pub receipt_number: Option<String>,
    pub h1b_start_date: Option<NaiveDate>,
    pub h1b_end_date: Option<NaiveDate>,
    pub h1b_status: H1bStatusEnum,
}