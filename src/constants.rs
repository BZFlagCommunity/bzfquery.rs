// must match MaxPacketLen at https://github.com/BZFlag-Dev/bzflag/blob/2.4/include/Protocol.h#L47
pub const BUFFER_SIZE: usize = 1024;

// must match GameType order at https://github.com/BZFlag-Dev/bzflag/blob/2.4/include/global.h#L89-L95
pub const GAME_STYLES: [&str; 4] = ["FFA", "CTF", "OFFA", "Rabbit"];
// must match TeamColor order at https://github.com/BZFlag-Dev/bzflag/blob/2.4/include/global.h#L54-L66
pub const TEAM_NAMES: [&str; 8] = ["Rogue", "Red", "Green", "Blue", "Purple", "Observer", "Rabbit", "Hunter"];

// must match GameOptions at https://github.com/BZFlag-Dev/bzflag/blob/2.4/include/global.h#L97-L108
pub const GAME_OPTION_FLAGS: u16 = 0x0002; // superflags allowed
pub const GAME_OPTION_JUMPING: u16 = 0x0008; // jumping allowed
pub const GAME_OPTION_INERTIA: u16 = 0x0010; // momentum for all
pub const GAME_OPTION_RICOCHET: u16 = 0x0020; // all shots ricochet
pub const GAME_OPTION_SHAKING: u16 = 0x0040; // can drop bad flags
pub const GAME_OPTION_ANTIDOTE: u16 = 0x0080; // anti-bad flags
pub const GAME_OPTION_HANDICAP: u16 = 0x0100; // handicap players based on score
pub const GAME_OPTION_NO_TEAM_KILLS: u16 = 0x0400; // can not shoot team members

// must match MsgQueryGame, MsgQueryPlayers, MsgTeamUpdate, and MsgAddPlayer at https://github.com/BZFlag-Dev/bzflag/blob/2.4/include/Protocol.h
pub const MSG_QUERY_GAME: &[u8; 2] = b"qg";
pub const MSG_QUERY_PLAYERS: &[u8; 2] = b"qp";
pub const MSG_TEAM_UPDATE: &[u8; 2] = b"tu";
pub const MSG_ADD_PLAYER: &[u8; 2] = b"ap";
