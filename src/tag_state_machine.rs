use heapless::Vec;

/// Type-state state machine for the multi-anchor AltDS-TWR protocol, tag side.
///
/// This state machine is used to implement the multi-anchor multi-tag AltDS-TWR protocol.
///
/// The protocol runs in three phases:
/// 1. All anchors send a poll message to all tags.
/// 2. All tags send a response message to all anchors.
/// 3. All anchors send a final message to all tags.
///
/// At the end of the protocol, the tags will have the distance to all anchors.
pub struct TagSideStateMachine<STATE> {
    /// Addresses
    anchors: Vec<u16, 16>,

    /// Poll TX timestamps (in anchor time)
    poll_tx_ts: Vec<u64, 16>,

    /// Response TX timestamps (in tag time)
    response_tx_ts: Vec<u64, 16>,

    /// Response RX timestamps (in anchor time)
    response_rx_ts: Vec<u64, 16>,

    /// Final TX timestamps (in anchor time)
    final_tx_ts: Vec<u64, 16>,

    /// Final RX timestamps (in tag time)
    final_rx_ts: Vec<u64, 16>,

    /// The current state of the state machine.
    _state: STATE,
}

/// The `Idle` state, where there is no ranging in progress.
struct Idle;

/// The `WaitingForPoll` state, where the tag is waiting for a poll message from an anchor.
struct WaitingForAnchorPoll;

/// The `WaitingForFinal` state, where the tag is waiting for a final message from all anchors.
struct WaitingForAnchorFinal;

/// Implement `TagSideStateMachine` for `Idle`.
impl TagSideStateMachine<Idle> {
    /// Create a new `TagSideStateMachine` in the `Idle` state.
    pub fn new(anchors: Vec<u16, 16>) -> Self {
        Self {
            anchors: anchors,
            poll_tx_ts: Vec::new(),
            response_tx_ts: Vec::new(),
            response_rx_ts: Vec::new(),
            final_tx_ts: Vec::new(),
            final_rx_ts: Vec::new(),

            _state: Idle,
        }
    }

    /// Transition to the `WaitingForAnchorPoll` state.
    pub fn waiting_for_anchor_poll(self) -> TagSideStateMachine<WaitingForAnchorPoll> {
        TagSideStateMachine {
            anchors: self.anchors,
            poll_tx_ts: self.poll_tx_ts,
            response_tx_ts: self.response_tx_ts,
            response_rx_ts: self.response_rx_ts,
            final_tx_ts: self.final_tx_ts,
            final_rx_ts: self.final_rx_ts,

            _state: WaitingForAnchorPoll,
        }
    }
}

/// Implement `TagSideStateMachine` for `WaitingForAnchorPoll`.
impl TagSideStateMachine<WaitingForAnchorPoll> {
    /// Set the TX timestamp for a poll message.
    pub fn set_poll_tx_ts(&mut self, anchor: u16, poll_tx_ts: u64) {
        self.poll_tx_ts[anchor as usize] = poll_tx_ts;
    }

    /// Transition to the `WaitingForAnchorFinal` state.
    pub fn waiting_for_anchor_final(self) -> TagSideStateMachine<WaitingForAnchorFinal> {
        TagSideStateMachine {
            anchors: self.anchors,
            poll_tx_ts: self.poll_tx_ts,
            response_tx_ts: self.response_tx_ts,
            response_rx_ts: self.response_rx_ts,
            final_tx_ts: self.final_tx_ts,
            final_rx_ts: self.final_rx_ts,

            _state: WaitingForAnchorFinal,
        }
    }
}

/// Implement `TagSideStateMachine` for `WaitingForAnchorFinal`.
impl TagSideStateMachine<WaitingForAnchorFinal> {
    /// Set the TX timestamp for a response message.
    pub fn set_response_tx_ts(&mut self, anchor: u16, response_tx_ts: u64) {
        self.response_tx_ts[anchor as usize] = response_tx_ts;
    }

    /// Set the RX timestamp for a response message.
    pub fn set_response_rx_ts(&mut self, anchor: u16, response_rx_ts: u64) {
        self.response_rx_ts[anchor as usize] = response_rx_ts;
    }

    /// Set the TX timestamp for a final message. (parsed from the final message)
    pub fn set_final_tx_ts(&mut self, anchor: u16, final_tx_ts: u64) {
        self.final_tx_ts[anchor as usize] = final_tx_ts;
    }

    /// Set the RX timestamp for a final message. (retrieved from the RX timestamp register)
    pub fn set_final_rx_ts(&mut self, anchor: u16, final_rx_ts: u64) {
        self.final_rx_ts[anchor as usize] = final_rx_ts;
    }

    /// Transition to the `Idle` state.
    ///
    /// This is the end of the protocol.
    pub fn idle(self) -> TagSideStateMachine<Idle> {
        TagSideStateMachine {
            anchors: self.anchors,
            poll_tx_ts: self.poll_tx_ts,
            response_tx_ts: self.response_tx_ts,
            response_rx_ts: self.response_rx_ts,
            final_tx_ts: self.final_tx_ts,
            final_rx_ts: self.final_rx_ts,

            _state: Idle,
        }
    }
}
