use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::Json,
};
use sqlx::{PgPool, Row};
use crate::models::*;

pub async fn test_connection(
    State(pool): State<PgPool>,
) -> Result<Json<serde_json::Value>, StatusCode> {
    let row = sqlx::query("SELECT 1 as test")
        .fetch_one(&pool)
        .await
        .map_err(|e| {
            eprintln!("Database connection error: {}", e);
            StatusCode::INTERNAL_SERVER_ERROR
        })?;
    
    Ok(Json(serde_json::json!({
        "status": "Database connected successfully",
        "test": row.get::<i32, _>("test")
    })))
}

pub async fn get_all_customers(
    State(pool): State<PgPool>,
) -> Result<Json<Vec<CompleteCustomer>>, StatusCode> {
    let rows = sqlx::query(
        "SELECT email, first_name, last_name, dob, sex, marital_status, phone, 
        emergency_contact_name, emergency_contact_phone, employment_start_date,
        street_name, city, state, zip,
        client_name, client_street_name, client_city, client_state, client_zip,
        lca_title, lca_salary, lca_code, receipt_number, h1b_start_date, h1b_end_date, h1b_status
        FROM visa_details.h1bcustomer"
    )
    .fetch_all(&pool)
    .await
    .map_err(|e| {
        eprintln!("Database error in get_all_customers: {}", e);
        StatusCode::INTERNAL_SERVER_ERROR
    })?;

    let customers: Vec<CompleteCustomer> = rows.into_iter().map(|row| CompleteCustomer {
        email: row.get("email"),
        first_name: row.get("first_name"),
        last_name: row.get("last_name"),
        dob: row.get("dob"),
        sex: row.get("sex"),
        marital_status: row.get("marital_status"),
        phone: row.get("phone"),
        emergency_contact_name: row.get("emergency_contact_name"),
        emergency_contact_phone: row.get("emergency_contact_phone"),
        employment_start_date: row.get("employment_start_date"),
        street_name: row.get("street_name"),
        city: row.get("city"),
        state: row.get("state"),
        zip: row.get("zip"),
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

pub async fn create_personal(
    State(pool): State<PgPool>,
    Json(payload): Json<CreatePersonalRequest>,
) -> Result<Json<PersonalDetails>, StatusCode> {
    let row = sqlx::query(
        "INSERT INTO visa_details.h1bcustomer (email, first_name, last_name, dob, sex, marital_status, phone, 
         emergency_contact_name, emergency_contact_phone, employment_start_date)
         VALUES ($1, $2, $3, $4, $5::sex_enum, $6::marital_status_enum, $7, $8, $9, $10)
         RETURNING email, first_name, last_name, dob, sex, marital_status, phone, 
         emergency_contact_name, emergency_contact_phone, employment_start_date"
    )
    .bind(&payload.email)
    .bind(&payload.first_name)
    .bind(&payload.last_name)
    .bind(&payload.dob)
    .bind(&payload.sex)
    .bind(&payload.marital_status)
    .bind(&payload.phone)
    .bind(&payload.emergency_contact_name)
    .bind(&payload.emergency_contact_phone)
    .bind(&payload.employment_start_date)
    .fetch_one(&pool)
    .await
    .map_err(|e| {
        eprintln!("Database error in create_personal: {}", e);
        StatusCode::INTERNAL_SERVER_ERROR
    })?;

    let result = PersonalDetails {
        email: row.get("email"),
        first_name: row.get("first_name"),
        last_name: row.get("last_name"),
        dob: row.get("dob"),
        sex: row.get("sex"),
        marital_status: row.get("marital_status"),
        phone: row.get("phone"),
        emergency_contact_name: row.get("emergency_contact_name"),
        emergency_contact_phone: row.get("emergency_contact_phone"),
        employment_start_date: row.get("employment_start_date"),
    };

    Ok(Json(result))
}

pub async fn create_complete_customer(
    State(pool): State<PgPool>,
    Json(payload): Json<CreateCompleteCustomerRequest>,
) -> Result<Json<serde_json::Value>, StatusCode> {
    use std::time::{SystemTime, UNIX_EPOCH};
    let timestamp = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_nanos();
    
    let query_sql = format!(
        "/* query_{} */ INSERT INTO visa_details.h1bcustomer (
            email, first_name, last_name, dob, sex, marital_status, phone,
            emergency_contact_name, emergency_contact_phone, employment_start_date,
            street_name, city, state, zip,
            client_name, client_street_name, client_city, client_state, client_zip,
            lca_title, lca_salary, lca_code, receipt_number, h1b_start_date, h1b_end_date, h1b_status
        ) VALUES ($1, $2, $3, $4, $5::sex_enum, $6::marital_status_enum, $7, $8, $9, $10, $11, $12, $13, $14, $15, $16, $17, $18, $19, $20, $21, $22, $23, $24, $25, COALESCE($26, 'PENDING')::h1b_status_enum)",
        timestamp
    );
    
    let result = sqlx::query(&query_sql)
    .bind(&payload.email)
    .bind(&payload.first_name)
    .bind(&payload.last_name)
    .bind(&payload.dob)
    .bind(&payload.sex)
    .bind(&payload.marital_status)
    .bind(&payload.phone)
    .bind(&payload.emergency_contact_name)
    .bind(&payload.emergency_contact_phone)
    .bind(&payload.employment_start_date)
    .bind(&payload.street_name)
    .bind(&payload.city)
    .bind(&payload.state)
    .bind(&payload.zip)
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
    .execute(&pool)
    .await
    .map_err(|e| {
        eprintln!("Database error in create_complete_customer: {}", e);
        StatusCode::INTERNAL_SERVER_ERROR
    })?;

    Ok(Json(serde_json::json!({
        "message": "Customer created successfully",
        "email": payload.email,
        "rows_affected": result.rows_affected()
    })))
}

pub async fn get_customer_by_email(
    State(pool): State<PgPool>,
    Path(email): Path<String>,
) -> Result<Json<CompleteCustomer>, StatusCode> {
    let row = sqlx::query(
        "SELECT email, first_name, last_name, dob, sex, marital_status, phone, 
        emergency_contact_name, emergency_contact_phone, employment_start_date,
        street_name, city, state, zip,
        client_name, client_street_name, client_city, client_state, client_zip,
        lca_title, lca_salary, lca_code, receipt_number, h1b_start_date, h1b_end_date, h1b_status
        FROM visa_details.h1bcustomer WHERE email = $1"
    )
    .bind(email)
    .fetch_one(&pool)
    .await
    .map_err(|_| StatusCode::NOT_FOUND)?;

    let result = CompleteCustomer {
        email: row.get("email"),
        first_name: row.get("first_name"),
        last_name: row.get("last_name"),
        dob: row.get("dob"),
        sex: row.get("sex"),
        marital_status: row.get("marital_status"),
        phone: row.get("phone"),
        emergency_contact_name: row.get("emergency_contact_name"),
        emergency_contact_phone: row.get("emergency_contact_phone"),
        employment_start_date: row.get("employment_start_date"),
        street_name: row.get("street_name"),
        city: row.get("city"),
        state: row.get("state"),
        zip: row.get("zip"),
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

pub async fn update_address(
    State(pool): State<PgPool>,
    Path(email): Path<String>,
    Json(payload): Json<UpdateAddressRequest>,
) -> Result<Json<AddressDetails>, StatusCode> {
    let row = sqlx::query(
        "UPDATE visa_details.h1bcustomer 
         SET street_name = $2, city = $3, state = $4, zip = $5
         WHERE email = $1
         RETURNING email, street_name, city, state, zip"
    )
    .bind(email)
    .bind(&payload.street_name)
    .bind(&payload.city)
    .bind(&payload.state)
    .bind(&payload.zip)
    .fetch_one(&pool)
    .await
    .map_err(|_| StatusCode::NOT_FOUND)?;

    let result = AddressDetails {
        email: row.get("email"),
        street_name: row.get("street_name"),
        city: row.get("city"),
        state: row.get("state"),
        zip: row.get("zip"),
    };

    Ok(Json(result))
}

pub async fn update_h1b(
    State(pool): State<PgPool>,
    Path(email): Path<String>,
    Json(payload): Json<UpdateH1bRequest>,
) -> Result<Json<H1bDetails>, StatusCode> {
    let row = sqlx::query(
        "UPDATE visa_details.h1bcustomer 
         SET client_name = $2, client_street_name = $3, client_city = $4, client_state = $5, 
             client_zip = $6, lca_title = $7, lca_salary = $8, lca_code = $9, 
             receipt_number = $10, h1b_start_date = $11, h1b_end_date = $12, 
             h1b_status = COALESCE($13, h1b_status)::h1b_status_enum
         WHERE email = $1
         RETURNING email, client_name, client_street_name, client_city, client_state, client_zip,
         lca_title, lca_salary, lca_code, receipt_number, h1b_start_date, h1b_end_date, h1b_status"
    )
    .bind(email)
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
        email: row.get("email"),
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

pub async fn delete_customer(
    State(pool): State<PgPool>,
    Path(email): Path<String>,
) -> Result<Json<serde_json::Value>, StatusCode> {
    let result = sqlx::query("DELETE FROM visa_details.h1bcustomer WHERE email = $1")
        .bind(&email)
        .execute(&pool)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    if result.rows_affected() == 0 {
        return Err(StatusCode::NOT_FOUND);
    }

    Ok(Json(serde_json::json!({
        "message": "Customer deleted successfully",
        "email": email
    })))
}