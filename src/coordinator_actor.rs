use actix::prelude::*;

use super::messages;
use super::worker_actor::WorkerActor;

pub struct CoordinatorActor {
  workers: Vec<Addr<WorkerActor>>,
}

impl Default for CoordinatorActor {
  fn default() -> CoordinatorActor {
    CoordinatorActor { workers: vec![] }
  }
}

impl Actor for CoordinatorActor {
  type Context = Context<Self>;

  fn started(&mut self, _ctx: &mut Self::Context) {
    info!("Starting CoordinatorActor");
  }
}

impl Handler<messages::Connect> for CoordinatorActor {
  type Result = Addr<WorkerActor>;

  fn handle(&mut self, msg: messages::Connect, _: &mut Context<Self>) -> Self::Result {
    info!("WorkerActor connected");
    self.workers.push(msg.addr.clone());
    msg.addr
  }
}

impl Handler<messages::RefreshTableSources> for CoordinatorActor {
  type Result = ();

  fn handle(&mut self, msg: messages::RefreshTableSources, _: &mut Context<Self>) -> Self::Result {
    for worker in &self.workers {
      let message = messages::RefreshTableSources {
        table_sources: msg.table_sources.clone(),
      };
      worker.do_send(message);
    }
  }
}

impl Handler<messages::RefreshFunctionSources> for CoordinatorActor {
  type Result = ();

  fn handle(
    &mut self,
    msg: messages::RefreshFunctionSources,
    _: &mut Context<Self>,
  ) -> Self::Result {
    for worker in &self.workers {
      let message = messages::RefreshFunctionSources {
        function_sources: msg.function_sources.clone(),
      };
      worker.do_send(message);
    }
  }
}
