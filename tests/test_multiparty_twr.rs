// Test that emulates a real-world scenario of a multiparty TWR

use std::println;

use heapless::Vec;
use magic_loc_protocol::anchor_state_machine::AnchorSideStateMachine;
use magic_loc_protocol::anchor_state_machine::{AnyAnchorSideStateMachine, Idle};
use magic_loc_protocol::tag_state_machine;

#[test]
fn scenario_8anchor_3tag() {
    // Assume synchronization has already been done
    let anchor_addresses: [u16; 8] = [0, 1, 2, 3, 4, 5, 6, 7];
    let tag_addresses: [u16; 3] = [100, 101, 102];
    let mut anchor_state_machines: [AnyAnchorSideStateMachine; 8] = array_init::array_init(|i| {
        AnchorSideStateMachine::<Idle>::new(
            i as u16,
            Vec::from_slice(&anchor_addresses).unwrap(),
            Vec::from_slice(&tag_addresses).unwrap(),
        )
        .into()
    });

    let mut tag_state_machines: [tag_state_machine::AnyTagSideStateMachine; 3] =
        array_init::array_init(|i| {
            tag_state_machine::TagSideStateMachine::new(
                i as u16,
                Vec::from_slice(&anchor_addresses).unwrap(),
                Vec::from_slice(&tag_addresses).unwrap(),
            )
            .into()
        });

    // All tags start in the waiting for poll state
    for tag_state_machine in tag_state_machines.iter_mut() {
        tag_state_machine.to_waiting_for_anchor_poll().unwrap();
    }

    for (i, anchor_state_machine) in anchor_state_machines.iter_mut().enumerate() {
        let txts = i as u64;

        anchor_state_machine.to_waiting_for_response(txts).unwrap();

        // All tags receive the poll
        for (j, tag_state_machine) in tag_state_machines.iter_mut().enumerate() {
            let rxts = txts + j as u64;
            let tsm = tag_state_machine.as_waiting_for_anchor_poll_mut().unwrap();
            tsm.set_poll_tx_ts(i, txts);
            tsm.set_poll_rx_ts(i, rxts);
        }
    }

    // All tags send a response
    for (i, tag_state_machine) in tag_state_machines.iter_mut().enumerate() {
        let txts = (i as u64) + 1000;

        tag_state_machine.to_waiting_for_anchor_final().unwrap();

        let tsm = tag_state_machine.as_waiting_for_anchor_final_mut().unwrap();
        tsm.set_response_tx_ts(txts);

        // All anchors receive the response
        for (j, anchor_state_machine) in anchor_state_machines.iter_mut().enumerate() {
            let rxts = txts + j as u64;
            let asm = anchor_state_machine.as_waiting_for_response_mut().unwrap();
            asm.set_response_rx_ts(i, rxts);
        }
    }

    // All anchors send the final
    for (i, anchor_state_machine) in anchor_state_machines.iter_mut().enumerate() {
        let txts = (i as u64) + 2000;
        anchor_state_machine.to_sending_final().unwrap();

        // All tags receive the final
        for (j, tag_state_machine) in tag_state_machines.iter_mut().enumerate() {
            let rxts = txts + j as u64;
            let tsm = tag_state_machine.as_waiting_for_anchor_final_mut().unwrap();
            tsm.set_response_rx_ts(
                i,
                anchor_state_machine
                    .as_sending_final_mut()
                    .unwrap()
                    .get_response_rx_ts(j),
            );
            tsm.set_final_tx_ts(i, txts);
            tsm.set_final_rx_ts(i, rxts);
        }

        anchor_state_machine.to_idle().unwrap();
    }

    println!("Tag SM status: {:#?}", tag_state_machines);
}
