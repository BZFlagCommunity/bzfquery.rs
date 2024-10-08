use std::fmt;
use std::io::{Read, Write};
use std::net::TcpStream;
use std::str::from_utf8;

mod constants;
pub use constants::*;

// must match GameType order at https://github.com/BZFlag-Dev/bzflag/blob/c136308903a77ab8e7ed5bfebec5c13fecee0711/include/global.h#L89-L95
#[derive(Debug, Clone, Copy)]
pub enum GameType {
  FFA,
  CTF,
  OFFA,
  Rabbit,
}

impl fmt::Display for GameType {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    write!(f, "{}", GAME_STYLES[*self as usize])
  }
}

impl From<u16> for GameType {
  fn from(style: u16) -> Self {
    match style {
      0 => GameType::FFA,
      1 => GameType::CTF,
      2 => GameType::OFFA,
      3 => GameType::Rabbit,
      _ => panic!("invalid game style: {}", style),
    }
  }
}

// must match TeamColor order at https://github.com/BZFlag-Dev/bzflag/blob/c136308903a77ab8e7ed5bfebec5c13fecee0711/include/global.h#L54-L66
#[derive(Debug, Clone, Copy)]
pub enum TeamColor {
  Rogue,
  Red,
  Green,
  Blue,
  Purple,
  Observer,
  Rabbit,
  Hunter,
}

impl fmt::Display for TeamColor {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    write!(f, "{}", TEAM_NAMES[*self as usize])
  }
}

impl From<u16> for TeamColor {
  fn from(team: u16) -> Self {
    match team {
      0 => TeamColor::Rogue,
      1 => TeamColor::Red,
      2 => TeamColor::Green,
      3 => TeamColor::Blue,
      4 => TeamColor::Purple,
      5 => TeamColor::Observer,
      6 => TeamColor::Rabbit,
      7 => TeamColor::Hunter,
      _ => panic!("invalid team color: {}", team),
    }
  }
}

pub struct Query {
  pub style: GameType,
  pub options: Options,
  pub max_players: u16,
  pub max_shots: u16,
  pub shake_wins: u16,
  pub shake_timeout: u16,
  pub max_player_score: u16,
  pub max_team_score: u16,
  pub max_time: u16,
  pub elapsed_time: u16,
  pub teams: Vec<Team>,
  pub players: Vec<Player>,
}

pub struct Options {
  pub flags: bool,
  pub jumping: bool,
  pub inertia: bool,
  pub ricochet: bool,
  pub shaking: bool,
  pub antidote: bool,
  pub handicap: bool,
  pub no_team_kills: bool,
}

pub struct Team {
  pub team: TeamColor,
  pub size: u16,
  pub max_size: u16,
  pub wins: u16,
  pub losses: u16,
}

pub struct Player {
  pub id: u8,
  pub player_type: u16,
  pub team: TeamColor,
  pub wins: u16,
  pub losses: u16,
  pub tks: u16,
  pub callsign: String,
  pub motto: String,
}

impl fmt::Display for Query {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    let mut teams = String::new();
    let mut players = String::new();

    for team in self.teams.iter() {
      teams += format!("\n • {}", team).as_str();
    }

    for player in self.players.iter() {
      players += format!("\n • {}", player).as_str();
    }

    write!(
      f,
      "style: {}\noptions:\n  flags: {}\n  jumping: {}\n  inertia: {}\n  ricochet: {}\n  shaking: {}\n  antidote: {}\n  handicap: {}\n  no team kills: {}\nmax_players: {}\nmax_shots: {}\nshake_wins: {}\nshake_timeout: {}\nmax_player_score: {}\nmax_team_score: {}\nmax_time: {}\nelapsed_time: {}\nteams:{}\nplayers:{}",
      self.style,
      self.options.flags,
      self.options.jumping,
      self.options.inertia,
      self.options.ricochet,
      self.options.shaking,
      self.options.antidote,
      self.options.handicap,
      self.options.no_team_kills,
      self.max_players,
      self.max_shots,
      self.shake_wins,
      self.shake_timeout,
      self.max_player_score,
      self.max_team_score,
      self.max_time,
      self.elapsed_time,
      teams,
      players
    )
  }
}

impl fmt::Display for Team {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    write!(f, "{} team: {}/{} {}-{}", TEAM_NAMES[self.team as usize], self.size, self.max_size, self.wins, self.losses)
  }
}

impl fmt::Display for Player {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    write!(
      f,
      "#{} {} ({}) type: {} team: {} wins: {} losses: {} tks: {}",
      self.id, self.callsign, self.motto, self.player_type, TEAM_NAMES[self.team as usize], self.wins, self.losses, self.tks
    )
  }
}

fn unpack_u16(bytes: &[u8], index: usize) -> u16 {
  if bytes.len() < index * 2 + 2 {
    panic!("tried to call unpack_u16 on slice too short. len: {} index: {}", bytes.len(), index);
  }

  (bytes[index * 2] as u16 >> 8) | (bytes[index * 2 + 1] as u16)
}

fn get_response(stream: &mut TcpStream, buffer: &mut [u8], code: &[u8; 2]) {
  let cmd_data_length;
  let mut cmd_buffer = [0u8; 4];

  loop {
    stream.read_exact(&mut cmd_buffer).unwrap();

    if &cmd_buffer[2..4] != code {
      continue;
    }

    cmd_data_length = unpack_u16(&cmd_buffer, 0);
    stream.read_exact(&mut buffer[0..cmd_data_length as usize]).unwrap();

    break;
  }
}

fn cmd(stream: &mut TcpStream, buffer: &mut [u8], code: &[u8; 2]) {
  stream.write_all(&[0u8, 0u8, code[0], code[1]]).unwrap();
  get_response(stream, buffer, code);
}

pub fn query(host: &str, port: u16) -> Query {
  let mut stream = TcpStream::connect(format!("{}:{}", host, port)).unwrap();

  // send magic header
  stream.write_all(b"BZFLAG\r\n\r\n").unwrap();

  let mut buffer = [0u8; BUFFER_SIZE];

  // check magic and protocol version
  stream.read_exact(&mut buffer[0..9]).unwrap();

  // must match https://github.com/BZFlag-Dev/bzflag/blob/58736bd8fb2094f8ef9a17961e838d12634dd871/src/date/buildDate.cxx#L106-L110
  // note: last byte is 0xff if full, otherwise it is undefined (due to use of memcpy) - see https://github.com/BZFlag-Dev/bzflag/blob/58736bd8fb2094f8ef9a17961e838d12634dd871/src/bzfs/bzfs.cxx#L1389
  if &buffer[0..8] != b"BZFS0221" {
    let text = from_utf8(&buffer).unwrap();
    panic!("invalid protocol version: {}", text);
  } else if buffer[9] == 0xff {
    panic!("server is full");
  }

  cmd(&mut stream, &mut buffer, MSG_QUERY_GAME);

  // must match sendQueryGame order at https://github.com/BZFlag-Dev/bzflag/blob/58736bd8fb2094f8ef9a17961e838d12634dd871/src/bzfs/bzfs.cxx#L3100-L3132

  let raw_options = unpack_u16(&buffer, 1);

  let mut query = Query {
    style: unpack_u16(&buffer, 0).into(),
    options: Options {
      flags: (raw_options & GAME_OPTION_FLAGS) > 0,
      jumping: (raw_options & GAME_OPTION_JUMPING) > 0,
      inertia: (raw_options & GAME_OPTION_INERTIA) > 0,
      ricochet: (raw_options & GAME_OPTION_RICOCHET) > 0,
      shaking: (raw_options & GAME_OPTION_SHAKING) > 0,
      antidote: (raw_options & GAME_OPTION_ANTIDOTE) > 0,
      handicap: (raw_options & GAME_OPTION_HANDICAP) > 0,
      no_team_kills: (raw_options & GAME_OPTION_NO_TEAM_KILLS) > 0,
    },
    max_players: unpack_u16(&buffer, 2),
    max_shots: unpack_u16(&buffer, 3),
    shake_wins: unpack_u16(&buffer, 16),
    shake_timeout: unpack_u16(&buffer, 17), // deciseconds (1/10th second)
    max_player_score: unpack_u16(&buffer, 18),
    max_team_score: unpack_u16(&buffer, 19),
    max_time: unpack_u16(&buffer, 20),
    elapsed_time: unpack_u16(&buffer, 21),
    teams: vec![],
    players: vec![],
  };

  // store observer team size for later use
  let observer_size = unpack_u16(&buffer, 9);
  // store max team sizes for later use
  let max_team_sizes = [
    unpack_u16(&buffer, 10),
    unpack_u16(&buffer, 11),
    unpack_u16(&buffer, 12),
    unpack_u16(&buffer, 13),
    unpack_u16(&buffer, 14),
    unpack_u16(&buffer, 15),
  ];

  cmd(&mut stream, &mut buffer, MSG_QUERY_PLAYERS);

  // must match sendQueryPlayers at https://github.com/BZFlag-Dev/bzflag/blob/58736bd8fb2094f8ef9a17961e838d12634dd871/src/bzfs/bzfs.cxx#L3134-L3165
  let num_players = unpack_u16(&buffer, 1);

  get_response(&mut stream, &mut buffer, MSG_TEAM_UPDATE);

  // must match sendTeamUpdate at https://github.com/BZFlag-Dev/bzflag/blob/58736bd8fb2094f8ef9a17961e838d12634dd871/src/bzfs/bzfs.cxx#L470-L505
  let num_teams = buffer[0];
  for i in 0..num_teams {
    let team_buffer = &buffer[1..buffer.len()];
    let team_id = unpack_u16(team_buffer, i as usize * 4);

    query.teams.push(Team {
      team: team_id.into(),
      size: unpack_u16(team_buffer, i as usize * 4 + 1),
      max_size: max_team_sizes[team_id as usize],
      wins: unpack_u16(team_buffer, i as usize * 4 + 2),
      losses: unpack_u16(team_buffer, i as usize * 4 + 2),
    });
  }

  // manually add observer team as it is not part of MsgTeamUpdate
  query.teams.push(Team {
    team: TeamColor::Observer,
    size: observer_size,
    max_size: max_team_sizes[5],
    wins: 0,
    losses: 0,
  });

  for _ in 0..num_players {
    get_response(&mut stream, &mut buffer, MSG_ADD_PLAYER);
    let player_buffer = &buffer[1..buffer.len()];

    query.players.push(Player {
      id: buffer[0],
      player_type: unpack_u16(player_buffer, 0),
      team: unpack_u16(player_buffer, 1).into(),
      wins: unpack_u16(player_buffer, 2),
      losses: unpack_u16(player_buffer, 3),
      tks: unpack_u16(player_buffer, 4),
      callsign: from_utf8(&player_buffer[10..42]).unwrap().trim_matches(char::from(0)).to_owned(),
      motto: from_utf8(&player_buffer[42..170]).unwrap().trim_matches(char::from(0)).to_owned(),
    });
  }

  query
}
