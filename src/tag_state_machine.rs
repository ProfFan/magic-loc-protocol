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
#[derive(Debug, Default)]
pub struct TagSideStateMachine<STATE> {
    /// My address
    address: u16,

    /// Addresses
    anchors: Vec<u16, 16>,

    /// Tag Addresses
    tags: Vec<u16, 16>,

    /// Poll TX timestamps (in anchor time)
    poll_tx_ts: Vec<u64, 16>,

    /// Poll RX timestamps (in tag time)
    poll_rx_ts: Vec<u64, 16>,

    /// Response TX timestamp (in tag time)
    response_tx_ts: u64,

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
#[derive(Debug, Default)]
pub struct Idle;

/// The `WaitingForPoll` state, where the tag is waiting for a poll message from an anchor.
#[derive(Debug, Default)]
pub struct WaitingForAnchorPoll;

/// The `WaitingForFinal` state, where the tag is waiting for a final message from all anchors.
#[derive(Debug, Default)]
pub struct WaitingForAnchorFinal;

/// Implement `TagSideStateMachine` for `Idle`.
impl TagSideStateMachine<Idle> {
    /// Create a new `TagSideStateMachine` in the `Idle` state.
    pub fn new(address: u16, anchors: Vec<u16, 16>, tags: Vec<u16, 16>) -> Self {
        Self {
            address: address,
            poll_tx_ts: Vec::from_iter(core::iter::repeat(0).take(anchors.len())),
            poll_rx_ts: Vec::from_iter(core::iter::repeat(0).take(anchors.len())),
            response_rx_ts: Vec::from_iter(core::iter::repeat(0).take(anchors.len())),
            final_tx_ts: Vec::from_iter(core::iter::repeat(0).take(anchors.len())),
            final_rx_ts: Vec::from_iter(core::iter::repeat(0).take(anchors.len())),
            response_tx_ts: 0,
            anchors: anchors,
            tags: tags,

            _state: Idle,
        }
    }

    /// Transition to the `WaitingForAnchorPoll` state.
    pub fn waiting_for_anchor_poll(self) -> TagSideStateMachine<WaitingForAnchorPoll> {
        TagSideStateMachine {
            address: self.address,
            anchors: self.anchors,
            tags: self.tags,
            poll_tx_ts: self.poll_tx_ts,
            poll_rx_ts: self.poll_rx_ts,
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
    pub fn set_poll_tx_ts_idx(&mut self, anchor_idx: usize, poll_tx_ts: u64) {
        self.poll_tx_ts[anchor_idx] = poll_tx_ts;
    }

    /// Set the TX timestamp for a poll message.
    /// 
    /// Will panic if the anchor address is not found.
    pub fn set_poll_tx_ts(&mut self, anchor_addr: u16, poll_tx_ts: u64) {
        let anchor_idx = self.anchors.iter().position(|&addr| addr == anchor_addr).unwrap();
        self.poll_tx_ts[anchor_idx] = poll_tx_ts;
    }

    /// Set the RX timestamp for a poll message.
    pub fn set_poll_rx_ts_idx(&mut self, anchor_idx: usize, poll_rx_ts: u64) {
        self.poll_rx_ts[anchor_idx] = poll_rx_ts;
    }

    /// Set the RX timestamp for a poll message.
    /// 
    /// Will panic if the anchor address is not found.
    pub fn set_poll_rx_ts(&mut self, anchor_addr: u16, poll_rx_ts: u64) {
        let anchor_idx = self.anchors.iter().position(|&addr| addr == anchor_addr).unwrap();
        self.poll_rx_ts[anchor_idx] = poll_rx_ts;
    }

    /// Transition to the `WaitingForAnchorFinal` state.
    pub fn waiting_for_anchor_final(self) -> TagSideStateMachine<WaitingForAnchorFinal> {
        TagSideStateMachine {
            address: self.address,
            anchors: self.anchors,
            tags: self.tags,
            poll_tx_ts: self.poll_tx_ts,
            poll_rx_ts: self.poll_rx_ts,
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
    pub fn set_response_tx_ts(&mut self, response_tx_ts: u64) {
        self.response_tx_ts = response_tx_ts;
    }

    /// Set the RX timestamp for a response message.
    pub fn set_response_rx_ts_idx(&mut self, anchor_idx: usize, response_rx_ts: u64) {
        self.response_rx_ts[anchor_idx] = response_rx_ts;
    }
    
    /// Set the RX timestamp for a response message.
    /// 
    /// Will panic if the anchor address is not found.
    pub fn set_response_rx_ts(&mut self, anchor_addr: u16, response_rx_ts: u64) {
        let anchor_idx = self.anchors.iter().position(|&addr| addr == anchor_addr).unwrap();
        self.response_rx_ts[anchor_idx] = response_rx_ts;
    }

    /// Set the TX timestamp for a final message. (parsed from the final message)
    pub fn set_final_tx_ts_idx(&mut self, anchor_idx: usize, final_tx_ts: u64) {
        self.final_tx_ts[anchor_idx] = final_tx_ts;
    }

    /// Set the TX timestamp for a final message. (parsed from the final message)
    /// 
    /// Will panic if the anchor address is not found.
    pub fn set_final_tx_ts(&mut self, anchor_addr: u16, final_tx_ts: u64) {
        let anchor_idx = self.anchors.iter().position(|&addr| addr == anchor_addr).unwrap();
        self.final_tx_ts[anchor_idx] = final_tx_ts;
    }

    /// Set the RX timestamp for a final message. (retrieved from the RX timestamp register)
    pub fn set_final_rx_ts_idx(&mut self, anchor_idx: usize, final_rx_ts: u64) {
        self.final_rx_ts[anchor_idx] = final_rx_ts;
    }

    /// Set the RX timestamp for a final message. (retrieved from the RX timestamp register)
    /// 
    /// Will panic if the anchor address is not found.
    pub fn set_final_rx_ts(&mut self, anchor_addr: u16, final_rx_ts: u64) {
        let anchor_idx = self.anchors.iter().position(|&addr| addr == anchor_addr).unwrap();
        self.final_rx_ts[anchor_idx] = final_rx_ts;
    }

    /// Transition to the `Idle` state.
    ///
    /// This is the end of the protocol.
    pub fn idle(self) -> TagSideStateMachine<Idle> {
        TagSideStateMachine {
            address: self.address,
            anchors: self.anchors,
            tags: self.tags,
            poll_tx_ts: self.poll_tx_ts,
            poll_rx_ts: self.poll_rx_ts,
            response_tx_ts: self.response_tx_ts,
            response_rx_ts: self.response_rx_ts,
            final_tx_ts: self.final_tx_ts,
            final_rx_ts: self.final_rx_ts,

            _state: Idle,
        }
    }
}

// Type erasure for `TagSideStateMachine`.

/// Type erasure for `TagSideStateMachine`.
#[derive(Debug)]
pub enum AnyTagSideStateMachineErased {
    /// The `Idle` state.
    Idle(TagSideStateMachine<Idle>),

    /// The `WaitingForAnchorPoll` state.
    WaitingForAnchorPoll(TagSideStateMachine<WaitingForAnchorPoll>),

    /// The `WaitingForAnchorFinal` state.
    WaitingForAnchorFinal(TagSideStateMachine<WaitingForAnchorFinal>),
}

/// Type erasure for `TagSideStateMachine`.
#[derive(Debug)]
pub struct AnyTagSideStateMachine {
    /// The type-erased state machine.
    state_machine: AnyTagSideStateMachineErased,
}

/// Implement mutation methods for `AnyTagSideStateMachine`.
impl AnyTagSideStateMachine {
    /// Extract the underlying state machine type.
    pub fn as_idle_mut(&mut self) -> Option<&mut TagSideStateMachine<Idle>> {
        match &mut self.state_machine {
            AnyTagSideStateMachineErased::Idle(state_machine) => Some(state_machine),
            _ => None,
        }
    }

    /// Extract the underlying state machine type.
    pub fn as_waiting_for_anchor_poll_mut(
        &mut self,
    ) -> Option<&mut TagSideStateMachine<WaitingForAnchorPoll>> {
        match &mut self.state_machine {
            AnyTagSideStateMachineErased::WaitingForAnchorPoll(state_machine) => Some(state_machine),
            _ => None,
        }
    }

    /// Extract the underlying state machine type.
    pub fn as_waiting_for_anchor_final_mut(
        &mut self,
    ) -> Option<&mut TagSideStateMachine<WaitingForAnchorFinal>> {
        match &mut self.state_machine {
            AnyTagSideStateMachineErased::WaitingForAnchorFinal(state_machine) => {
                Some(state_machine)
            }
            _ => None,
        }
    }

    /// Transition to the `WaitingForAnchorPoll` state.
    pub fn to_waiting_for_anchor_poll(&mut self) -> Result<(), ()> {
        match self.state_machine {
            AnyTagSideStateMachineErased::Idle(ref mut state_machine) => {
                let state_machine = core::mem::take(state_machine);
                self.state_machine = AnyTagSideStateMachineErased::WaitingForAnchorPoll(
                    state_machine.waiting_for_anchor_poll(),
                );
                Ok(())
            }
            _ => Err(()),
        }
    }

    /// Transition to the `WaitingForAnchorFinal` state.
    pub fn to_waiting_for_anchor_final(&mut self) -> Result<(), ()> {
        match self.state_machine {
            AnyTagSideStateMachineErased::WaitingForAnchorPoll(ref mut state_machine) => {
                let state_machine = core::mem::take(state_machine);
                self.state_machine = AnyTagSideStateMachineErased::WaitingForAnchorFinal(
                    state_machine.waiting_for_anchor_final(),
                );
                Ok(())
            }
            _ => Err(()),
        }
    }
}

// Implement `From` for `TagSideStateMachine` and `AnyTagSideStateMachine`.

impl From<TagSideStateMachine<Idle>> for AnyTagSideStateMachine {
    fn from(state_machine: TagSideStateMachine<Idle>) -> Self {
        Self {
            state_machine: AnyTagSideStateMachineErased::Idle(state_machine),
        }
    }
}

impl From<TagSideStateMachine<WaitingForAnchorPoll>> for AnyTagSideStateMachine {
    fn from(state_machine: TagSideStateMachine<WaitingForAnchorPoll>) -> Self {
        Self {
            state_machine: AnyTagSideStateMachineErased::WaitingForAnchorPoll(state_machine),
        }
    }
}

impl From<TagSideStateMachine<WaitingForAnchorFinal>> for AnyTagSideStateMachine {
    fn from(state_machine: TagSideStateMachine<WaitingForAnchorFinal>) -> Self {
        Self {
            state_machine: AnyTagSideStateMachineErased::WaitingForAnchorFinal(state_machine),
        }
    }
}

// Implement `TryInto` for `TagSideStateMachine` and `AnyTagSideStateMachine`.

impl TryInto<TagSideStateMachine<Idle>> for AnyTagSideStateMachine {
    type Error = ();

    fn try_into(self) -> Result<TagSideStateMachine<Idle>, Self::Error> {
        match self.state_machine {
            AnyTagSideStateMachineErased::Idle(state_machine) => Ok(state_machine),
            _ => Err(()),
        }
    }
}

impl TryInto<TagSideStateMachine<WaitingForAnchorPoll>> for AnyTagSideStateMachine {
    type Error = ();

    fn try_into(self) -> Result<TagSideStateMachine<WaitingForAnchorPoll>, Self::Error> {
        match self.state_machine {
            AnyTagSideStateMachineErased::WaitingForAnchorPoll(state_machine) => Ok(state_machine),
            _ => Err(()),
        }
    }
}

impl TryInto<TagSideStateMachine<WaitingForAnchorFinal>> for AnyTagSideStateMachine {
    type Error = ();

    fn try_into(self) -> Result<TagSideStateMachine<WaitingForAnchorFinal>, Self::Error> {
        match self.state_machine {
            AnyTagSideStateMachineErased::WaitingForAnchorFinal(state_machine) => Ok(state_machine),
            _ => Err(()),
        }
    }
}

// Implement `TryFrom` for references

impl<'a> TryFrom<&'a AnyTagSideStateMachine> for &'a TagSideStateMachine<Idle> {
    type Error = ();

    fn try_from(state_machine: &'a AnyTagSideStateMachine) -> Result<Self, Self::Error> {
        match &state_machine.state_machine {
            AnyTagSideStateMachineErased::Idle(state_machine) => Ok(state_machine),
            _ => Err(()),
        }
    }
}

impl<'a> TryFrom<&'a AnyTagSideStateMachine> for &'a TagSideStateMachine<WaitingForAnchorPoll> {
    type Error = ();

    fn try_from(state_machine: &'a AnyTagSideStateMachine) -> Result<Self, Self::Error> {
        match &state_machine.state_machine {
            AnyTagSideStateMachineErased::WaitingForAnchorPoll(state_machine) => Ok(state_machine),
            _ => Err(()),
        }
    }
}

impl<'a> TryFrom<&'a AnyTagSideStateMachine> for &'a TagSideStateMachine<WaitingForAnchorFinal> {
    type Error = ();

    fn try_from(state_machine: &'a AnyTagSideStateMachine) -> Result<Self, Self::Error> {
        match &state_machine.state_machine {
            AnyTagSideStateMachineErased::WaitingForAnchorFinal(state_machine) => Ok(state_machine),
            _ => Err(()),
        }
    }
}

// Tests

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tag_state_machine() {
        let anchors: [u16; 8] = [0, 1, 2, 3, 4, 5, 6, 7];
        let tags = [100u16, 101, 102];
        let state_machine =
            TagSideStateMachine::<Idle>::new(0, Vec::from_iter(anchors), Vec::from_iter(tags));

        let mut state_machine = state_machine.waiting_for_anchor_poll();

        state_machine.set_poll_tx_ts(0, 0x1234_5678_9abc_def0);

        let mut state_machine = state_machine.waiting_for_anchor_final();

        state_machine.set_response_tx_ts(0x1234_5678_9abc_def1);
        state_machine.set_response_rx_ts(0, 0x1234_5678_9abc_def2);
        state_machine.set_final_tx_ts(0, 0x1234_5678_9abc_def3);
        state_machine.set_final_rx_ts(0, 0x1234_5678_9abc_def4);

        let state_machine = state_machine.idle();

        assert_eq!(state_machine.poll_tx_ts.len(), 8);
    }
}
