mod cli;
pub use cli::CLI;

mod utils;

// openocd -f interface/stlink-v2-1.cfg -f target/stm32f3x.cfg -c 'bindto 0.0.0.0'