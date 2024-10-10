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
    MissingService,
    MissingPrevalence,
    MissingProbes,
    InvalidProtocol(usize),
    InvalidPrevalence(usize),
}

fn parse_file(file: &str) -> Result<Service, ParseError> {
    Ok(Service { 
        name, 
        prevalence, 
        probes
    })
}