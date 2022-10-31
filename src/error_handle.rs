use std::fmt;
use std::fmt::Formatter;
use actix_web::http::StatusCode;
use actix_web::{HttpResponse, HttpResponseError};
use diesel::result::Error as DieselError;
use serde_json::json;

//定义一个自定义错误
#[derive(Debug, Serialize, Deserialize)]
pub struct CustomError{  
   pub error_status_code: u16,
   pub error_message: String,
}

impl CustomError {
    // 当有一个错误的时候 构造一个错误对象
    pub fn new(error_status_code: u16, error_message: String) -> CustomError {
        CustomError{
            error_status_code, 
            error_message,
        }
    }
}

impl fmt::Display for CustomError{
    // 实现打印信息
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result { 
        f.write_str(self.error_message.as_str())
    }
}

impl From<DieselError> for CustomError{
    // 当有orm  DieselError 如何处理它
    fn from(error: DieselError) -> CustomError{
        match error{
            DieselError::DatabaseError(_, err)=>CustomError::new(409, err.message().as_str()),
            DieselError::NotFound => {
                CustomError::new(404, "雇员没有找到".as_str())
            }
            err => CustomError::new(500, format!("未知的Diesel错误：{}", err)),
        }
    }
}

impl ResponseError for CustomError{
    // 当有 ResponseError 如何处理  
    fn error_response(&self) -> HttpResponse{
        let StatusCode = match StatusCode::from_u16(self.error_status_code){
            Ok(status_code) => status_code,
            Err(_) => StatusCode::INTERNAL_SERVER_ERROR,
        };

        let error_message = match   status_code.as_u16() <500{
            true => self.error_message.clone(),
            false => "服务器内部错误".to_string(),
         };
         HttpResponse::build(status_code).json(json!({"message": error_message}))
    }
}