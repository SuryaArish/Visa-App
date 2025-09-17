use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::Json,
};
use sqlx::{PgPool, Row};
use crate::models::*;
use utoipa;

#[utoipa::path(
    get,
    path = "/customers",
    responses(
        (status = 200, description = "List all customers", body = [CompleteCustomer])
    )
)]
pub async fn get_all_customers(
    State(pool): State<PgPool>,
) -> Result<Json<Vec<CompleteCustomer>>, StatusCode> {
    let rows = sqlx::query(
        "SELECT id, first_name, last_name, dob, sex, marital_status, phone, email, 
        email_verification, emergency_contact_name, emergency_contact_phone, employment_start_date,
        street_name, city, state, zip, address_start_date, address_end_date,
        client_name, client_street_name, client_city, client_state, client_zip,
        lca_title, lca_salary, lca_code, receipt_number, h1b_start_date, h1b_end_date, h1b_status
        FROM h1bcustomer"
    )
    .fetch_all(&pool)
    .await
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    let customers: Vec<CompleteCustomer> = rows.into_iter().map(|row| CompleteCustomer {
        id: row.get("id"),
        first_name: row.get("first_name"),
        last_name: row.get("last_name"),
        dob: row.get("dob"),
        sex: row.get("sex"),
        marital_status: row.get("marital_status"),
        phone: row.get("phone"),
        email: row.get("email"),
        email_verification: row.get("email_verification"),
        emergency_contact_name: row.get("emergency_contact_name"),
        emergency_contact_phone: row.get("emergency_contact_phone"),
        employment_start_date: row.get("employment_start_date"),
        street_name: row.get("street_name"),
        city: row.get("city"),
        state: row.get("state"),
        zip: row.get("zip"),
        address_start_date: row.get("address_start_date"),
        address_end_date: row.get("address_end_date"),
        client_name: row.get("client_name"),
        client_street_name: row.get("client_street_name"),
        client_city: row.get("client_city"),
        client_state: row.get("client_state"),
        client_zip: row.get("client_zip"),
        lca_title: row.get("lca_title"),
        lca_salary: row.get("lca_salary"),
        lca_code: row.get("lca_code"),
        receipt_number: row.get("receipt_number"),
        h1b_start_date: row.get("h1b_start_date"),
        h1b_end_date: row.get("h1b_end_date"),
        h1b_status: row.get("h1b_status"),
    }).collect();

    Ok(Json(customers))
}

#[utoipa::path(
    get,
    path = "/customers/{id}",
    params(
        ("id" = i64, Path, description = "Customer ID")
    ),
    responses(
        (status = 200, description = "Customer found", body = CompleteCustomer),
        (status = 404, description = "Customer not found")
    )
)]
pub async fn get_customer_by_id(
    State(pool): State<PgPool>,
    Path(id): Path<i64>,
) -> Result<Json<CompleteCustomer>, StatusCode> {
    let row = sqlx::query(
        "SELECT id, first_name, last_name, dob, sex, marital_status, phone, email, 
        email_verification, emergency_contact_name, emergency_contact_phone, employment_start_date,
        street_name, city, state, zip, address_start_date, address_end_date,
        client_name, client_street_name, client_city, client_state, client_zip,
        lca_title, lca_salary, lca_code, receipt_number, h1b_start_date, h1b_end_date, h1b_status
        FROM h1bcustomer WHERE id = $1"
    )
    .bind(id)
    .fetch_one(&pool)
    .await
    .map_err(|_| StatusCode::NOT_FOUND)?;

    let result = CompleteCustomer {
        id: row.get("id"),
        first_name: row.get("first_name"),
        last_name: row.get("last_name"),
        dob: row.get("dob"),
        sex: row.get("sex"),
        marital_status: row.get("marital_status"),
        phone: row.get("phone"),
        email: row.get("email"),
        email_verification: row.get("email_verification"),
        emergency_contact_name: row.get("emergency_contact_name"),
        emergency_contact_phone: row.get("emergency_contact_phone"),
        employment_start_date: row.get("employment_start_date"),
        street_name: row.get("street_name"),
        city: row.get("city"),
        state: row.get("state"),
        zip: row.get("zip"),
        address_start_date: row.get("address_start_date"),
        address_end_date: row.get("address_end_date"),
        client_name: row.get("client_name"),
        client_street_name: row.get("client_street_name"),
        client_city: row.get("client_city"),
        client_state: row.get("client_state"),
        client_zip: row.get("client_zip"),
        lca_title: row.get("lca_title"),
        lca_salary: row.get("lca_salary"),
        lca_code: row.get("lca_code"),
        receipt_number: row.get("receipt_number"),
        h1b_start_date: row.get("h1b_start_date"),
        h1b_end_date: row.get("h1b_end_date"),
        h1b_status: row.get("h1b_status"),
    };

    Ok(Json(result))
}

#[utoipa::path(
    post,
    path = "/customer/personal",
    request_body = CreatePersonalRequest,
    responses(
        (status = 200, description = "Customer created", body = PersonalDetails)
    )
)]
pub async fn create_personal(
    State(pool): State<PgPool>,
    Json(payload): Json<CreatePersonalRequest>,
) -> Result<Json<PersonalDetails>, StatusCode> {
    let row = sqlx::query(
        "INSERT INTO h1bcustomer (first_name, last_name, dob, sex, marital_status, phone, email, 
         emergency_contact_name, emergency_contact_phone, employment_start_date)
         VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10)
         RETURNING id, first_name, last_name, dob, sex, marital_status, phone, email, 
         email_verification, emergency_contact_name, emergency_contact_phone, employment_start_date"
    )
    .bind(&payload.first_name)
    .bind(&payload.last_name)
    .bind(&payload.dob)
    .bind(&payload.sex)
    .bind(&payload.marital_status)
    .bind(&payload.phone)
    .bind(&payload.email)
    .bind(&payload.emergency_contact_name)
    .bind(&payload.emergency_contact_phone)
    .bind(&payload.employment_start_date)
    .fetch_one(&pool)
    .await
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    let result = PersonalDetails {
        id: row.get("id"),
        first_name: row.get("first_name"),
        last_name: row.get("last_name"),
        dob: row.get("dob"),
        sex: row.get("sex"),
        marital_status: row.get("marital_status"),
        phone: row.get("phone"),
        email: row.get("email"),
        email_verification: row.get("email_verification"),
        emergency_contact_name: row.get("emergency_contact_name"),
        emergency_contact_phone: row.get("emergency_contact_phone"),
        employment_start_date: row.get("employment_start_date"),
    };

    Ok(Json(result))
}

#[utoipa::path(
    post,
    path = "/customer/{id}/address",
    params(
        ("id" = i64, Path, description = "Customer ID")
    ),
    request_body = UpdateAddressRequest,
    responses(
        (status = 200, description = "Address updated", body = AddressDetails)
    )
)]
pub async fn update_address(
    State(pool): State<PgPool>,
    Path(id): Path<i64>,
    Json(payload): Json<UpdateAddressRequest>,
) -> Result<Json<AddressDetails>, StatusCode> {
    let row = sqlx::query(
        "UPDATE h1bcustomer 
         SET street_name = $2, city = $3, state = $4, zip = $5, 
             address_start_date = $6, address_end_date = $7, updated_at = CURRENT_TIMESTAMP
         WHERE id = $1
         RETURNING id, street_name, city, state, zip, address_start_date, address_end_date"
    )
    .bind(id)
    .bind(&payload.street_name)
    .bind(&payload.city)
    .bind(&payload.state)
    .bind(&payload.zip)
    .bind(&payload.address_start_date)
    .bind(&payload.address_end_date)
    .fetch_one(&pool)
    .await
    .map_err(|_| StatusCode::NOT_FOUND)?;

    let result = AddressDetails {
        id: row.get("id"),
        street_name: row.get("street_name"),
        city: row.get("city"),
        state: row.get("state"),
        zip: row.get("zip"),
        address_start_date: row.get("address_start_date"),
        address_end_date: row.get("address_end_date"),
    };

    Ok(Json(result))
}

#[utoipa::path(
    post,
    path = "/customer/{id}/h1b",
    params(
        ("id" = i64, Path, description = "Customer ID")
    ),
    request_body = UpdateH1bRequest,
    responses(
        (status = 200, description = "H1B details updated", body = H1bDetails)
    )
)]
pub async fn update_h1b(
    State(pool): State<PgPool>,
    Path(id): Path<i64>,
    Json(payload): Json<UpdateH1bRequest>,
) -> Result<Json<H1bDetails>, StatusCode> {
    let row = sqlx::query(
        "UPDATE h1bcustomer 
         SET client_name = $2, client_street_name = $3, client_city = $4, client_state = $5, 
             client_zip = $6, lca_title = $7, lca_salary = $8, lca_code = $9, 
             receipt_number = $10, h1b_start_date = $11, h1b_end_date = $12, 
             h1b_status = COALESCE($13, h1b_status), updated_at = CURRENT_TIMESTAMP
         WHERE id = $1
         RETURNING id, client_name, client_street_name, client_city, client_state, client_zip,
         lca_title, lca_salary, lca_code, receipt_number, h1b_start_date, h1b_end_date, h1b_status"
    )
    .bind(id)
    .bind(&payload.client_name)
    .bind(&payload.client_street_name)
    .bind(&payload.client_city)
    .bind(&payload.client_state)
    .bind(&payload.client_zip)
    .bind(&payload.lca_title)
    .bind(&payload.lca_salary)
    .bind(&payload.lca_code)
    .bind(&payload.receipt_number)
    .bind(&payload.h1b_start_date)
    .bind(&payload.h1b_end_date)
    .bind(&payload.h1b_status)
    .fetch_one(&pool)
    .await
    .map_err(|_| StatusCode::NOT_FOUND)?;

    let result = H1bDetails {
        id: row.get("id"),
        client_name: row.get("client_name"),
        client_street_name: row.get("client_street_name"),
        client_city: row.get("client_city"),
        client_state: row.get("client_state"),
        client_zip: row.get("client_zip"),
        lca_title: row.get("lca_title"),
        lca_salary: row.get("lca_salary"),
        lca_code: row.get("lca_code"),
        receipt_number: row.get("receipt_number"),
        h1b_start_date: row.get("h1b_start_date"),
        h1b_end_date: row.get("h1b_end_date"),
        h1b_status: row.get("h1b_status"),
    };

    Ok(Json(result))
}

#[utoipa::path(
    delete,
    path = "/customers/{id}",
    params(
        ("id" = i64, Path, description = "Customer ID")
    ),
    responses(
        (status = 200, description = "Customer deleted successfully"),
        (status = 404, description = "Customer not found")
    )
)]
pub async fn delete_customer(
    State(pool): State<PgPool>,
    Path(id): Path<i64>,
) -> Result<Json<serde_json::Value>, StatusCode> {
    let result = sqlx::query("DELETE FROM h1bcustomer WHERE id = $1")
        .bind(id)
        .execute(&pool)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    if result.rows_affected() == 0 {
        return Err(StatusCode::NOT_FOUND);
    }

    Ok(Json(serde_json::json!({
        "message": "Customer deleted successfully",
        "id": id
    })))
}