use crate::command::Command;
use crate::command_result::CommandResult;
use tokio::sync::mpsc;
use tokio::sync::oneshot;

pub(crate) type CommandPayload = (Command, oneshot::Sender<CommandResult>);
pub(crate) type CommandSend = mpsc::Sender<CommandPayload>;
pub(crate) type CommandReceive = mpsc::Receiver<CommandPayload>;
