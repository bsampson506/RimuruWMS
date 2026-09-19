use serde::{Deserialize};
use uuid::Uuid;
use crate::domain::location_aggregate::{LocationType, StorageType,LocationStatus, LocationLockStatus};
use crate::domain::location_errors::DomainError;
use crate::domain::location_events::{LocationCreatedEvent};

#[derive(Deserialize, Debug)]
pub struct CreateLocationCommand {
    pub id: Uuid,
    pub tenant_id: Uuid,
    pub barcode: String,
    pub warehouse_id: String,
    pub zone: String,
    pub aisle: String,
    pub bay: u16,
    pub level: u8,
    pub bin: u8,
    pub length: f64,
    pub width: f64,
    pub height: f64,
    pub location_type: LocationType,
    pub storage_type: StorageType,
    pub location_status: LocationStatus,
    pub location_lock_status: LocationLockStatus,
}

impl CreateLocationCommand {
    pub fn create_location_command(self) -> Result<LocationCreatedEvent, DomainError> {
        if self.id.is_nil() || self.tenant_id.is_nil() {
            return Err(DomainError::InvalidBarcode);
        }
        Ok(LocationCreatedEvent{
            id: self.id,
            tenant_id: self.tenant_id,
            barcode: self.barcode,
            warehouse_id: self.warehouse_id,
            zone: self.zone,
            aisle: self.aisle,
            bay: self.bay,
            level: self.level,
            bin: self.bin,
            length: self.length,
            width: self.width,
            height: self.height,
            location_type: self.location_type,
            storage_type: self.storage_type,
            location_status: self.location_status,
            location_lock_status: self.location_lock_status,
        })
}
}