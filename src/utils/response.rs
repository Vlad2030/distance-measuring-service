pub fn json<T>(
    status: u16,
    data: &T,
) -> ntex::web::HttpResponse
where
    for<'de> T: serde::Serialize + serde::Deserialize<'de>,
{
    ntex::web::HttpResponseBuilder::new(ntex::http::StatusCode::from_u16(status).unwrap())
        .json(data)
}
