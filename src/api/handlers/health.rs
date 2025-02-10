use {crate::core, crate::utils};

#[ntex::web::get("/health")]
async fn healthcheck() -> core::result::ApiResult {
    Ok(utils::response::json(
        200_u16,
        &serde_json::json!({"successful": true}),
    ))
}
