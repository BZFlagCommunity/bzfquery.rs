use bzfquery;

#[cfg(feature = "color")]
mod color;
#[cfg(feature = "color")]
use color::*;

#[cfg(not(feature = "color"))]
mod no_color;
#[cfg(not(feature = "color"))]
use no_color::*;

const DEFAULT_PORT: u16 = 5154;

fn print_heading(text: &str) {
  println!("{}{}{}{}{}", BRIGHT, UNDERLINE, WHITE, text, RESET);
}

fn bool_to_string(value: bool) -> String {
  format!("{}{}{}{}", BRIGHT, if value { GREEN } else { RED }, if value { "yes" } else { "no" }, RESET)
}

fn team_color(team: u16) -> &'static str {
  match team {
    0 | 7 => YELLOW,
    1 => RED,
    2 => GREEN,
    3 => BLUE,
    4 => MAGENTA,
    5 | 6 => WHITE,
    _ => "",
  }
}

fn player_score_len(player: &bzfquery::Player) -> usize {
  (player.wins as i32 - player.losses as i32).to_string().len()
}

fn main() {
  let mut args: Vec<String> = std::env::args().collect();
  args.remove(0); // remove first arguement which is self

  if args.len() != 1 || args[0] == "help" || args[0] == "-h" || args[0] == "--help" {
    println!("{} v{}\n", env!("CARGO_PKG_NAME"), env!("CARGO_PKG_VERSION"));
    println!("Usage:");
    println!("    {} host[:port]", env!("CARGO_PKG_NAME"));
    std::process::exit(0);
  }

  let parts: Vec<&str> = args[0].split(":").collect();
  let host = parts[0];
  let port = match parts.len() {
    2 => parts[1].parse().unwrap_or(DEFAULT_PORT),
    _ => DEFAULT_PORT,
  };

  let query = bzfquery::query(host, port);

  print_heading("Configuration");
  println!("Game style:  {}{}{}{}", BRIGHT, WHITE, bzfquery::GAME_STYLES[query.style as usize], RESET);
  println!("Flags:       {}", bool_to_string(query.options.flags));
  println!("Jumping:     {}", bool_to_string(query.options.jumping));
  println!("Ricochet:    {}", bool_to_string(query.options.ricochet));
  println!("Team kills:  {}", bool_to_string(!query.options.no_team_kills));

  println!("");
  print_heading("Teams");

  for team in query.teams {
    if team.max_size == 0 {
      continue;
    }

    println!(
      " • {bright}{}{:<9}{reset} [{bright}{}{}{reset}]",
      team_color(team.team),
      bzfquery::TEAM_NAMES[team.team as usize],
      WHITE,
      team.wins as i32 - team.losses as i32,
      bright = BRIGHT,
      reset = RESET
    );
  }

  let mut max_player_callsign_length = 0;
  let mut max_player_score_length = 0;
  for player in &query.players {
    if player.callsign.len() > max_player_callsign_length {
      max_player_callsign_length = player.callsign.len();
    }

    let score_string_len = player_score_len(player);
    if score_string_len > max_player_score_length {
      max_player_score_length = score_string_len;
    }
  }

  max_player_callsign_length += 2;
  max_player_score_length += 2;

  println!("");
  print_heading("Players");
  if query.players.len() > 0 {
    for player in query.players {
      println!(
        " • {callsign}{callsign_padding}[{bright}{white}{score}{reset}]{score_padding}({bright}{team_color}{team}{reset})",
        bright = BRIGHT,
        white = WHITE,
        reset = RESET,
        callsign = player.callsign,
        score = player.wins as i32 - player.losses as i32,
        team_color = team_color(player.team),
        team = bzfquery::TEAM_NAMES[player.team as usize],
        callsign_padding = (0..max_player_callsign_length - player.callsign.len()).map(|_| " ").collect::<String>(),
        score_padding = (0..max_player_score_length - player_score_len(&player)).map(|_| " ").collect::<String>()
      );
    }
  } else {
    println!(" No players online");
  }
}
