use actix_web::{delete, get, patch, post, web, HttpResponse, Responder};
use serde_json::json;
use sqlx::PgPool;
use crate::models::user_item::User;
use crate::models::user_item::UpdateUser;

// 서버 상태 확인을 위한 핸들러
#[get("/healthchecker")]
async fn health_checker_handler() -> impl Responder {
    const MESSAGE: &str = "Web Server is running";

    HttpResponse::Ok().json(json!({"status": "success","message": MESSAGE}))
}

// 새로운 사용자 항목 생성 핸들러
#[post("/create-item")]
async fn create_item_handler(pool: web::Data<PgPool>, item: web::Json<User>) -> impl Responder {
    let result = sqlx::query!(
        "INSERT INTO newtable (id,name,email) VALUES ($1,$2,$3) RETURNING id, name, email",
        item.id, item.name, item.email
    )
    .fetch_one(pool.get_ref())
    .await;

    match result {
        Ok(row) => HttpResponse::Created().json(json!({"status": "success", "data": {"id": row.id, "name": row.name, "email": row.email}})),
        Err(_) => HttpResponse::InternalServerError().json(json!({"status": "error", "message": "Failed to create item"})),
    }
}

// 모든 사용자 항목 조회 핸들러
#[get("/get-items")]
async fn get_items_handler(pool: web::Data<PgPool>) -> impl Responder {
    let result = sqlx::query!("SELECT * FROM newtable")
        .fetch_all(pool.get_ref())
        .await;
    match result {
        Ok(rows) => {
            let items: Vec<_> = rows.iter().map(|row| {
                json!({"id": row.id, "name": row.name})
            }).collect();
            HttpResponse::Ok().json(json!({"status": "success", "data": items}))
        }
        Err(_) => HttpResponse::InternalServerError().json(json!({"status": "error", "message": "Failed to get items"})),
    }
}

// 특정 ID의 사용자 항목 조회 핸들러
#[get("/get-item/{id}")]
async fn get_item_handler(pool: web::Data<PgPool>, path: web::Path<i32>) -> impl Responder {
    let id: i32 = path.into_inner(); // 경로에서 추출한 ID

    // 데이터베이스에서 항목 조회
    let result = sqlx::query!(
        "SELECT id, name, email FROM newtable WHERE id = $1",
        id
    )
    .fetch_one(pool.get_ref())
    .await;

    match result {
        Ok(row) => HttpResponse::Ok().json(json!({"status": "success", "data": {"id": row.id, "name": row.name, "email": row.email}})),
        Err(sqlx::Error::RowNotFound) => HttpResponse::NotFound().json(json!({"status": "error", "message": "Item not found"})),
        Err(_) => HttpResponse::InternalServerError().json(json!({"status": "error", "message": "Failed to fetch item"})),
    }
}

// 특정 ID의 사용자 항목 업데이트 핸들러
#[patch("/update-item/{id}")]
async fn update_item_handler(
    pool: web::Data<PgPool>,
    path: web::Path<i32>,
    item: web::Json<UpdateUser>,
) -> impl Responder {
    let id = path.into_inner();

    let result = sqlx::query!(
        "UPDATE newtable SET name = COALESCE($1, name), email = COALESCE($2, email) WHERE id = $3 RETURNING id, name, email",
        item.name,
        item.email,
        id
    )
    .fetch_one(pool.get_ref())
    .await;

    match result {
        Ok(row) => HttpResponse::Ok().json(json!({"status": "success", "data": {"id": row.id, "name": row.name, "email": row.email}})),
        Err(sqlx::Error::RowNotFound) => HttpResponse::NotFound().json(json!({"status": "error", "message": "Item not found"})),
        Err(_) => HttpResponse::InternalServerError().json(json!({"status": "error", "message": "Failed to update item"})),
    }
}

// 특정 ID의 사용자 항목 삭제 핸들러
#[delete("/delete-item/{id}")]
async fn delete_item_handler(pool: web::Data<PgPool>,path: web::Path<i32>) -> impl Responder {
    let id = path.into_inner();

    let result = sqlx::query!("DELETE FROM newtable WHERE id = $1",id)
                .execute(pool.get_ref())
                .await;

    match result {
        Ok(_) => HttpResponse::Ok().json(json!({"status": "success", "message": "Item deleted"})),
        Err(_) => HttpResponse::InternalServerError().json(json!({"status": "error", "message": "Failed to delete item"})),
    }
}
