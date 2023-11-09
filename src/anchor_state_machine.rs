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
#[derive(Clone, Debug, Default)]
pub struct AnchorSideStateMachine<STATE> {
    /// Anchor address
    address: u16,

    /// Anchor addresses in the network
    anchor_addresses: Vec<u16, 16>,

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
#[derive(Debug, Clone, Default)]
pub struct Idle;

/// The `WaitingForResponse` state, where the anchor is waiting for response messages from all tags.
#[derive(Debug, Clone, Default)]
pub struct WaitingForResponse;

/// The `SendingFinal` state, where the anchor is sending final messages to all tags.
#[derive(Debug, Clone, Default)]
pub struct SendingFinal;

/// Implement `AnchorSideStateMachine` for `Idle`.
impl AnchorSideStateMachine<Idle> {
    /// Create a new `AnchorSideStateMachine` in the `Idle` state.
    pub fn new(address: u16, anchors: Vec<u16, 16>, tags: Vec<u16, 16>) -> Self {
        Self {
            address: address,
            anchor_addresses: anchors,
            response_rx_ts: Vec::from_iter((0..tags.len()).map(|_| 0)),
            tags: tags,
            poll_tx_ts: 0,
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
            address: self.address,
            anchor_addresses: self.anchor_addresses,
        }
    }
}

/// Implement `AnchorSideStateMachine` for `WaitingForResponse`.
impl AnchorSideStateMachine<WaitingForResponse> {
    /// Set the RX timestamp for a response message.
    pub fn set_response_rx_ts(&mut self, tag_idx: usize, response_rx_ts: u64) {
        self.response_rx_ts[tag_idx] = response_rx_ts;
    }

    /// Transition to the `SendingFinal` state.
    pub fn sending_final(self) -> AnchorSideStateMachine<SendingFinal> {
        AnchorSideStateMachine {
            tags: self.tags,
            poll_tx_ts: self.poll_tx_ts,
            response_rx_ts: self.response_rx_ts,
            _state: SendingFinal,
            address: self.address,
            anchor_addresses: self.anchor_addresses,
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
            response_rx_ts: self.response_rx_ts,
            _state: Idle,
            address: self.address,
            anchor_addresses: self.anchor_addresses,
        }
    }

    /// Get the RX timestamp for a response message.
    pub fn get_response_rx_ts(&self, tag_idx: usize) -> u64 {
        self.response_rx_ts[tag_idx]
    }
}

/// Type erased state machine for the multi-anchor AltDS-TWR protocol, anchor side.
#[derive(Debug)]
pub enum AnchorSideStateMachineTypeErased {
    Idle(AnchorSideStateMachine<Idle>),
    WaitingForResponse(AnchorSideStateMachine<WaitingForResponse>),
    SendingFinal(AnchorSideStateMachine<SendingFinal>),
}

#[derive(Debug)]
pub struct AnyAnchorSideStateMachine {
    state_machine: AnchorSideStateMachineTypeErased,
}

impl AnyAnchorSideStateMachine {
    /// Get a mutable reference to the state machine in the `Idle` state.
    pub fn as_idle_mut(&mut self) -> Option<&mut AnchorSideStateMachine<Idle>> {
        match &mut self.state_machine {
            AnchorSideStateMachineTypeErased::Idle(state_machine) => Some(state_machine),
            _ => None,
        }
    }

    /// Get a mutable reference to the state machine in the `WaitingForResponse` state.
    pub fn as_waiting_for_response_mut(
        &mut self,
    ) -> Option<&mut AnchorSideStateMachine<WaitingForResponse>> {
        match &mut self.state_machine {
            AnchorSideStateMachineTypeErased::WaitingForResponse(state_machine) => {
                Some(state_machine)
            }
            _ => None,
        }
    }

    /// Get a mutable reference to the state machine in the `SendingFinal` state.
    pub fn as_sending_final_mut(&mut self) -> Option<&mut AnchorSideStateMachine<SendingFinal>> {
        match &mut self.state_machine {
            AnchorSideStateMachineTypeErased::SendingFinal(state_machine) => Some(state_machine),
            _ => None,
        }
    }

    /// Transition to the `WaitingForResponse` state, from the `Idle` state.
    ///
    /// Error if the state machine is not in the `Idle` state.
    pub fn waiting_for_response(mut self, poll_tx_ts: u64) -> Result<Self, ()> {
        match self.state_machine {
            AnchorSideStateMachineTypeErased::Idle(state_machine) => {
                self.state_machine = AnchorSideStateMachineTypeErased::WaitingForResponse(
                    state_machine.waiting_for_response(poll_tx_ts),
                );
                Ok(self)
            }
            _ => Err(()),
        }
    }

    /// Transition to the `WaitingForResponse` state, from the `Idle` state.
    /// Mutates the state machine in place.
    ///
    /// Error if the state machine is not in the `Idle` state.
    pub fn to_waiting_for_response(&mut self, poll_tx_ts: u64) -> Result<(), ()> {
        match &mut self.state_machine {
            AnchorSideStateMachineTypeErased::Idle(state_machine) => {
                let state_machine_taken = core::mem::take(state_machine);

                self.state_machine = AnchorSideStateMachineTypeErased::WaitingForResponse(
                    state_machine_taken.waiting_for_response(poll_tx_ts),
                );
                Ok(())
            }
            _ => Err(()),
        }
    }

    /// Transition to the `SendingFinal` state, from the `WaitingForResponse` state.
    /// Mutates the state machine in place.
    ///
    /// Error if the state machine is not in the `WaitingForResponse` state.
    pub fn to_sending_final(&mut self) -> Result<(), ()> {
        match &mut self.state_machine {
            AnchorSideStateMachineTypeErased::WaitingForResponse(state_machine) => {
                let state_machine_taken = core::mem::take(state_machine);

                self.state_machine = AnchorSideStateMachineTypeErased::SendingFinal(
                    state_machine_taken.sending_final(),
                );
                Ok(())
            }
            _ => Err(()),
        }
    }

    /// Transition to the `Idle` state, from the `SendingFinal` state.
    ///
    /// Error if the state machine is not in the `SendingFinal` state.
    pub fn to_idle(&mut self) -> Result<(), ()> {
        match &mut self.state_machine {
            AnchorSideStateMachineTypeErased::SendingFinal(state_machine) => {
                let state_machine_taken = core::mem::take(state_machine);

                self.state_machine =
                    AnchorSideStateMachineTypeErased::Idle(state_machine_taken.idle());
                Ok(())
            }
            _ => Err(()),
        }
    }
}

impl TryInto<AnchorSideStateMachine<Idle>> for AnyAnchorSideStateMachine {
    type Error = ();

    fn try_into(self) -> Result<AnchorSideStateMachine<Idle>, Self::Error> {
        match self.state_machine {
            AnchorSideStateMachineTypeErased::Idle(state_machine) => Ok(state_machine),
            _ => Err(()),
        }
    }
}

impl TryInto<AnchorSideStateMachine<WaitingForResponse>> for AnyAnchorSideStateMachine {
    type Error = ();

    fn try_into(self) -> Result<AnchorSideStateMachine<WaitingForResponse>, Self::Error> {
        match self.state_machine {
            AnchorSideStateMachineTypeErased::WaitingForResponse(state_machine) => {
                Ok(state_machine)
            }
            _ => Err(()),
        }
    }
}

impl TryInto<AnchorSideStateMachine<SendingFinal>> for AnyAnchorSideStateMachine {
    type Error = ();

    fn try_into(self) -> Result<AnchorSideStateMachine<SendingFinal>, Self::Error> {
        match self.state_machine {
            AnchorSideStateMachineTypeErased::SendingFinal(state_machine) => Ok(state_machine),
            _ => Err(()),
        }
    }
}

// From traits

impl From<AnchorSideStateMachine<Idle>> for AnyAnchorSideStateMachine {
    fn from(state_machine: AnchorSideStateMachine<Idle>) -> Self {
        Self {
            state_machine: AnchorSideStateMachineTypeErased::Idle(state_machine),
        }
    }
}

impl From<AnchorSideStateMachine<WaitingForResponse>> for AnyAnchorSideStateMachine {
    fn from(state_machine: AnchorSideStateMachine<WaitingForResponse>) -> Self {
        Self {
            state_machine: AnchorSideStateMachineTypeErased::WaitingForResponse(state_machine),
        }
    }
}

impl From<AnchorSideStateMachine<SendingFinal>> for AnyAnchorSideStateMachine {
    fn from(state_machine: AnchorSideStateMachine<SendingFinal>) -> Self {
        Self {
            state_machine: AnchorSideStateMachineTypeErased::SendingFinal(state_machine),
        }
    }
}

// Impl `TryFrom` to reference types

impl<'a> TryFrom<&'a AnyAnchorSideStateMachine> for &'a AnchorSideStateMachine<Idle> {
    type Error = ();

    fn try_from(state_machine: &'a AnyAnchorSideStateMachine) -> Result<Self, Self::Error> {
        match &state_machine.state_machine {
            AnchorSideStateMachineTypeErased::Idle(state_machine) => Ok(state_machine),
            _ => Err(()),
        }
    }
}

impl<'a> TryFrom<&'a AnyAnchorSideStateMachine> for &'a AnchorSideStateMachine<WaitingForResponse> {
    type Error = ();

    fn try_from(state_machine: &'a AnyAnchorSideStateMachine) -> Result<Self, Self::Error> {
        match &state_machine.state_machine {
            AnchorSideStateMachineTypeErased::WaitingForResponse(state_machine) => {
                Ok(state_machine)
            }
            _ => Err(()),
        }
    }
}

impl<'a> TryFrom<&'a AnyAnchorSideStateMachine> for &'a AnchorSideStateMachine<SendingFinal> {
    type Error = ();

    fn try_from(state_machine: &'a AnyAnchorSideStateMachine) -> Result<Self, Self::Error> {
        match &state_machine.state_machine {
            AnchorSideStateMachineTypeErased::SendingFinal(state_machine) => Ok(state_machine),
            _ => Err(()),
        }
    }
}

// Tests

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_idle() {
        let state_machine = AnchorSideStateMachine::new(1, Vec::new(), Vec::new());
        let state_machine = state_machine.waiting_for_response(0);
        let state_machine = state_machine.sending_final();
        let state_machine = state_machine.idle();

        assert_eq!(state_machine.address, 1);
    }

    #[test]
    fn test_any() {
        let mut state_machines: [AnyAnchorSideStateMachine; 8] = array_init::array_init(|_| {
            AnyAnchorSideStateMachine::from(AnchorSideStateMachine::new(0, Vec::new(), Vec::new()))
        });

        // Test failed conversion
        let state_machine_fail: Result<&AnchorSideStateMachine<WaitingForResponse>, _> =
            (&state_machines[0]).try_into();
        assert!(state_machine_fail.is_err());

        // Test if we can get a reference to the state machine
        let state_machine: &AnchorSideStateMachine<Idle> = (&state_machines[0]).try_into().unwrap();
        state_machines[0] = state_machine.clone().waiting_for_response(0).into();

        // Now the state machine should be in the `WaitingForResponse` state
        let state_machine: &AnchorSideStateMachine<WaitingForResponse> =
            (&state_machines[0]).try_into().unwrap();
        state_machines[0] = state_machine.clone().sending_final().into();
    }

    #[test]
    fn test_any_mutate() {
        let mut any_sm =
            AnyAnchorSideStateMachine::from(AnchorSideStateMachine::new(0, Vec::new(), Vec::new()));

        let result = any_sm.waiting_for_response(1);

        assert!(result.is_ok());

        any_sm = result.unwrap();

        // Check that the state machine is now in the `WaitingForResponse` state
        let state_machine: &AnchorSideStateMachine<WaitingForResponse> =
            &any_sm.try_into().unwrap();

        assert_eq!(state_machine.poll_tx_ts, 1);
    }
}
