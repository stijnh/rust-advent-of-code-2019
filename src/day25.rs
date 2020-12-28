use crate::common::*;
use crate::intcode::*;
use std::collections::HashMap;
use std::convert::TryInto;

use Dir::*;
#[derive(Copy, Clone, PartialEq, Eq, Debug)]
enum Dir {
    North,
    South,
    West,
    East,
}

impl Dir {
    fn name(self) -> &'static str {
        use Dir::*;
        match self {
            North => "north",
            South => "south",
            West => "west",
            East => "east",
        }
    }

    fn opposite(self) -> Self {
        use Dir::*;
        match self {
            North => South,
            South => North,
            West => East,
            East => West,
        }
    }
}

#[derive(Debug, Default)]
struct Room {
    name: String,
    description: String,
    items: Vec<String>,
    doors: Vec<Dir>,
}

fn parse_room(output: &str) -> Result<Room> {
    let mut loc = Room::default();

    #[derive(PartialEq, Eq, Debug)]
    enum State {
        Idle,
        Doors,
        Items,
        Done,
    }
    use State::*;
    let mut state = Idle;

    for line in output.lines() {
        if line.starts_with("== ") && line.ends_with(" ==") {
            loc = default();
            loc.name = line[3..line.len() - 3].to_string();
            state = Idle;
        } else if state == Items {
            if line.starts_with("- ") {
                loc.items.push(line[2..].to_string());
            } else if line == "" {
                state = Idle;
            } else {
                bail!("invalid line {:?} in state {:?}", line, state);
            }
        } else if state == Doors {
            if line == "- north" {
                loc.doors.push(North);
            } else if line == "- south" {
                loc.doors.push(South);
            } else if line == "- west" {
                loc.doors.push(West);
            } else if line == "- east" {
                loc.doors.push(East);
            } else if line == "" {
                state = Idle;
            } else {
                bail!("invalid line {:?} in state {:?}", line, state);
            }
        } else if state == Idle {
            if line == "Doors here lead:" {
                state = Doors;
            } else if line == "Items here:" {
                state = Items;
            } else if line == "Command?" {
                state = Done;
            } else if line != "" {
                if !loc.description.is_empty() {
                    loc.description.push('\n');
                }

                loc.description += line;
            }
        } else if line != "" {
            bail!("invalid line {:?} in state {:?}", line, state);
        }
    }

    Ok(loc)
}

fn send_commands(program: &mut Program, cmds: &[&str]) -> Result<String> {
    use ExecState::*;
    let mut input = cmds
        .iter()
        .flat_map(|cmd| cmd.chars().chain(Some('\n')))
        .map(|c| c as u8 as i64);
    let mut output = String::new();

    for cmd in cmds {
        println!("> {}", cmd);
    }

    loop {
        match program.resume(&mut input)? {
            Output(c) => {
                if let Ok(c) = (c as u32).try_into() {
                    output.push(c);
                } else {
                    bail!("invalid character: {}", c);
                }
            }
            Input => break,
            Halted => {
                if output.is_empty() {
                    bail!("program halted unexpectedly");
                } else {
                    break;
                }
            }
        }
    }

    for line in output.lines() {
        println!("< {}", line);
    }

    Ok(output)
}

fn send_command(program: &mut Program, cmd: &str) -> Result<String> {
    send_commands(program, &[cmd])
}

fn move_to(program: &mut Program, dir: Dir) -> Result<Room> {
    parse_room(&send_command(program, dir.name())?)
}

fn dfs(
    program: &mut Program,
    current: &Room,
    path: &mut Vec<Dir>,
    visited: &mut HashMap<String, Vec<Dir>>,
    inventory: &mut Vec<String>,
) -> Result {
    const IGNORE_ITEMS: &[&str] = &[
        "giant electromagnet",
        "photons",
        "molten lava",
        "infinite loop",
        "escape pod",
    ];

    if visited
        .insert(current.name.to_string(), path.clone())
        .is_some()
    {
        return Ok(());
    }

    println!("visited {} via {:?}", current.name, path);

    if current.name == "Security Checkpoint" {
        return Ok(());
    }

    for item in &current.items {
        if !IGNORE_ITEMS.contains(&item.as_str()) {
            send_command(program, &format!("take {}", item))?;
            inventory.push(item.clone());
        }
    }

    for &dir in &current.doors {
        let room = move_to(program, dir)?;
        path.push(dir);

        dfs(program, &room, path, visited, inventory)?;

        path.pop();
        let _= move_to(program, dir.opposite())?;
    }

    Ok(())
}

fn pickup_everything_and_find_security(program: &mut Program) -> Result<Vec<String>> {
    let root = parse_room(&send_commands(program, &[])?)?;

    let mut visited = default();
    let mut inventory = vec![];

    dfs(
        program,
        &root,
        &mut vec![],
        &mut visited,
        &mut inventory,
    )?;

    for &dir in &visited["Security Checkpoint"] {
        let _ = move_to(program, dir);
    }

    Ok(inventory)
}

fn crack_security(program: &mut Program, inventory: &[String]) -> Result<Room> {
    let n = inventory.len();

    for mask in 0..(1 << n) {
        let mut program = program.clone();
        for i in 0..n {
            if mask & (1 << i) == 0 {
                send_command(&mut program, &format!("drop {}", inventory[i]))?;
            }
        }

        let room = move_to(&mut program, North)?;
        if room.name != "Security Checkpoint" {
            return Ok(room);
        }
    }

    bail!("failed to crack security");
}

pub(crate) fn run(_args: &[&str]) -> Result {
    let mut program = parse_program("day25")?;
    let inventory = pickup_everything_and_find_security(&mut program)?;
    crack_security(&mut program, &inventory)?;

    Ok(())
}
