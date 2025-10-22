use poem::{
  Endpoint, Middleware, Request, Result,
};
use crate::event_store::EventStore;

struct EventStoreMiddleware<'a> {
  store: &'a EventStore,
}

impl<'a> EventStoreMiddleware<'a> {
  fn create(store: &'a EventStore) -> Self {
    EventStoreMiddleware {
      store: store
    }
  }
}

impl<'a, E: Endpoint> Middleware<E> for EventStoreMiddleware<'a> {
  type Output = EventStoreMiddlewareImpl<'a, E>;
  fn transform(&self, ep: E) -> Self::Output {
    EventStoreMiddlewareImpl {
      ep,
      store: self.store
    }
  }
}

struct EventStoreMiddlewareImpl<E> {
  ep: E,
  store: &EventStore,
}

impl<E: Endpoint> Endpoint for EventStoreMiddlewareImpl<E> {
  type Output = E::Output;
  async fn call(&self, mut req: Request) -> Result<Self::Output> {

    //req.extensions_mut().insert(self.store);

    self.ep.call(req).await
  }
}