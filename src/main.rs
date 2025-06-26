use std::fmt;

use swayipc::Connection;

#[derive(Debug)]
enum Error {
    Ipc(swayipc::Error),
    Plain(&'static str),
}

impl From<swayipc::Error> for Error {
    fn from(value: swayipc::Error) -> Self {
        Error::Ipc(value)
    }
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Ipc(value) => value.fmt(f),
            Self::Plain(string) => write!(f, "Error: {string}"),
        }
    }
}

struct Output {
    name: String,
    width: i32,
}

impl TryFrom<&swayipc::Output> for Output {
    type Error = Error;

    fn try_from(value: &swayipc::Output) -> Result<Self, Self::Error> {
        value
            .current_mode
            .ok_or(Error::Plain("Output lacks current mode"))
            .map(|ref mode| Output {
                name: value.name.clone(),
                width: mode.width,
            })
    }
}

fn main() -> Result<(), Error> {
    let mut connection = Connection::new().unwrap();
    let mut x: i32 = 0;
    for ref output in get_sway_outputs(&mut connection)? {
        set_sway_output_position(&mut connection, &output.name, x, 0)?;
        x += output.width;
    }
    Ok(())
}

fn get_sway_outputs(connection: &mut Connection) -> Result<Vec<Output>, Error> {
    Ok(connection
        .get_outputs()?
        .into_iter()
        .map(|ref output| output.try_into().inspect_err(|e| eprintln!("{e}")))
        .flatten()
        .collect())
}

fn set_sway_output_position(
    connection: &mut Connection,
    screen_name: &impl std::fmt::Display,
    x: i32,
    y: i32,
) -> Result<(), Error> {
    connection.run_command(format!("output {screen_name} pos {x} {y}"))?;
    Ok(())
}
