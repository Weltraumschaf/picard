extern crate pretty_env_logger;
use std::env;
use warp::Filter;
use picard::models;
use picard::filters;

#[macro_use] extern crate log;
#[tokio::main]
async fn main() {
    if env::var_os("RUST_LOG").is_none() {
        // Set `RUST_LOG=picard=debug` to see debug logs,
        // this only shows access logs.
        env::set_var("RUST_LOG", "picard=info");
    }
    pretty_env_logger::init();

    info!("Starting Picard!");
    let db = models::blank_db();
    let api = filters::todos(db)
        .or(warp::fs::dir("public_html"));
    // View access logs by setting `RUST_LOG=picard`.
    let routes = api.with(warp::log("picard"));
    warp::serve(routes)
        .run(([127, 0, 0, 1], 3030))
        .await;
}

#[cfg(test)]
mod tests {
    use warp::http::StatusCode;
    use warp::test::request;

    use super::{
        filters,
        models::{self, Todo},
    };

    #[tokio::test]
    async fn test_post() {
        let db = models::blank_db();
        let api = filters::todos(db);

        let resp = request()
            .method("POST")
            .path("/todos")
            .json(&Todo {
                id: 1,
                text: "test 1".into(),
                completed: false,
            })
            .reply(&api)
            .await;

        assert_eq!(resp.status(), StatusCode::CREATED);
    }

    #[tokio::test]
    async fn test_post_conflict() {
        let db = models::blank_db();
        db.lock().await.push(todo1());
        let api = filters::todos(db);

        let resp = request()
            .method("POST")
            .path("/todos")
            .json(&todo1())
            .reply(&api)
            .await;

        assert_eq!(resp.status(), StatusCode::BAD_REQUEST);
    }

    #[tokio::test]
    async fn test_put_unknown() {
        let _ = pretty_env_logger::try_init();
        let db = models::blank_db();
        let api = filters::todos(db);

        let resp = request()
            .method("PUT")
            .path("/todos/1")
            .header("authorization", "Bearer admin")
            .json(&todo1())
            .reply(&api)
            .await;

        assert_eq!(resp.status(), StatusCode::NOT_FOUND);
    }

    fn todo1() -> Todo {
        Todo {
            id: 1,
            text: "test 1".into(),
            completed: false,
        }
    }
}
