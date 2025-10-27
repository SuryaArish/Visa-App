use axum::{
    extract::Path,
    http::StatusCode,
    response::Json,
};
use sqlx::{Row, Executor};
use crate::models::*;
use crate::config::database::get_db_pool;
use std::time::{SystemTime, UNIX_EPOCH};

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

pub async fn create_visa_details(
    Json(payload): Json<CreateCompleteCustomerRequest>,
) -> Result<Json<serde_json::Value>, StatusCode> {
    println!("🔥 create_visa_details function called");
    let pool = get_db_pool();
    let h1b_status = payload.h1b_status.as_deref().unwrap_or("Active");
    
    let raw_sql = format!("INSERT INTO visa_db.h1bcustomer (
            email, first_name, last_name, dob, sex, marital_status, phone,
            emergency_contact_name, emergency_contact_phone, employment_start_date,
            street_name, city, state, zip,
            client_name, client_street_name, client_city, client_state, client_zip,
            lca_title, lca_salary, lca_code, receipt_number, h1b_start_date, h1b_end_date, h1b_status
        ) VALUES (
            '{}', '{}', '{}', '{}', '{}'::visa_db.sex_enum, '{}'::visa_db.marital_status_enum, '{}',
            '{}', '{}', '{}', '{}', '{}', '{}', '{}', '{}', '{}', '{}', '{}', '{}', '{}', {}, '{}', '{}', '{}', '{}', '{}'::visa_db.h1b_status_enum
        )",
        payload.email.replace("'", "''"), payload.first_name.replace("'", "''"), payload.last_name.replace("'", "''"), 
        payload.dob, payload.sex, payload.marital_status, payload.phone.replace("'", "''"),
        payload.emergency_contact_name.replace("'", "''"), payload.emergency_contact_phone.replace("'", "''"), payload.employment_start_date,
        payload.street_name.replace("'", "''"), payload.city.replace("'", "''"), payload.state.replace("'", "''"), payload.zip.replace("'", "''"),
        payload.client_name.replace("'", "''"), payload.client_street_name.replace("'", "''"), payload.client_city.replace("'", "''"), 
        payload.client_state.replace("'", "''"), payload.client_zip.replace("'", "''"),
        payload.lca_title.replace("'", "''"), payload.lca_salary, payload.lca_code.replace("'", "''"), 
        payload.receipt_number.replace("'", "''"), payload.h1b_start_date, payload.h1b_end_date, h1b_status
    );
    
    match pool.execute(raw_sql.as_str()).await {
        Ok(result) => {
            Ok(Json(serde_json::json!({
                "message": "Visa details created successfully",
                "email": payload.email,
                "rows_affected": result.rows_affected()
            })))
        },
        Err(e) => {
            eprintln!("❌ Database error in create_visa_details: {}", e);
            eprintln!("❌ Error details: {:?}", e);
            eprintln!("❌ SQL Query: {}", raw_sql);
            eprintln!("❌ Email: {}", payload.email);
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}

pub async fn get_all_customers_with_status() -> Result<Json<Vec<serde_json::Value>>, StatusCode> {
    println!("🔥 get_all_customers_with_status function called");
    let pool = get_db_pool();
    
    let timestamp = std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap().as_nanos();
    let raw_sql = format!("SELECT customer_id, email, first_name, last_name, dob, sex::text, marital_status::text, phone, 
        emergency_contact_name, emergency_contact_phone, employment_start_date,
        street_name, city, state, zip,
        client_name, client_street_name, client_city, client_state, client_zip,
        lca_title, lca_salary, lca_code, receipt_number, h1b_start_date, h1b_end_date, h1b_status::text
        FROM visa_db.h1bcustomer WHERE h1b_status = 'Active' -- {}", timestamp);
    
    let rows = pool.fetch_all(raw_sql.as_str())
    .await
    .map_err(|e| {
        eprintln!("❌ Database error in get_all_customers_with_status: {}", e);
        eprintln!("❌ Error details: {:?}", e);
        eprintln!("❌ SQL Query: {}", raw_sql);
        StatusCode::INTERNAL_SERVER_ERROR
    })?;

    let customers: Vec<serde_json::Value> = rows.into_iter().map(|row| {
        serde_json::json!({
            "customer_id": row.get::<uuid::Uuid, _>("customer_id"),
            "email": row.get::<String, _>("email"),
            "first_name": row.get::<String, _>("first_name"),
            "last_name": row.get::<String, _>("last_name"),
            "dob": row.get::<chrono::NaiveDate, _>("dob"),
            "sex": row.get::<String, _>("sex"),
            "marital_status": row.get::<String, _>("marital_status"),
            "phone": row.get::<String, _>("phone"),
            "emergency_contact_name": row.get::<String, _>("emergency_contact_name"),
            "emergency_contact_phone": row.get::<String, _>("emergency_contact_phone"),
            "employment_start_date": row.get::<chrono::NaiveDate, _>("employment_start_date"),
            "street_name": row.get::<String, _>("street_name"),
            "city": row.get::<String, _>("city"),
            "state": row.get::<String, _>("state"),
            "zip": row.get::<String, _>("zip"),
            "client_name": row.get::<String, _>("client_name"),
            "client_street_name": row.get::<String, _>("client_street_name"),
            "client_city": row.get::<String, _>("client_city"),
            "client_state": row.get::<String, _>("client_state"),
            "client_zip": row.get::<String, _>("client_zip"),
            "lca_title": row.get::<String, _>("lca_title"),
            "lca_salary": row.get::<rust_decimal::Decimal, _>("lca_salary"),
            "lca_code": row.get::<String, _>("lca_code"),
            "receipt_number": row.get::<String, _>("receipt_number"),
            "h1b_start_date": row.get::<chrono::NaiveDate, _>("h1b_start_date"),
            "h1b_end_date": row.get::<chrono::NaiveDate, _>("h1b_end_date"),
            "h1b_status": row.get::<String, _>("h1b_status")
        })
    }).collect();

    Ok(Json(customers))
}

pub async fn get_customer_by_id(
    Path(customer_id): Path<String>,
) -> Result<Json<serde_json::Value>, StatusCode> {
    let pool = get_db_pool();
    
    let raw_sql = format!("SELECT customer_id, email, first_name, last_name, dob, sex::text, marital_status::text, phone, 
        emergency_contact_name, emergency_contact_phone, employment_start_date,
        street_name, city, state, zip,
        client_name, client_street_name, client_city, client_state, client_zip,
        lca_title, lca_salary, lca_code, receipt_number, h1b_start_date, h1b_end_date, h1b_status::text
        FROM visa_db.h1bcustomer WHERE customer_id::text = '{}' AND h1b_status = 'Active'", customer_id.replace("'", "''"));
    
    match pool.fetch_optional(raw_sql.as_str())
        .await {
        Ok(Some(row)) => {
            let mut response = serde_json::Map::new();
            response.insert("customer_id".to_string(), serde_json::json!(row.get::<uuid::Uuid, _>("customer_id")));
            response.insert("email".to_string(), serde_json::json!(row.get::<String, _>("email")));
            response.insert("first_name".to_string(), serde_json::json!(row.get::<String, _>("first_name")));
            response.insert("last_name".to_string(), serde_json::json!(row.get::<String, _>("last_name")));
            response.insert("dob".to_string(), serde_json::json!(row.get::<chrono::NaiveDate, _>("dob")));
            response.insert("sex".to_string(), serde_json::json!(row.get::<String, _>("sex")));
            response.insert("marital_status".to_string(), serde_json::json!(row.get::<String, _>("marital_status")));
            response.insert("phone".to_string(), serde_json::json!(row.get::<String, _>("phone")));
            response.insert("emergency_contact_name".to_string(), serde_json::json!(row.get::<String, _>("emergency_contact_name")));
            response.insert("emergency_contact_phone".to_string(), serde_json::json!(row.get::<String, _>("emergency_contact_phone")));
            response.insert("employment_start_date".to_string(), serde_json::json!(row.get::<chrono::NaiveDate, _>("employment_start_date")));
            response.insert("street_name".to_string(), serde_json::json!(row.get::<String, _>("street_name")));
            response.insert("city".to_string(), serde_json::json!(row.get::<String, _>("city")));
            response.insert("state".to_string(), serde_json::json!(row.get::<String, _>("state")));
            response.insert("zip".to_string(), serde_json::json!(row.get::<String, _>("zip")));
            response.insert("client_name".to_string(), serde_json::json!(row.get::<String, _>("client_name")));
            response.insert("client_street_name".to_string(), serde_json::json!(row.get::<String, _>("client_street_name")));
            response.insert("client_city".to_string(), serde_json::json!(row.get::<String, _>("client_city")));
            response.insert("client_state".to_string(), serde_json::json!(row.get::<String, _>("client_state")));
            response.insert("client_zip".to_string(), serde_json::json!(row.get::<String, _>("client_zip")));
            response.insert("lca_title".to_string(), serde_json::json!(row.get::<String, _>("lca_title")));
            response.insert("lca_salary".to_string(), serde_json::json!(row.get::<rust_decimal::Decimal, _>("lca_salary")));
            response.insert("lca_code".to_string(), serde_json::json!(row.get::<String, _>("lca_code")));
            response.insert("receipt_number".to_string(), serde_json::json!(row.get::<String, _>("receipt_number")));
            response.insert("h1b_start_date".to_string(), serde_json::json!(row.get::<chrono::NaiveDate, _>("h1b_start_date")));
            response.insert("h1b_end_date".to_string(), serde_json::json!(row.get::<chrono::NaiveDate, _>("h1b_end_date")));
            response.insert("h1b_status".to_string(), serde_json::json!(row.get::<String, _>("h1b_status")));
            Ok(Json(serde_json::Value::Object(response)))
        },
        Ok(None) => {
            Ok(Json(serde_json::json!({
                "message": "Data not found"
            })))
        },
        Err(e) => {
            eprintln!("❌ Database error in get_customer_by_id: {}", e);
            eprintln!("❌ Error details: {:?}", e);
            eprintln!("❌ Customer ID: {}", customer_id);
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}

pub async fn get_customer_by_email(
    Path(email): Path<String>,
) -> Result<Json<Vec<serde_json::Value>>, StatusCode> {
    let pool = get_db_pool();
    
    let raw_sql = format!("SELECT customer_id, email, first_name, last_name, dob, sex::text, marital_status::text, phone, 
        emergency_contact_name, emergency_contact_phone, employment_start_date,
        street_name, city, state, zip,
        client_name, client_street_name, client_city, client_state, client_zip,
        lca_title, lca_salary, lca_code, receipt_number, h1b_start_date, h1b_end_date, h1b_status::text
        FROM visa_db.h1bcustomer WHERE email = '{}' AND h1b_status = 'Active'", email.replace("'", "''"));
    
    match pool.fetch_all(raw_sql.as_str())
        .await {
        Ok(rows) => {
            if rows.is_empty() {
                Ok(Json(vec![serde_json::json!({
                    "message": "Data not found"
                })]))
            } else {
                let customers: Vec<serde_json::Value> = rows.into_iter().map(|row| {
                    serde_json::json!({
                        "customer_id": row.get::<uuid::Uuid, _>("customer_id"),
                        "email": row.get::<String, _>("email"),
                        "first_name": row.get::<String, _>("first_name"),
                        "last_name": row.get::<String, _>("last_name"),
                        "dob": row.get::<chrono::NaiveDate, _>("dob"),
                        "sex": row.get::<String, _>("sex"),
                        "marital_status": row.get::<String, _>("marital_status"),
                        "phone": row.get::<String, _>("phone"),
                        "emergency_contact_name": row.get::<String, _>("emergency_contact_name"),
                        "emergency_contact_phone": row.get::<String, _>("emergency_contact_phone"),
                        "employment_start_date": row.get::<chrono::NaiveDate, _>("employment_start_date"),
                        "street_name": row.get::<String, _>("street_name"),
                        "city": row.get::<String, _>("city"),
                        "state": row.get::<String, _>("state"),
                        "zip": row.get::<String, _>("zip"),
                        "client_name": row.get::<String, _>("client_name"),
                        "client_street_name": row.get::<String, _>("client_street_name"),
                        "client_city": row.get::<String, _>("client_city"),
                        "client_state": row.get::<String, _>("client_state"),
                        "client_zip": row.get::<String, _>("client_zip"),
                        "lca_title": row.get::<String, _>("lca_title"),
                        "lca_salary": row.get::<rust_decimal::Decimal, _>("lca_salary"),
                        "lca_code": row.get::<String, _>("lca_code"),
                        "receipt_number": row.get::<String, _>("receipt_number"),
                        "h1b_start_date": row.get::<chrono::NaiveDate, _>("h1b_start_date"),
                        "h1b_end_date": row.get::<chrono::NaiveDate, _>("h1b_end_date"),
                        "h1b_status": row.get::<String, _>("h1b_status")
                    })
                }).collect();
                Ok(Json(customers))
            }
        },
        Err(e) => {
            eprintln!("❌ Database error in get_customer_by_email: {}", e);
            eprintln!("❌ Error details: {:?}", e);
            eprintln!("❌ Email: {}", email);
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}

pub async fn soft_delete_customer_by_id(
    Path(customer_id): Path<String>,
) -> Result<Json<serde_json::Value>, StatusCode> {
    println!("🔥 soft_delete_customer_by_id function called for customer_id: {}", customer_id);
    let pool = get_db_pool();

    match sqlx::query("UPDATE visa_db.h1bcustomer SET h1b_status = 'Inactive' WHERE customer_id = $1::uuid AND h1b_status = 'Active'")
        .bind(&customer_id)
        .execute(&pool)
        .await 
    {
        Ok(result) => {
            if result.rows_affected() == 0 {
                Ok(Json(serde_json::json!({
                    "status": 404,
                    "message": "Record not found in the database",
                    "customer_id": customer_id
                })))
            } else {
                Ok(Json(serde_json::json!({
                    "message": "Customer soft deleted successfully",
                    "customer_id": customer_id,
                    "rows_affected": result.rows_affected()
                })))
            }
        },
        Err(e) => {
            eprintln!("❌ Database error in soft_delete_customer_by_id: {}", e);
            eprintln!("❌ Soft delete error details: {:?}", e);
            eprintln!("❌ Customer ID: {}", customer_id);
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}

pub async fn update_customer_by_id(
    Path(customer_id): Path<String>,
    Json(payload): Json<serde_json::Value>,
) -> Result<Json<serde_json::Value>, StatusCode> {
    println!("🔥 update_customer_by_id function called for customer_id: {}", customer_id);
    let pool = get_db_pool();

    let raw_sql = format!("UPDATE visa_db.h1bcustomer SET 
        email = '{}', first_name = '{}', last_name = '{}', dob = '{}', 
        sex = '{}'::visa_db.sex_enum, marital_status = '{}'::visa_db.marital_status_enum, 
        phone = '{}', emergency_contact_name = '{}', emergency_contact_phone = '{}', 
        employment_start_date = '{}', street_name = '{}', city = '{}', state = '{}', 
        zip = '{}', client_name = '{}', client_street_name = '{}', client_city = '{}', 
        client_state = '{}', client_zip = '{}', lca_title = '{}', lca_salary = {}, 
        lca_code = '{}', receipt_number = '{}', h1b_start_date = '{}', h1b_end_date = '{}'
        WHERE customer_id = '{}'::uuid",
        payload["email"].as_str().unwrap_or("").replace("'", "''"),
        payload["first_name"].as_str().unwrap_or("").replace("'", "''"),
        payload["last_name"].as_str().unwrap_or("").replace("'", "''"),
        payload["dob"].as_str().unwrap_or(""),
        payload["sex"].as_str().unwrap_or(""),
        payload["marital_status"].as_str().unwrap_or(""),
        payload["phone"].as_str().unwrap_or("").replace("'", "''"),
        payload["emergency_contact_name"].as_str().unwrap_or("").replace("'", "''"),
        payload["emergency_contact_phone"].as_str().unwrap_or("").replace("'", "''"),
        payload["employment_start_date"].as_str().unwrap_or(""),
        payload["street_name"].as_str().unwrap_or("").replace("'", "''"),
        payload["city"].as_str().unwrap_or("").replace("'", "''"),
        payload["state"].as_str().unwrap_or("").replace("'", "''"),
        payload["zip"].as_str().unwrap_or("").replace("'", "''"),
        payload["client_name"].as_str().unwrap_or("").replace("'", "''"),
        payload["client_street_name"].as_str().unwrap_or("").replace("'", "''"),
        payload["client_city"].as_str().unwrap_or("").replace("'", "''"),
        payload["client_state"].as_str().unwrap_or("").replace("'", "''"),
        payload["client_zip"].as_str().unwrap_or("").replace("'", "''"),
        payload["lca_title"].as_str().unwrap_or("").replace("'", "''"),
        payload["lca_salary"].as_str().unwrap_or("0"),
        payload["lca_code"].as_str().unwrap_or("").replace("'", "''"),
        payload["receipt_number"].as_str().unwrap_or("").replace("'", "''"),
        payload["h1b_start_date"].as_str().unwrap_or(""),
        payload["h1b_end_date"].as_str().unwrap_or(""),
        customer_id.replace("'", "''")
    );

    match pool.execute(raw_sql.as_str()).await {
        Ok(result) => {
            if result.rows_affected() > 0 {
                Ok(Json(serde_json::json!({
                    "message": "Customer updated successfully",
                    "customer_id": customer_id,
                    "rows_affected": result.rows_affected()
                })))
            } else {
                Ok(Json(serde_json::json!({
                    "message": "Customer not found",
                    "customer_id": customer_id
                })))
            }
        },
        Err(e) => {
            eprintln!("❌ Database error in update_customer_by_id: {}", e);
            eprintln!("❌ Error details: {:?}", e);
            eprintln!("❌ Customer ID: {}", customer_id);
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}