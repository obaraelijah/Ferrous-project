use std::str::FromStr;

#[derive(Debug)]
pub struct Service {
    pub name: String,
    pub prevalence: Prevalence,
    pub probes: Vec<Probe>,
}

impl Service {
    pub fn from_file(file: &str) -> Result<Self, ParseError> {
        parse_file(file)
    }
}

#[derive(Debug)]
pub struct Probe {
    pub protocol: Protocol,
    pub payload: Payload,
    pub regex: String,
    pub sub_regex: Option<Vec<String>>,
}
#[derive(Debug)]
pub enum Payload {
    Empty,
    String(String),
    Base64(String),
}
#[derive(Debug, Copy, Clone)]
pub enum Protocol {
    Tcp,
    Udp,
    Tls,
}


#[derive(Debug, Copy, Clone)]
pub enum Prevalence {
    Often,
    Average,
    Obscure,
}

#[derive(Debug)]
pub enum ParseError {
    /// The initial `service: <name>` line is missing
    MissingService,
    /// The second line `prevalence:` is missing
    MissingPrevalence,
    /// The third line `probes:` is missing or has no lines following
    MissingProbes,
    
    /// A probe's value has been passed twice
    DuplicateValue(&'static str, usize),
    /// A probe's value is missing
    MissingValue(&'static str, usize),
    /// An unknown probe
    UnknownValue(usize),
    
    /// Both `payload_str` and `payload_b64` are specified
    ConflictingPayload { probe_line: usize },
    
    /// The sub regex must be the last key in any probe
    ValueAfterSubRegex(usize),
    /// A sub regex item before `sub_regex:`
    UnexpectedSubRegex(usize),
    /// The sub regex is specified but empty,
    MissingSubRegex { probe_line: usize },
    
    /// Invalid value for `protocol: `
    InvalidProtocol(usize),
    /// Invalid value for `prevalence: `
    InvalidPrevalence(usize),
}

fn parse_file(file: &str) -> Result<Service, ParseError> {
    let mut lines = file
        .lines()
        .enumerate()
        .filter(|(_, line)| !(line.is_empty() || line.trim_start().starts_with('#')))
        .map(|(index, line)| (index + 1, line));

    let name = lines
        .next()
        .ok_or(ParseError::MissingService)?
        .1
        .strip_prefix("service: ")
        .ok_or(ParseError::MissingService)?
        .to_string();

    let snd = lines.next().ok_or(ParseError::MissingPrevalence)?;
    let prevalence: Prevalence = snd
        .1
        .strip_prefix("prevalence: ")
        .ok_or(ParseError::MissingPrevalence)?
        .parse()
        .map_err(|_| ParseError::InvalidPrevalence(snd.0))?;


    lines.next().ok_or(ParseError::MissingProbes)?;
    
    let mut probes = Vec::new();

    Ok(Service { 
        name, 
        prevalence, 
        probes
    })
}