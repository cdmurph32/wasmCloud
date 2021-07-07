use serde_json::json;
use wasmbus_rpc::actor::prelude::*;
use wasmbus_rpc::core::{Actor, ActorReceiver, HealthCheckRequest, HealthCheckResponse};
use wasmcloud_example_httpserver::{HttpRequest, HttpResponse, HttpServer, HttpServerReceiver};

#[derive(Debug, Default, Actor)]
#[services(Actor, HttpServer)]
struct EchoActor {}

/// Implementation of HttpServer trait methods
#[async_trait]
impl HttpServer for EchoActor {
    async fn handle_request(
        &self,
        _ctx: &context::Context<'_>,
        value: &HttpRequest,
    ) -> std::result::Result<HttpResponse, RpcError> {
        let body = json!({
            "method": &value.method,
            "path": &value.path,
            "query_string": &value.query_string,
            "body": b"xyz".to_vec(),
        });
        let resp = HttpResponse {
            body: serde_json::to_vec(&body)
                .map_err(|e| RpcError::ActorHandler(format!("serializing response: {}", e)))?,
            ..Default::default()
        };
        Ok(resp)
    }
}

/// Implementation of Actor trait methods
#[async_trait]
impl Actor for EchoActor {
    async fn health_request(
        &self,
        _ctx: &context::Context<'_>,
        _value: &HealthCheckRequest,
    ) -> std::result::Result<HealthCheckResponse, RpcError> {
        Ok(HealthCheckResponse {
            healthy: false,
            message: Some(String::from("OK")),
        })
    }
}
