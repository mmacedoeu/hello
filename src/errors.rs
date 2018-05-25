#[derive(Debug, ErrorChain)]
pub enum ErrorKind {
    Msg(String),

    #[error_chain(foreign)]
    Fmt(::std::fmt::Error),

    #[cfg(unix)]
    #[error_chain(foreign)]
    Io(::std::io::Error),

    #[error_chain(foreign)]
    Clap(::clap::Error),    
}