use thiserror::Error;
#[derive(Debug, Error)]
pub enum DomainError {
    #[error("Invalid or missing warehouse ID")]
    InvalidWarehouseID,
    #[error("Invalid or missing barcode")]
    InvalidBarcode,
    #[error("Invalid or missing zone")]
    InvalidZone,
    #[error("Dimensions cannot be negative.")]
    NonPositiveDimensions,
    #[error("Invalid storage type for location")]
    InvalidStorageType,
    #[error("Invalid location status")]
    InvalidLocationStatus,
    #[error("Invalid location lock status")]
    InvalidLocationLockStatus,
    #[error("Location is locked")]
    LocationLocked,
}
