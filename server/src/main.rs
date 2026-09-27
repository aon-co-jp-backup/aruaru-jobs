//! aruaru-jobs: フリーランス案件をまとめて検索できる集約サイト。
//!
//! 検索自体はaruaru-llm(`/v1/search/raw`、source=google、aruaru-search
//! 優先・無制限・APIキー不要)へ薄く委譲する。このサーバー自身は
//! (1) 静的フロントエンド配信 (2) `/api/jobs/search`での検索プロキシ、
//! の2つだけを行う。各案件カードには「Maid Cafe Programming Schoolで
//! 開発」ボタンがあり、`vscode://aon-co-jp.maid-cafe-programming-school/
//! import-job?title=...&url=...&snippet=...` へ飛んで、その案件を
//! 種に一緒にプログラミング学習しながら開発を始められる。

use open_runo_poem_compat::{get, post, Json, Request, Response, Route, Server, StatusCode, TcpListener};
use serde::{Deserialize, Serialize};
use std::path::PathBuf;

fn aruaru_llm_base_url() -> String {
    std::env::var("ARUARU_LLM_BASE_URL").unwrap_or_else(|_| "http://127.0.0.1:4600".to_string())
}

fn static_dir() -> PathBuf {
    std::env::var("ARUARU_JOBS_STATIC_DIR")
        .map(PathBuf::from)
        .unwrap_or_else(|_| PathBuf::from("web"))
}

#[derive(Deserialize)]
struct JobSearchRequest {
    query: String,
}

#[derive(Serialize)]
struct JobSearchResponse {
    query: String,
    results: serde_json::Value,
}

#[derive(Serialize)]
struct ErrorResponse {
    error: String,
}

fn static_asset(path: &str, content_type: &'static str) -> open_runo_poem_compat::hyper_compat::Handler {
    open_runo_poem_compat::hyper_compat::static_file_handler(static_dir().join(path), content_type)
}

async fn job_search_handler(
    req: Request,
    _params: open_runo_poem_compat::hyper_compat::Params,
) -> Response {
    let body: JobSearchRequest = match Json::from_body(req).await {
        Ok(Json(v)) => v,
        Err(resp) => return resp,
    };

    if body.query.trim().is_empty() {
        return Json(ErrorResponse {
            error: "query must not be empty".to_string(),
        })
        .into_response(StatusCode::BAD_REQUEST);
    }

    let client = reqwest::Client::new();
    let payload = serde_json::json!({
        "source": "google",
        "query": format!("フリーランス 案件 {}", body.query),
        "max_results": 10,
        "free_only": true,
    });

    let url = format!("{}/v1/search/raw", aruaru_llm_base_url());
    match client.post(&url).json(&payload).send().await {
        Ok(resp) => match resp.json::<serde_json::Value>().await {
            Ok(results) => Json(JobSearchResponse {
                query: body.query,
                results,
            })
            .into_response(StatusCode::OK),
            Err(e) => Json(ErrorResponse {
                error: format!("aruaru-llm returned an unparseable response: {e}"),
            })
            .into_response(StatusCode::BAD_GATEWAY),
        },
        Err(e) => Json(ErrorResponse {
            error: format!("could not reach aruaru-llm at {url}: {e}"),
        })
        .into_response(StatusCode::BAD_GATEWAY),
    }
}

#[tokio::main]
async fn main() -> std::io::Result<()> {
    let bind_addr: std::net::SocketAddr = std::env::var("ARUARU_JOBS_BIND_ADDR")
        .unwrap_or_else(|_| "0.0.0.0:4700".to_string())
        .parse()
        .expect("ARUARU_JOBS_BIND_ADDR must be a valid socket address");

    let app = Route::new()
        .at("/", get(static_asset("index.html", "text/html; charset=utf-8")))
        .at(
            "/app.js",
            get(static_asset("app.js", "text/javascript; charset=utf-8")),
        )
        .at(
            "/style.css",
            get(static_asset("style.css", "text/css; charset=utf-8")),
        )
        .at(
            "/api/jobs/search",
            post(open_runo_poem_compat::handler_fn(job_search_handler)),
        )
        .with_cors()
        .with_compression();

    let (addr, handle) = Server::new(TcpListener::bind(bind_addr)).run(app).await?;
    println!("aruaru-jobs-server listening on {addr}");
    handle.await.expect("server task panicked");
    Ok(())
}
