use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use chrono::NaiveDate;
use utoipa::ToSchema;

#[derive(Debug, FromRow, Serialize)]
pub struct PersonalDetails {
    pub email: String,
    pub first_name: String,
    pub last_name: String,
    pub dob: NaiveDate,
    pub sex: String,
    pub marital_status: String,
    pub phone: String,
    pub emergency_contact_name: Option<String>,
    pub emergency_contact_phone: Option<String>,
    pub employment_start_date: Option<NaiveDate>,
}

#[derive(Debug, FromRow, Serialize)]
pub struct AddressDetails {
    pub email: String,
    pub street_name: Option<String>,
    pub city: Option<String>,
    pub state: Option<String>,
    pub zip: Option<String>,
}

#[derive(Debug, FromRow, Serialize)]
pub struct CustomerAddress {
    pub email: String,
    pub first_name: String,
    pub last_name: String,
    pub street_name: Option<String>,
    pub city: Option<String>,
    pub state: Option<String>,
    pub zip: Option<String>,
}

#[derive(Debug, FromRow, Serialize)]
pub struct H1bDetails {
    pub email: String,
    pub client_name: Option<String>,
    pub client_street_name: Option<String>,
    pub client_city: Option<String>,
    pub client_state: Option<String>,
    pub client_zip: Option<String>,
    pub lca_title: Option<String>,
    pub lca_salary: Option<f64>,
    pub lca_code: Option<String>,
    pub receipt_number: Option<String>,
    pub h1b_start_date: Option<NaiveDate>,
    pub h1b_end_date: Option<NaiveDate>,
    pub h1b_status: String,
}

#[derive(Debug, Deserialize, ToSchema)]
pub struct CreatePersonalRequest {
    pub first_name: String,
    pub last_name: String,
    pub dob: NaiveDate,
    pub sex: String,
    pub marital_status: String,
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
}

#[derive(Debug, Deserialize, ToSchema)]
pub struct UpdateH1bRequest {
    pub client_name: Option<String>,
    pub client_street_name: Option<String>,
    pub client_city: Option<String>,
    pub client_state: Option<String>,
    pub client_zip: Option<String>,
    pub lca_title: Option<String>,
    pub lca_salary: Option<f64>,
    pub lca_code: Option<String>,
    pub receipt_number: Option<String>,
    pub h1b_start_date: Option<NaiveDate>,
    pub h1b_end_date: Option<NaiveDate>,
    pub h1b_status: Option<String>,
}

#[derive(Debug, Deserialize, ToSchema)]
pub struct CreateCompleteCustomerRequest {
    // Personal Details
    pub email: String,
    pub first_name: String,
    pub last_name: String,
    pub dob: NaiveDate,
    pub sex: String,
    pub marital_status: String,
    pub phone: String,
    pub emergency_contact_name: Option<String>,
    pub emergency_contact_phone: Option<String>,
    pub employment_start_date: Option<NaiveDate>,
    // Address Details
    pub street_name: Option<String>,
    pub city: Option<String>,
    pub state: Option<String>,
    pub zip: Option<String>,
    // H1B Details
    pub client_name: Option<String>,
    pub client_street_name: Option<String>,
    pub client_city: Option<String>,
    pub client_state: Option<String>,
    pub client_zip: Option<String>,
    pub lca_title: Option<String>,
    pub lca_salary: Option<f64>,
    pub lca_code: Option<String>,
    pub receipt_number: Option<String>,
    pub h1b_start_date: Option<NaiveDate>,
    pub h1b_end_date: Option<NaiveDate>,
    pub h1b_status: Option<String>,
}

#[derive(Debug, FromRow, Serialize, ToSchema)]
pub struct CompleteCustomer {
    pub email: String,
    pub first_name: String,
    pub last_name: String,
    pub dob: NaiveDate,
    pub sex: String,
    pub marital_status: String,
    pub phone: String,
    pub emergency_contact_name: Option<String>,
    pub emergency_contact_phone: Option<String>,
    pub employment_start_date: Option<NaiveDate>,
    pub street_name: Option<String>,
    pub city: Option<String>,
    pub state: Option<String>,
    pub zip: Option<String>,
    pub client_name: Option<String>,
    pub client_street_name: Option<String>,
    pub client_city: Option<String>,
    pub client_state: Option<String>,
    pub client_zip: Option<String>,
    pub lca_title: Option<String>,
    pub lca_salary: Option<f64>,
    pub lca_code: Option<String>,
    pub receipt_number: Option<String>,
    pub h1b_start_date: Option<NaiveDate>,
    pub h1b_end_date: Option<NaiveDate>,
    pub h1b_status: String,
}