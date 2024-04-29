use event_loop::{try_get, Handled, HandlerStruct, Is};
use super::console::ConsoleOutput;
use crate::state::MACState;
use super::web::broadcast_event;

// Messages *********************

// pub struct NewConsoleMessage(pub Option<String>);

// Handlers *********************

#[allow(clippy::module_name_repetitions)]
pub struct SseBroadcastMessages;
impl<IM, OM> HandlerStruct<MACState, IM, OM> for SseBroadcastMessages
where
    IM: Is<ConsoleOutput>
{
    fn handle_message(
        &mut self,
        _state: &MACState,
        message: &IM,
    ) -> Option<event_loop::Handled<OM>> {
        let console_out: &ConsoleOutput = try_get(message)?;
        let cloned_co = console_out.clone();
        return Handled::<OM>::future(async move {
            broadcast_event(&cloned_co).await;
            None
        });
    }
}
