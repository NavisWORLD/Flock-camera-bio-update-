use axum::{
    extract::{Request, State},
    http::{header::AUTHORIZATION, StatusCode},
    middleware::Next,
    response::{IntoResponse, Response},
    Json,
};
use serde::Serialize;

#[derive(Debug, Clone)]
pub struct AuthConfig {
    api_key: Option<String>,
}

impl AuthConfig {
    pub fn new(api_key: Option<String>) -> Self {
        Self {
            api_key: api_key.filter(|value| !value.trim().is_empty()),
        }
    }
}

#[derive(Debug, Serialize)]
struct AuthError {
    error: &'static str,
}

pub async fn require_api_key(
    State(config): State<AuthConfig>,
    request: Request,
    next: Next,
) -> Response {
    let authorization = request
        .headers()
        .get(AUTHORIZATION)
        .and_then(|value| value.to_str().ok());

    if bearer_matches(authorization, config.api_key.as_deref()) {
        next.run(request).await
    } else {
        (
            StatusCode::UNAUTHORIZED,
            Json(AuthError {
                error: "missing or invalid bearer API key",
            }),
        )
            .into_response()
    }
}

pub fn bearer_matches(header: Option<&str>, expected_key: Option<&str>) -> bool {
    let Some(expected_key) = expected_key else {
        return false;
    };
    let Some(header) = header else {
        return false;
    };
    let Some(provided_key) = header.strip_prefix("Bearer ") else {
        return false;
    };
    constant_time_eq(provided_key.as_bytes(), expected_key.as_bytes())
}

fn constant_time_eq(left: &[u8], right: &[u8]) -> bool {
    if left.len() != right.len() {
        return false;
    }
    let mut difference = 0_u8;
    for (&a, &b) in left.iter().zip(right.iter()) {
        difference |= a ^ b;
    }
    difference == 0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn missing_server_key_denies_every_request() {
        assert!(!bearer_matches(Some("Bearer anything"), None));
    }

    #[test]
    fn correct_bearer_key_is_accepted() {
        assert!(bearer_matches(Some("Bearer secret-value"), Some("secret-value")));
    }

    #[test]
    fn wrong_or_malformed_bearer_key_is_rejected() {
        assert!(!bearer_matches(Some("Bearer wrong"), Some("secret-value")));
        assert!(!bearer_matches(Some("Basic secret-value"), Some("secret-value")));
        assert!(!bearer_matches(None, Some("secret-value")));
    }
}
