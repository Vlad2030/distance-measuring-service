use crate::core;

pub type Result<T> = std::result::Result<T, core::error::Error>;
pub type ApiResult = Result<ntex::web::HttpResponse>;
pub type JsonResult<T> =
    std::result::Result<ntex::web::types::Json<T>, ntex::web::error::JsonPayloadError>;
