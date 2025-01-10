use bevy_matchbox::matchbox_socket::PeerId;

// The first generic parameter, u8, is the input type: 4-directions + fire fits
// easily in a single byte
// The second parameter is the address type of peers: Matchbox' WebRtcSocket
// addresses are called `PeerId`s
pub type Config = bevy_ggrs::GgrsConfig<u8, PeerId>;