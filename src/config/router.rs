use actix_web::web;
use crate::handlers::user_handler::{
    health_checker_handler, create_item_handler, get_items_handler,
    get_item_handler, update_item_handler, delete_item_handler,
};

// API 경로 config 설정
pub fn config(conf: &mut web::ServiceConfig) {
    let scope = web::scope("/api")
        .service(health_checker_handler) // 건강 체크 
        .service(create_item_handler)    // 유저 생성 
        .service(get_items_handler)      // 모든 유저 조회 
        .service(get_item_handler)       // 특정 유저 조회 
        .service(update_item_handler)    // 유저 수정 
        .service(delete_item_handler);   // 유저 삭제 

    conf.service(scope);
}