#[cfg(test)]

use dotenv;

pub fn init_env() {
    dotenv::from_filename(".env.test").ok();
}
