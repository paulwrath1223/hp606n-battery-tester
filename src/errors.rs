use std::net::AddrParseError;
use std::num::{ParseFloatError, ParseIntError};
use std::string::FromUtf8Error;
use std::time::SystemTimeError;
use prologix_gpib_ethernet_controller_manager::errors::GpibControllerError;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum BatTestError {
    #[error("Error trying to send or receive data: {0}")]
    TcpIoError(#[from] std::io::Error),
    #[error("reqwest error: {0}")]
    ReqwestError(#[from] reqwest::Error),
    #[error("error parsing integer from a string: {0}")]
    ParseIntError(#[from] ParseIntError),
    #[error("error parsing string from TCPStream: {0}")]
    ParseStringError(#[from] FromUtf8Error),
    #[error("error parsing Ip address: {0}")]
    ParseIpAddressError(#[from] AddrParseError),
    #[error("error in GPIB controller library: {0}")]
    ControllerLibraryError(#[from] GpibControllerError),
    #[error("error in CSV library: {0}")]
    CsvError(#[from] csv::Error),
    #[error("error parsing Float: {0}")]
    ParseFloatError(#[from] ParseFloatError),
    #[error("error with system time: {0}")]
    SystemTimeError(#[from] SystemTimeError),
    #[error("Error converting path to string")]
    PathToStringError,
    #[error("Error converting scientific notation string to float")]
    SciNotParseError

}