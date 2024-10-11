use serde::{Serialize, Deserialize};

//유저 구조체
#[derive(Serialize, Deserialize, Debug)]
pub struct User {
    pub id: Option<i32>, // 사용자의 고유 ID
    pub name: String,    // 사용자의 이름
    pub email: String,   // 사용자의 이메일 주소
}

// 유저 업데이트 구조체
#[derive(Serialize, Deserialize, Debug)]
pub struct UpdateUser {
    pub name: Option<String>, // 업데이트할 사용자의 이름
    pub email: Option<String>, // 업데이트할 사용자의 이메일 주소
}

