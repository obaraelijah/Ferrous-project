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
    Ok(Service { 
        name, 
        prevalence, 
        probes
    })
}