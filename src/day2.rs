use std::{error::Error, fmt::Display, num::ParseIntError, str::FromStr};

#[derive(Debug, Clone)]
pub enum Command {
    Forward(usize),
    Down(usize),
    Up(usize),
}

#[derive(Debug)]
pub enum ParseCommandError {
    MissingName,
    MissingArgument,
    ExtraTokens(usize),
    WrongArgument(ParseIntError),
    UnknownCommand(String),
}

impl FromStr for Command {
    type Err = ParseCommandError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut tokens = s.split_whitespace();

        let cmd = tokens.next().ok_or(ParseCommandError::MissingName)?;
        let arg = tokens.next().ok_or(ParseCommandError::MissingArgument)?;
        let extra = tokens.count();
        if extra != 0 {
            return Err(ParseCommandError::ExtraTokens(extra));
        }

        // we can parse number here as all commands use a single numeric arg
        let arg: usize = arg.parse()
            .map_err(|e| ParseCommandError::WrongArgument(e))?;

        // dispatch command type
        match cmd {
            "forward" => Ok(Command::Forward(arg)),
            "down" => Ok(Command::Down(arg)),
            "up" => Ok(Command::Up(arg)),
            other => Err(ParseCommandError::UnknownCommand(other.to_string())),
        }
    }
}

impl Display for ParseCommandError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Failed to parse command: {:?}", self)
    }
}

impl Error for ParseCommandError {}


#[derive(Debug)]
pub struct Position {
    horizontal: isize,
    depth: isize,
    aim: isize,
}

impl Position {
    pub fn new() -> Self {
        Self {
            horizontal: 0,
            depth: 0,
            aim: 0,
        }
    }

    pub fn result(&self) -> isize {
        self.horizontal * self.depth
    }

    // Part 1
    pub fn update1(&mut self, cmd: Command) {
        match cmd {
            Command::Forward(n) => self.horizontal += n as isize,
            Command::Down(n) => self.depth += n as isize,
            Command::Up(n) => {
                self.depth -= n as isize;
                assert!(self.depth >= 0, "Negative depth: {}", self.depth);
            },
        }
    }

    // Part 2
    pub fn update2(&mut self, cmd: Command) {
        match cmd {
            Command::Down(n) => self.aim += n as isize,
            Command::Up(n) => self.aim -= n as isize,
            Command::Forward(n) => {
                self.horizontal += n as isize;
                self.depth += self.aim * n as isize;
            }
        }
    }
}

pub fn move_by<F>(commands: &[Command], mut update: F) -> Position
    where F: FnMut(&mut Position, Command)
{
    let mut pos = Position::new();
    for cmd in commands.iter() {
        update(&mut pos, cmd.clone())
    }
    pos
}
