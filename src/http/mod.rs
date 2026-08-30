mod health;
mod github;
use super::env;


use axum::{
     Router, routing::get,
};
// tower is used for oneshot in test, don't remove it even it your linter is screaming
use tower::ServiceExt; 
use octocrab::Octocrab;

#[derive(Clone)]
pub struct AppState {
    octo: Octocrab,
}

fn create_app(config: &env::Config) -> Router {
        let octocrab = Octocrab::builder();

        let octocrab = match config.env.as_str() {
            "prod" => octocrab 
                .personal_token(config.github_token.clone())
                .build()
                .unwrap(),
            _ => octocrab
                .build()
                .unwrap(),
        };

        let state = AppState{ octo: octocrab };
        
         Router::new()
        .route("/", get(health::root))
        .route("/health", get(health::health))
        .route("/prget", get(github::prget))
        .with_state(state)
}

#[tokio::main]
pub async fn serve(config: env::Config) {
    let app = create_app(&config); 

    // I'm new to rust but using the env config was my first time fighting the borrow checker.
    // Cloning the values seem to work, but it cost a bit on the heap. Its not the way to go if you
    // care about borrow checker. Will probably check it later
    let listener = tokio::net::TcpListener::bind((config.host.clone(), config.port)).await.expect("failed to bind tcp listener");

    println!("Starting server at: {}:{}\n", config.host, config.port);
    axum::serve(listener, app).await.expect("failed to start server");
}


// ----- [ Tests ] -----


#[cfg(test)]
mod tests {
    use axum::{body::{Body}, http::{Request, StatusCode}};
use serde_json::Value;

    use super::*;


    
    #[tokio::test]
    async fn test_health() {
        let config = env::Config::get_env();
        let app = create_app(&config);

        let req = Request::builder()
            .uri("/health")
            .body(Body::empty())
            .unwrap();

        let res = app.oneshot(req).await.unwrap();

        assert_eq!(res.status(), StatusCode::OK);

        let body = axum::body::to_bytes(res.into_body(), 1024).await.unwrap();

        let json: Value = serde_json::from_slice(
            &body
        ).unwrap();

        assert_eq!(json["status"], "ok");
        assert_eq!(json["message"], "healthy")
    }
}
