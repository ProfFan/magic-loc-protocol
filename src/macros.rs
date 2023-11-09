// Macros for auto generating `TryInto`, `From`, and `TryFrom` for state machines.

/// Generates the `TryInto`, `From`, and `TryFrom` (`AnyXXX`, `XXXErased`) for a state machine.
/// 
/// # Example
/// 
/// ```notrust
/// use magic_loc_protocol::macros::generate_state_machine_traits;
/// 
/// generate_state_machine_traits!(
///    /// The state machine.
///    AnchorSideStateMachine,
///   /// The type erased type
///   AnyAnchorSideStateMachine,
///   /// The internal enum that holds the type erased state machine.
///   AnchorSideStateMachineErased,
/// );
/// ```
/// The macro extracts all variants from `AnchorSideStateMachineErased`
#[macro_export]
macro_rules! generate_state_machine_traits {
    (
        $(#[$meta:meta])*
        $state_machine:ident,
        $any_state_machine:ident,
        $state_machine_erased:ident,
    ) => {}

}
