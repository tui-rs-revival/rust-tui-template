use std::{sync::Arc, time::Duration};

use anyhow::{anyhow, Context, Result};
use tokio::sync::Mutex;
use tracing::debug;

use crate::{
  action::Action,
  components::{home::Home, Component},
  event::EventHandler,
  tui::Tui,
};

pub struct App {
  pub events: EventHandler,
  pub home: Home,
  pub tui: Tui,
}

impl App {
  pub fn new(tick_rate: u64) -> Result<Self> {
    let tui = Tui::new().context(anyhow!("Unable to create TUI")).unwrap();
    let events = EventHandler::new(tick_rate);
    let mut home = Home::default();
    home.init()?;
    Ok(Self { tui, events, home })
  }

  pub async fn run(&mut self) -> Result<()> {
    loop {
      self
        .tui
        .terminal
        .draw(|f| {
          self.home.render(f, f.size());
        })
        .unwrap();
      let event = self.events.next().await;
      let mut action = Some(self.home.handle_events(event).await);
      while action.is_some() {
        action = self.home.dispatch(action.unwrap()).await;
      }
      if !(self.home.is_running) {
        break;
      }
    }
    Ok(())
  }
}
