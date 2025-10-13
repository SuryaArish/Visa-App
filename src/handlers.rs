use axum::{
    extract::Path,
    http::StatusCode,
    response::Json,
};
use sqlx::Row;
use crate::models::*;
use crate::config::database::get_db_pool;
use std::time::{SystemTime, UNIX_EPOCH};

fn get_timestamp() -> u128 {
    SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_nanos()
}

pub async fn health_check() -> Result<Json<serde_json::Value>, StatusCode> {
    Ok(Json(serde_json::json!({
        "status": "OK",
        "message": "API is running"
    })))
}

pub async fn test_connection() -> Result<Json<serde_json::Value>, StatusCode> {
    let pool = get_db_pool();
    match sqlx::query("SELECT 1 as test")
        .fetch_one(&pool)
        .await {
        Ok(_) => Ok(Json(serde_json::json!({
            "status": "Database connected successfully"
        }))),
        Err(e) => {
            eprintln!("Database connection error: {}", e);
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}

pub async fn get_all_customers() -> Result<Json<Vec<CreateCustomer>>, StatusCode> {
    println!("🔥 get_all_customers function called");
    let pool = get_db_pool();
    
    let query_sql = "SELECT email, first_name, last_name, dob, sex::text, marital_status::text, phone, 
        emergency_contact_name, emergency_contact_phone, employment_start_date,
        street_name, city, state, zip,
        client_name, client_street_name, client_city, client_state, client_zip,
        lca_title, lca_salary, lca_code, receipt_number, h1b_start_date, h1b_end_date, h1b_status::text
        FROM visa_info.h1bcustomer";
    
    let rows = sqlx::query(query_sql)
    .fetch_all(&pool)
    .await
    .map_err(|e| {
        eprintln!("Database error in get_all_customers: {}", e);
        StatusCode::INTERNAL_SERVER_ERROR
    })?;

    let customers: Vec<CreateCustomer> = rows.into_iter().map(|row| CreateCustomer {
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

pub async fn create_visa_details(
    Json(payload): Json<CreateCompleteCustomerRequest>,
) -> Result<Json<serde_json::Value>, StatusCode> {
    println!("🔥 create_visa_details function called");
    let pool = get_db_pool();
    let h1b_status = payload.h1b_status.as_deref().unwrap_or("Active");
    
    let mut tx = pool.begin().await.map_err(|e| {
        eprintln!("Failed to begin transaction: {}", e);
        StatusCode::INTERNAL_SERVER_ERROR
    })?;
    
    let timestamp = get_timestamp();
    let query_sql = format!(
        "/* create_{} */ INSERT INTO visa_info.h1bcustomer (
            email, first_name, last_name, dob, sex, marital_status, phone,
            emergency_contact_name, emergency_contact_phone, employment_start_date,
            street_name, city, state, zip,
            client_name, client_street_name, client_city, client_state, client_zip,
            lca_title, lca_salary, lca_code, receipt_number, h1b_start_date, h1b_end_date, h1b_status
        ) VALUES (
            $1, $2, $3, $4, $5::visa_info.sex_enum, $6::visa_info.marital_status_enum, $7,
            $8, $9, $10, $11, $12, $13, $14, $15, $16, $17, $18, $19, $20, $21, $22, $23, $24, $25, $26::visa_info.h1b_status_enum
        )", timestamp
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
    .bind(h1b_status)
    .execute(&mut *tx)
    .await
    .map_err(|e| {
        eprintln!("Database error in create_visa_details: {}", e);
        StatusCode::INTERNAL_SERVER_ERROR
    })?;
    
    tx.commit().await.map_err(|e| {
        eprintln!("Failed to commit transaction: {}", e);
        StatusCode::INTERNAL_SERVER_ERROR
    })?;

    Ok(Json(serde_json::json!({
        "message": "Visa details created successfully",
        "email": payload.email,
        "rows_affected": result.rows_affected()
    })))
}

pub async fn update_visa_details(
    Path(email): Path<String>,
    Json(payload): Json<UpdateVisaDetailsRequest>,
) -> Result<Json<serde_json::Value>, StatusCode> {
    println!("🔥 update_visa_details function called for email: {}", email);
    let pool = get_db_pool();
    // First, get the current record
    let mut tx = pool.begin().await.map_err(|e| {
        eprintln!("Failed to begin transaction: {}", e);
        StatusCode::INTERNAL_SERVER_ERROR
    })?;
    
    let timestamp = get_timestamp();
    let select_sql = format!("/* select_{} */ SELECT * FROM visa_info.h1bcustomer WHERE email = $1", timestamp);
    
    let current_row = sqlx::query(&select_sql)
        .bind(&email)
        .fetch_optional(&mut *tx)
        .await
        .map_err(|e| {
            eprintln!("Database error in update_visa_details select: {}", e);
            StatusCode::INTERNAL_SERVER_ERROR
        })?;
    
    let current = match current_row {
        Some(row) => row,
        None => {
            return Ok(Json(serde_json::json!({
                "status": 404,
                "message": "Record not found in the database",
                "email": email
            })));
        }
    };
    
    // Use provided values or keep current values
    let first_name = payload.first_name.unwrap_or_else(|| current.get("first_name"));
    let last_name = payload.last_name.unwrap_or_else(|| current.get("last_name"));
    let dob = payload.dob.unwrap_or_else(|| current.get("dob"));
    let sex = payload.sex.unwrap_or_else(|| current.get("sex"));
    let marital_status = payload.marital_status.unwrap_or_else(|| current.get("marital_status"));
    let phone = payload.phone.unwrap_or_else(|| current.get("phone"));
    let emergency_contact_name = payload.emergency_contact_name.unwrap_or_else(|| current.get("emergency_contact_name"));
    let emergency_contact_phone = payload.emergency_contact_phone.unwrap_or_else(|| current.get("emergency_contact_phone"));
    let employment_start_date = payload.employment_start_date.unwrap_or_else(|| current.get("employment_start_date"));
    let street_name = payload.street_name.unwrap_or_else(|| current.get("street_name"));
    let city = payload.city.unwrap_or_else(|| current.get("city"));
    let state = payload.state.unwrap_or_else(|| current.get("state"));
    let zip = payload.zip.unwrap_or_else(|| current.get("zip"));
    let client_name = payload.client_name.unwrap_or_else(|| current.get("client_name"));
    let client_street_name = payload.client_street_name.unwrap_or_else(|| current.get("client_street_name"));
    let client_city = payload.client_city.unwrap_or_else(|| current.get("client_city"));
    let client_state = payload.client_state.unwrap_or_else(|| current.get("client_state"));
    let client_zip = payload.client_zip.unwrap_or_else(|| current.get("client_zip"));
    let lca_title = payload.lca_title.unwrap_or_else(|| current.get("lca_title"));
    let lca_salary = payload.lca_salary.unwrap_or_else(|| current.get("lca_salary"));
    let lca_code = payload.lca_code.unwrap_or_else(|| current.get("lca_code"));
    let receipt_number = payload.receipt_number.unwrap_or_else(|| current.get("receipt_number"));
    let h1b_start_date = payload.h1b_start_date.unwrap_or_else(|| current.get("h1b_start_date"));
    let h1b_end_date = payload.h1b_end_date.unwrap_or_else(|| current.get("h1b_end_date"));
    let h1b_status = payload.h1b_status.unwrap_or_else(|| current.get("h1b_status"));
    
    let update_timestamp = get_timestamp();
    let update_sql = format!(
        "/* update_{} */ UPDATE visa_info.h1bcustomer SET
            first_name = $2, last_name = $3, dob = $4, sex = $5::visa_info.sex_enum,
            marital_status = $6::visa_info.marital_status_enum, phone = $7,
            emergency_contact_name = $8, emergency_contact_phone = $9, employment_start_date = $10,
            street_name = $11, city = $12, state = $13, zip = $14,
            client_name = $15, client_street_name = $16, client_city = $17, client_state = $18, client_zip = $19,
            lca_title = $20, lca_salary = $21, lca_code = $22, receipt_number = $23,
            h1b_start_date = $24, h1b_end_date = $25, h1b_status = $26::visa_info.h1b_status_enum
         WHERE email = $1", update_timestamp
    );
    
    let result = sqlx::query(&update_sql)
        .bind(&email)
        .bind(&first_name)
        .bind(&last_name)
        .bind(&dob)
        .bind(&sex)
        .bind(&marital_status)
        .bind(&phone)
        .bind(&emergency_contact_name)
        .bind(&emergency_contact_phone)
        .bind(&employment_start_date)
        .bind(&street_name)
        .bind(&city)
        .bind(&state)
        .bind(&zip)
        .bind(&client_name)
        .bind(&client_street_name)
        .bind(&client_city)
        .bind(&client_state)
        .bind(&client_zip)
        .bind(&lca_title)
        .bind(&lca_salary)
        .bind(&lca_code)
        .bind(&receipt_number)
        .bind(&h1b_start_date)
        .bind(&h1b_end_date)
        .bind(&h1b_status)
        .execute(&mut *tx)
        .await
        .map_err(|e| {
            eprintln!("Database error in update_visa_details: {}", e);
            StatusCode::INTERNAL_SERVER_ERROR
        })?;
    
    tx.commit().await.map_err(|e| {
        eprintln!("Failed to commit transaction: {}", e);
        StatusCode::INTERNAL_SERVER_ERROR
    })?;

    Ok(Json(serde_json::json!({
        "message": "Visa details updated successfully",
        "email": email,
        "rows_affected": result.rows_affected()
    })))
}

pub async fn delete_visa_details(
    Path(email): Path<String>,
) -> Result<Json<serde_json::Value>, StatusCode> {
    println!("🔥 delete_visa_details function called for email: {}", email);
    let pool = get_db_pool();
    let mut tx = pool.begin().await.map_err(|e| {
        eprintln!("Failed to begin transaction: {}", e);
        StatusCode::INTERNAL_SERVER_ERROR
    })?;
    
    let timestamp = get_timestamp();
    let query_sql = format!("/* delete_{} */ DELETE FROM visa_info.h1bcustomer WHERE email = $1", timestamp);
    
    let result = sqlx::query(&query_sql)
        .bind(&email)
        .execute(&mut *tx)
        .await
        .map_err(|e| {
            eprintln!("Database error in delete_visa_details: {}", e);
            StatusCode::INTERNAL_SERVER_ERROR
        })?;

    if result.rows_affected() == 0 {
        return Ok(Json(serde_json::json!({
            "status": 404,
            "message": "Record not found in the database",
            "email": email
        })));
    }
    
    tx.commit().await.map_err(|e| {
        eprintln!("Failed to commit transaction: {}", e);
        StatusCode::INTERNAL_SERVER_ERROR
    })?;

    Ok(Json(serde_json::json!({
        "message": "Visa details deleted successfully",
        "email": email,
        "rows_affected": result.rows_affected()
    })))
}

pub async fn soft_delete_customer(
    Path(email): Path<String>,
) -> Result<Json<serde_json::Value>, StatusCode> {
    println!("🔥 soft_delete_customer function called for email: {}", email);
    let pool = get_db_pool();

    // Use the pool directly, SQLx will manage connections automatically
    match sqlx::query("UPDATE visa_info.h1bcustomer SET h1b_status = 'Inactive' WHERE email = $1")
        .bind(&email)
        .execute(&pool) // <-- pass the pool, not manually acquired conn
        .await 
    {
        Ok(result) => {
            if result.rows_affected() == 0 {
                Ok(Json(serde_json::json!({
                    "status": 404,
                    "message": "Record not found in the database",
                    "email": email
                })))
            } else {
                Ok(Json(serde_json::json!({
                    "message": "Customer soft deleted successfully",
                    "email": email,
                    "rows_affected": result.rows_affected()
                })))
            }
        },
        Err(e) => {
            eprintln!("Database error: {}", e);
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}

pub async fn get_all_h1b_customers() -> Result<Json<Vec<CreateCustomer>>, StatusCode> {
    println!("🔥 get_all_h1b_customers function called");
    let pool = get_db_pool();
    let mut tx = pool.begin().await.map_err(|e| {
        eprintln!("Failed to begin transaction: {}", e);
        StatusCode::INTERNAL_SERVER_ERROR
    })?;
    
    let timestamp = get_timestamp();
    let query_sql = format!(
        "/* get_h1b_{} */ SELECT email, first_name, last_name, dob, sex::text, marital_status::text, phone, 
        emergency_contact_name, emergency_contact_phone, employment_start_date,
        street_name, city, state, zip,
        client_name, client_street_name, client_city, client_state, client_zip,
        lca_title, lca_salary, lca_code, receipt_number, h1b_start_date, h1b_end_date, h1b_status::text
        FROM visa_info.h1bcustomer WHERE h1b_status = 'Active'", timestamp
    );
    
    let rows = sqlx::query(&query_sql)
    .fetch_all(&mut *tx)
    .await
    .map_err(|e| {
        eprintln!("Database error in get_all_h1b_customers: {}", e);
        StatusCode::INTERNAL_SERVER_ERROR
    })?;
    
    tx.commit().await.map_err(|e| {
        eprintln!("Failed to commit transaction: {}", e);
        StatusCode::INTERNAL_SERVER_ERROR
    })?;

    let customers: Vec<CreateCustomer> = rows.into_iter().map(|row| CreateCustomer {
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