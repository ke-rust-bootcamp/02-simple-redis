use super::CommandExecutor;
use crate::{cmd::Echo, RespFrame, RespNull};

impl CommandExecutor for Echo {
    fn execute(self, backend: &crate::Backend) -> RespFrame {
        match backend.echo(&self.key) {
            Some(v) => v,
            None => RespFrame::Null(RespNull),
        }
    }
}
