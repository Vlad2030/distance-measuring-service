use {crate::core, crate::models, crate::utils};

#[ntex::web::post("/distance")]
async fn get_distance(
    json: core::result::JsonResult<models::distance::Distance>
) -> core::result::ApiResult {
    if let Err(e) = json {
        return Err(e.into());
    }

    let distance_query = json.unwrap().into_inner();
    let response = distance_query.calculate()?;

    Ok(utils::response::json(200_u16, &response))
}

#[ntex::web::get("/units")]
async fn get_units() -> core::result::ApiResult {
    Ok(utils::response::json(
        200_u16,
        &serde_json::json!({"units": models::unit::Unit::as_vec()}),
    ))
}
