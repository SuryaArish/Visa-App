use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use chrono::NaiveDate;
use utoipa::ToSchema;
use rust_decimal::Decimal;
use uuid::Uuid;

#[derive(Debug, Deserialize, ToSchema)]
pub struct CreateCompleteCustomerRequest {
    pub email: String,
    pub login_email: String,
    pub first_name: String,
    pub last_name: String,
    pub dob: NaiveDate,
    pub sex: String,
    pub marital_status: String,
    pub phone: String,
    pub emergency_contact_name: String,
    pub emergency_contact_phone: String,
    pub employment_start_date: NaiveDate,
    pub street_name: String,
    pub city: String,
    pub state: String,
    pub zip: String,
    pub client_name: String,
    pub client_street_name: String,
    pub client_city: String,
    pub client_state: String,
    pub client_zip: String,
    pub lca_title: String,
    pub lca_salary: Decimal,
    pub lca_code: String,
    pub receipt_number: String,
    pub h1b_start_date: NaiveDate,
    pub h1b_end_date: NaiveDate,
    pub h1b_status: Option<String>,
}

#[derive(Debug, FromRow, Serialize, ToSchema)]
pub struct CreateCustomer {
    pub customer_id: Uuid,
    pub email: String,
    pub first_name: String,
    pub last_name: String,
    pub dob: NaiveDate,
    pub sex: String,
    pub marital_status: String,
    pub phone: String,
    pub emergency_contact_name: String,
    pub emergency_contact_phone: String,
    pub employment_start_date: NaiveDate,
    pub street_name: String,
    pub city: String,
    pub state: String,
    pub zip: String,
    pub client_name: String,
    pub client_street_name: String,
    pub client_city: String,
    pub client_state: String,
    pub client_zip: String,
    pub lca_title: String,
    pub lca_salary: Decimal,
    pub lca_code: String,
    pub receipt_number: String,
    pub h1b_start_date: NaiveDate,
    pub h1b_end_date: NaiveDate,
    pub h1b_status: String,
}

#[derive(Debug, Deserialize, ToSchema)]
pub struct UpdateVisaDetailsRequest {
    pub first_name: Option<String>,
    pub last_name: Option<String>,
    pub dob: Option<NaiveDate>,
    pub sex: Option<String>,
    pub marital_status: Option<String>,
    pub phone: Option<String>,
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
    pub lca_salary: Option<Decimal>,
    pub lca_code: Option<String>,
    pub receipt_number: Option<String>,
    pub h1b_start_date: Option<NaiveDate>,
    pub h1b_end_date: Option<NaiveDate>,
    pub h1b_status: Option<String>,
}

#[derive(Debug, Deserialize, ToSchema)]
pub struct SoftDeleteRequest {
    pub email: String,
}