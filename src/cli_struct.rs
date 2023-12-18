use clap::Parser;


#[derive(Parser, Debug)]
pub struct CliArgs {
    /// IP address of the controller
    #[arg(short, long)]
    pub ip_string: String,

    /// the address to find the load device (HP606n) on the GPIB bus
    #[arg(short, long)]
    pub gpib_addr: u8,

    /// the current to draw from the battery in Amps
    #[arg(short='d', long)]
    pub discharge_current: f64,

    /// the range for the current sensor on the machine. this differs on different machines,
    /// so check which range best matches your discharge current.
    /// This field is the index of the range, so for the 6063B which has ranges [0, 1A] and [0, 10A],
    /// you would enter `0` for discharge currents less than 1A, and `1` otherwise
    #[arg(short='c', long, default_value_t = 1u8)]
    pub current_range: u8,

    /// the voltage to stop discharging the battery.
    #[arg(short='v', long, default_value_t = 0f64)]
    pub cutoff_voltage: f64,

    /// path and name for the output file
    #[arg(short='p', long)]
    pub output_file: String,

    /// number of times to check the voltage per minute.
    /// This affects cutoff voltage checks if set too low.
    #[arg(short='r', long)]
    pub polling_rate: f64
}