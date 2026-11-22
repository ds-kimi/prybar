use axum::body::Body;
use axum::http::{Request, StatusCode};
use tower::ServiceExt as _;

use crate::app::{router, State};

fn app() -> axum::Router {
    let conn = crate::db::open(":memory:").expect("db");
    router(State::new(conn, Some("0123456789abcdef".into())))
}

async fn post(app: &axum::Router, body: &str) -> StatusCode {
    let req = Request::post("/v1/event")
        .body(Body::from(body.to_string()))
        .expect("request");
    app.clone().oneshot(req).await.expect("response").status()
}

#[tokio::test]
async fn ingest_validates_and_counts() {
    let app = app();
    let start = r#"{"event":"app_start","version":"0.1.0","os":"macos","arch":"aarch64"}"#;
    assert_eq!(post(&app, start).await, StatusCode::NO_CONTENT);
    assert_eq!(post(&app, start).await, StatusCode::NO_CONTENT);
    let crash = r#"{"event":"crash","version":"0.1.0","os":"macos","arch":"aarch64","message":"boom","frames":["prybar_gui::main"]}"#;
    assert_eq!(post(&app, crash).await, StatusCode::NO_CONTENT);
    assert_eq!(post(&app, "not json").await, StatusCode::BAD_REQUEST);
    let extra = r#"{"event":"app_start","version":"0.1.0","os":"macos","arch":"x","ip":"1.2.3.4"}"#;
    assert_eq!(post(&app, extra).await, StatusCode::BAD_REQUEST);
    assert_eq!(
        post(&app, &"x".repeat(20_000)).await,
        StatusCode::PAYLOAD_TOO_LARGE
    );
    stats_page_needs_the_token(&app).await;
}

async fn stats_page_needs_the_token(app: &axum::Router) {
    let page = |auth: &str| {
        Request::get("/stats")
            .header("authorization", auth)
            .body(Body::empty())
            .expect("request")
    };
    let bad = app
        .clone()
        .oneshot(page("Bearer nope"))
        .await
        .expect("response");
    assert_eq!(bad.status(), StatusCode::UNAUTHORIZED);
    let ok = app
        .clone()
        .oneshot(page("Bearer 0123456789abcdef"))
        .await
        .expect("response");
    assert_eq!(ok.status(), StatusCode::OK);
    let html = axum::body::to_bytes(ok.into_body(), 1 << 20)
        .await
        .expect("body");
    let html = String::from_utf8_lossy(&html);
    assert!(
        html.contains("<td>app_start</td><td>0.1.0</td><td>macos</td><td>aarch64</td><td>2</td>"),
        "{html}"
    );
    assert!(html.contains("prybar_gui::main"));
}
