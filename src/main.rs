use poem::{listener::TcpListener, Route, Server, Middleware};
use poem_openapi::{payload::Json, OpenApi, OpenApiService, Object};

mod plugins;//::http::HttpDestination;
use plugins::http::HttpDestination;

mod data;
use data::EventStore;

struct EventStoreMiddleware {
    event_store: EventStore,
}
impl<E: poem::Endpoint> Middleware<E> for EventStoreMiddleware {
    type Output = EventStoreEndpoint<E>;

    fn transform(&self, ep: E) -> Self::Output {
        EventStoreEndpoint {
            event_store: self.event_store.clone(),
            inner: ep,
        }
    }
}

#[derive(Object)]
struct EventScheduleRequest {
    event_body: String,
    schedule_time: i64,
    destination: String,//HttpDestination,
}

struct API;

#[OpenApi]
impl API {
    #[oai(path = "/healthcheck", method = "get")]
    async fn healthcheck(&self) -> Json<&'static str> {
        // Eventually, this should provide real health status of dependent services.
        Json("OK")
    }

    #[oai(path = "/schedule_event", method = "post")]
    async fn schedule_event(&self, req: Json<EventScheduleRequest>) -> Json<&'static str> {
        Json("Hello from Poem!")
    }
}

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    let api_service = OpenApiService::new(API, "", "1.0").server("http://localhost:8000/api");
    let ui = api_service.swagger_ui();

    let event_store = EventStore::connect().await.expect("Failed to connect to the database");

    let app = Route::new()
        .at("/api", api_service)
        .at("/docs", ui);
    Server::new(TcpListener::bind("127.0.0.1:8000")).run(app).await
}
