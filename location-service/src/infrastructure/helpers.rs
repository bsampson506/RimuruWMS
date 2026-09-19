use std::str::FromStr;
use std::env::var;
use crate::infrastructure::app_error::AppError;

/// Parse environment variable and parse to a specific type.
pub fn parse_env_var<T>(env_var_key: &str) -> Result<T, AppError>
where
    T: FromStr,
    T::Err: std::fmt::Display,
{
    let env_variable = var(&env_var_key)?;

    // Manually map Err potentially returned from parse,
    // as we don't know the type parsed(i.e ParseIntError, ParseStringError, etc),
    env_variable.parse::<T>().map_err(|e| AppError::Parse(e.to_string()))
}
