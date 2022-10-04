use aide::openapi::{OpenApi, Info};

const OPENAPI_DOC_VERSION: &str = "0.1.0";

pub fn api_docs() -> OpenApi {
    OpenApi {
        info: Info {
            title: "edihkal".to_string(),
            summary: Some(
                "A service for logging and analyzing data around personal drug use.".to_string(),
            ),
            version: OPENAPI_DOC_VERSION.to_string(),
            ..Info::default()
        },
        ..OpenApi::default()
    }
}
