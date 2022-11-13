use anyhow::anyhow;
use common::board::Board;
use common::grid::Position;
use common::json::Name;
use common::{PubPlayerInfo, State};
use players::player::{PlayerApi, PlayerApiError, PlayerApiResult};
use players::strategy::PlayerAction;
use serde::Deserialize;
use std::io::Write;
use std::net::TcpStream;
use std::time::Duration;

use crate::json::{JsonFunctionCall, JsonResult};

/// Acts as a proxy for players across a network
struct PlayerProxy {
    name: Name,
    stream: TcpStream,
}

impl PlayerProxy {
    fn new(name: Name, stream: TcpStream) -> Self {
        Self { name, stream }
    }

    /// Reads a single `JsonResult` from `self.stream`
    ///
    /// # Errors
    /// This will error if reading from the stream or deserializing the `JsonResult` fails
    fn read_result(&self) -> PlayerApiResult<JsonResult> {
        let mut de = serde_json::Deserializer::from_reader(self.stream.try_clone()?);
        Ok(JsonResult::deserialize(&mut de)?)
    }

    /// Writes a `JsonFunctionCall` to `self.stream`
    ///
    /// # Errors
    /// This will error if writing to `self.stream` fails
    fn send_function_call(&self, func: &JsonFunctionCall) -> PlayerApiResult<()> {
        let msg = serde_json::to_string(func)?;
        self.stream.write_all(msg.as_bytes())?;
        Ok(())
    }
}

impl PlayerApi for PlayerProxy {
    fn name(&self) -> PlayerApiResult<Name> {
        Ok(self.name.clone())
    }

    fn propose_board0(&self, cols: u32, rows: u32) -> PlayerApiResult<Board> {
        // the spec doesn't say anything about calling propose_board0 on `PlayerProxy`s
        todo!()
    }

    fn setup(
        &mut self,
        state: Option<State<PubPlayerInfo>>,
        goal: Position,
    ) -> PlayerApiResult<()> {
        // create function call message
        self.send_function_call(&JsonFunctionCall::setup(state, goal))?;
        // TODO: what should this timeout actually be?
        self.stream.set_read_timeout(Some(Duration::from_secs(2)))?;
        match self.read_result()? {
            JsonResult::Void => Ok(()),
            _ => Err(PlayerApiError::Other(anyhow!(
                "Got something other than \"void\", when calling `setup`!"
            ))),
        }
    }

    fn take_turn(&self, state: State<PubPlayerInfo>) -> PlayerApiResult<PlayerAction> {
        self.send_function_call(&JsonFunctionCall::take_turn(state))?;
        // TODO: what should this timeout actually be?
        self.stream.set_read_timeout(Some(Duration::from_secs(2)))?;
        match self.read_result()? {
            JsonResult::Choice(ch) => ch
                .into_action(&state.board)
                .map_err(|e| PlayerApiError::Other(e.into())),
            _ => Err(PlayerApiError::Other(anyhow!(
                "Got something other than a JsonChoice when calling `take_turn`!"
            ))),
        }
    }

    fn won(&mut self, did_win: bool) -> PlayerApiResult<()> {
        self.send_function_call(&JsonFunctionCall::win(did_win))?;
        // TODO: what should this timeout actually be?
        self.stream.set_read_timeout(Some(Duration::from_secs(2)))?;
        match self.read_result()? {
            JsonResult::Void => Ok(()),
            _ => Err(PlayerApiError::Other(anyhow!(
                "Got something other than \"void\" when calling `won`!"
            ))),
        }
    }
}
