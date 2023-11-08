// Opportunistic synchronization of time within the network
//
// Since our protocol need TDMA, we need to synchronize the time within the network.
//
// In this setting we assume that all anchors can hear each other, and anchor 0 is the root of the
// network. The root will periodically send a beacon message, and all other anchors will synchronize
// their time to the root.
//
// After all anchors have synchronized their time to the root, the tags just need to calculate their time slot
// based on their address.
