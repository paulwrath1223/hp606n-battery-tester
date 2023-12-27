mod errors;
mod cli_struct;

use csv::Writer;
use chrono::Utc;
use tokio::time::{self, Duration};
use std::net::IpAddr;
use clap::Parser;
use log::{debug, info};
use prologix_gpib_ethernet_controller_manager::errors::GpibControllerError;
use crate::errors::BatTestError;
use prologix_gpib_ethernet_controller_manager::gpib_controller::GpibController;
use crate::cli_struct::CliArgs;


#[tokio::main]
async fn main() -> Result<(), BatTestError> {
    let call_time = Utc::now();
    let args = CliArgs::parse();
    let mut output = Writer::from_path(&args.output_file)?;
    info!("opened csv");

    let mut watts_sum: f64 = 0f64;
    let mut amps_sum: f64 = 0f64;
    let mut entries: usize = 0;

    output.write_record(&["Call time", &call_time.to_string(), "time the executable was ran"])?;
    output.write_record(&["IP address", &args.ip_string, "IP address of the controller"])?;
    output.write_record(&["GPIB address", &args.gpib_addr.to_string(), "address of the GPIB device"])?;
    output.write_record(&["Target discharge current", &args.discharge_current.to_string(), "Target discharge rate in amps"])?;
    output.write_record(&["Current range", &args.current_range.to_string(), "the range for the current sensor on the machine. this differs on different machines, so check which range best matches your discharge current. This field is the index of the range, so for the 6063B which has ranges [0, 1A] and [0, 10A], you would enter 0 for discharge currents less than 1A, and 1 otherwise"])?;
    output.write_record(&["Cutoff voltage", &args.cutoff_voltage.to_string(), "the voltage to stop the discharge"])?;
    output.write_record(&["Output path", &args.output_file, "the path of this very file you are reading right now relative to the executable"])?;
    output.write_record(&["Polling rate", &args.polling_rate.to_string(), "how many times a minute the device was queried"])?;

    let ip_addr: IpAddr = args.ip_string.parse()?;
    let mut controller = GpibController::try_new_from(ip_addr)?;
    controller.gpib_send_to_addr("*IDN?\n", args.gpib_addr)?;
    let raw_target_device_info = controller.read_data()?;
    let target_device_info = &raw_target_device_info[0..raw_target_device_info.len() - 2];
    info!("Target device identification: {}", target_device_info);
    output.write_record(&["Target device identification", &target_device_info, "whatever the device on the given address responded to `*IDN?` with"])?;

    controller.gpib_send_to_addr("INPUT OFF\n", args.gpib_addr)?;
    controller.gpib_send_to_addr("MODE:CURR\n", args.gpib_addr)?;

    let mut temp_message: String = String::from("CURR:LEVEL ");
    temp_message.push_str(&args.discharge_current.to_string());
    temp_message.push('\n');

    controller.gpib_send_to_addr(&temp_message, args.gpib_addr)?;


    output.write_record(&["", "", ""])?;
    output.write_record(&["time", "voltage", "current"])?;
    output.flush()?;
    controller.gpib_send_to_addr("INPUT ON\n", args.gpib_addr)?;
    info!("turned on test");
    let start_time = std::time::SystemTime::now();
    let mut voltage: f64;
    let mut current: f64;
    (voltage, _) = query(&mut controller, args.gpib_addr)?;

    let interval_delay = Duration::from_secs_f64(60f64 / args.polling_rate);

    debug!("interval delay: {:?}", interval_delay);

    let mut interval = time::interval(interval_delay);

    interval.tick().await;
    while voltage > args.cutoff_voltage {
        interval.tick().await;
        (voltage, current) = query(&mut controller, args.gpib_addr)?;
        output.write_record(&[
            &Utc::now().format("%T").to_string(),
            &voltage.to_string(),
            &current.to_string()
        ])?;
        output.flush()?;
        amps_sum += current;
        watts_sum += current * voltage;
        entries += 1;
        debug!("{} amps\t\t{} volts", current, voltage);
    }
    let total_hours = start_time.elapsed()?.as_secs_f64() / 3600f64;
    controller.gpib_send_to_addr("INPUT OFF\n", args.gpib_addr)?;
    output.flush()?;
    let avg_amps = amps_sum / entries as f64;
    let avg_watts = watts_sum / entries as f64;
    let total_amp_hours = avg_amps * total_hours;
    let total_watt_hours = avg_watts * total_hours;
    info!("Discharge complete. Sucked {} watt hours and {} amp hours out of that bad boy.", total_watt_hours, total_amp_hours);
    output.write_record(&["", "", ""])?;
    output.write_record(&["Finish time", "Watt hours", "Amp hours"])?;
    output.write_record(&[
        &Utc::now().format("%T").to_string(),
        &total_watt_hours.to_string(),
        &total_amp_hours.to_string()
    ])?;
    controller.gpib_send_to_addr("INPUT OFF\n", args.gpib_addr)?;
    output.flush()?;
    Ok(())
}

pub fn query(gpib_controller: &mut GpibController, gpib_address: u8) -> Result<(f64, f64), BatTestError> {
    gpib_controller.gpib_send_to_addr("MEAS:VOLT?\n", gpib_address)?;
    let voltage_string = gpib_controller.read_data()?.trim();
    let voltage: f64 = sci_not_to_float(voltage_string)?;
    gpib_controller.gpib_send_to_addr("MEAS:CURR?\n", gpib_address)?;
    let current_string = gpib_controller.read_data()?.trim();
    let current: f64 = sci_not_to_float(current_string)?;
    Ok((voltage, current))
}

const KOSHER_CHARS: &str = "0123456789-.E";

pub fn sci_not_to_float(str_in: &str) -> Result<f64, BatTestError> {
    let mut upper_str = str_in.to_ascii_uppercase();
    upper_str.retain(|c| { KOSHER_CHARS.contains(c) });
    let mut split = upper_str.splitn(2, 'E');

    let base = split.next().ok_or(BatTestError::SciNotParseError)?;
    let scale = split.next().ok_or(BatTestError::SciNotParseError)?;
    let float_base: f64 = base.parse()?;
    let scale_int: i32 = scale.parse()?;
    Ok(float_base * (10f64.powi(scale_int)))
}

pub fn gpib_send_and_listen_wrapper(gpib_controller: &mut GpibController, message: &str, gpib_address: u8, ignore_response: bool) -> Result<Option<String>, BatTestError>{
    gpib_controller.gpib_send_to_addr(message, gpib_address)?;

    if ignore_response {
        return Ok(None)
    }
    match gpib_controller.read_data() {
        Ok(s) => {
            Ok(Some(s.to_string()))
        }
        Err(GpibControllerError::TcpIoError(_)) => {
            Ok(None)
        }
        Err(e) => {
            Err(BatTestError::ControllerLibraryError(e))
        }
    }
}