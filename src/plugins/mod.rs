
pub mod http;

pub trait InboundPlugin {
  fn listen(&self);
}

pub trait OutboundPlugin {
  fn send(&self);
}