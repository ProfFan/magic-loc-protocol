use heapless::Vec;

/// Type-state state machine for the multi-anchor AltDS-TWR protocol, anchor side.
///
/// This state machine is used to implement the multi-anchor multi-tag AltDS-TWR protocol.
///
/// The protocol runs in three phases:
/// 1. All anchors send a poll message to all tags.
/// 2. All tags send a response message to all anchors.
/// 3. All anchors send a final message to all tags.
///
/// At the end of the protocol, the tags will have the distance to all anchors.
pub struct AnchorSideStateMachine<STATE> {
    /// Addresses (tags)
    tags: Vec<u16, 16>,

    /// The current TX timestamp for the poll message.
    ///
    /// Should not be accessible when state is `Idle`.
    poll_tx_ts: u64,

    /// The current RX timestamps for the response messages.
    ///
    /// Can only be set when state is `WaitingForResponse`, and read when state is `SendingFinal`.
    response_rx_ts: Vec<u64, 16>,

    /// The current state of the state machine.
    _state: STATE,
}

/// The `Idle` state, where there is no ranging in progress.
pub struct Idle;

/// The `WaitingForResponse` state, where the anchor is waiting for response messages from all tags.
pub struct WaitingForResponse;

/// The `SendingFinal` state, where the anchor is sending final messages to all tags.
pub struct SendingFinal;

/// Implement `AnchorSideStateMachine` for `Idle`.
impl AnchorSideStateMachine<Idle> {
    /// Create a new `AnchorSideStateMachine` in the `Idle` state.
    pub fn new(tags: Vec<u16, 16>) -> Self {
        Self {
            tags: tags,
            poll_tx_ts: 0,
            response_rx_ts: Vec::new(),
            _state: Idle,
        }
    }

    /// Transition to the `WaitingForResponse` state.
    pub fn waiting_for_response(
        self,
        poll_tx_ts: u64,
    ) -> AnchorSideStateMachine<WaitingForResponse> {
        AnchorSideStateMachine {
            tags: self.tags,
            poll_tx_ts,
            response_rx_ts: self.response_rx_ts,
            _state: WaitingForResponse,
        }
    }
}

/// Implement `AnchorSideStateMachine` for `WaitingForResponse`.
impl AnchorSideStateMachine<WaitingForResponse> {
    /// Set the RX timestamp for a response message.
    pub fn set_response_rx_ts(&mut self, tag: u8, response_rx_ts: u64) {
        self.response_rx_ts[tag as usize] = response_rx_ts;
    }

    /// Transition to the `SendingFinal` state.
    pub fn sending_final(self) -> AnchorSideStateMachine<SendingFinal> {
        AnchorSideStateMachine {
            tags: self.tags,
            poll_tx_ts: self.poll_tx_ts,
            response_rx_ts: self.response_rx_ts,
            _state: SendingFinal,
        }
    }
}

/// Implement `AnchorSideStateMachine` for `SendingFinal`.
///
/// In this state we just wait for the final message to be sent, and then transition back to `Idle`.
impl AnchorSideStateMachine<SendingFinal> {
    /// Transition to the `Idle` state.
    pub fn idle(self) -> AnchorSideStateMachine<Idle> {
        AnchorSideStateMachine {
            tags: self.tags,
            poll_tx_ts: 0,
            response_rx_ts: Vec::new(),
            _state: Idle,
        }
    }
}
