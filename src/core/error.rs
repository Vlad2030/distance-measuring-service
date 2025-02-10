use {
    crate::models,
    derive_more::{Display, Error as DError},
};

#[derive(Debug, Display, DError, Clone)]
pub enum Error {
    #[display(fmt = "Calculate error, {}", explanation)]
    Calculate {
        explanation: String,
    },

    #[display(fmt = "Internal error, {}", err)]
    Internal {
        err: String,
    },

    #[display(
        fmt = "Invalid field `{}`, it should be {}",
        field,
        explanation
    )]
    InvalidField {
        field: String,
        explanation: String,
    },
}

impl Default for Error {
    fn default() -> Self {
        Self::Internal {
            err: "please try again later".into(),
        }
    }
}

impl ntex::web::error::WebResponseError for Error {
    fn status_code(&self) -> ntex::http::StatusCode {
        match *self {
            Self::Calculate {
                ..
            } => ntex::http::StatusCode::BAD_REQUEST,
            Self::Internal {
                ..
            } => ntex::http::StatusCode::INTERNAL_SERVER_ERROR,
            Self::InvalidField {
                ..
            } => ntex::http::StatusCode::BAD_REQUEST,
        }
    }

    fn error_response(
        &self,
        _: &ntex::web::HttpRequest,
    ) -> ntex::web::HttpResponse {
        let status_code = self.status_code();
        let error_response = models::error::Error {
            detail: self.to_string(),
        };
        ntex::web::HttpResponse::build(status_code).json::<models::error::Error>(&error_response)
    }
}

impl From<ntex::web::error::JsonPayloadError> for Error {
    fn from(err: ntex::web::error::JsonPayloadError) -> Self {
        Error::Internal {
            err: format!("{:?}", err),
        }
    }
}
