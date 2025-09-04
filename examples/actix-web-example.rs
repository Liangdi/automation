use std::sync::{Arc, Mutex};

use actix_web::{post, web, HttpServer, Responder};
use automation::{enums::InputAction, executor::ActionExecutor};
use tracing::info;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let executor = Arc::new(Mutex::new(ActionExecutor::new()));
    HttpServer::new(move || {
        actix_web::App::new()
            .app_data(web::Data::new(executor.clone()))
            .service(execute_action)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await?;  
    Ok(())
}

// 处理动作执行
#[post("execute")]
async fn execute_action(
    action: web::Json<InputAction>,
    executor: web::Data<Arc<Mutex<ActionExecutor>>>,
) -> impl Responder {
    info!("Received action: {:?}", action);

    let executor = executor.lock().unwrap();
    let (result, duration) = executor.execute(&action).await;

    info!("Executed in {}ms: {}", duration, result);
    // let resp = HttpResponse::Ok().json(serde_json::json!({
    //     "status": "success",
    //     "execution_time_ms": duration,
    //     "result": result
    // }));

    "Success"
}
