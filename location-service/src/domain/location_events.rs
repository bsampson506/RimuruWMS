use uuid::Uuid;
use serde::Deserialize;
use crate::domain::location_aggregate::{LocationLockStatus,LocationStatus,LocationType, StorageType};
// wrap events into one enum to allow one generic apply_event function.
#[derive(Debug)]
pub enum LocationEvent {
    LocationCreated(LocationCreatedEvent),
    LocationDeleted(LocationDeletedEvent),
    LocationCapacityUpdated(LocationCapacityUpdatedEvent),
    LocationLockStatusUpdate(LocationLockStatusUpdateEvent),
    LocationStatusUpdate(LocationStatusUpdateEvent),
    LocationTypeUpdate(LocationTypeUpdateEvent),
}

#[derive(Debug, Deserialize)]
pub struct LocationCreatedEvent {
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

#[derive(Debug)]
pub struct LocationDeletedEvent {
    pub tenant_id: Uuid,
    pub id: Uuid,
}

#[derive(Debug)]
pub struct LocationCapacityUpdatedEvent {
    pub id: Uuid,
    pub tenant_id: Uuid,
    pub length: f64,
    pub width: f64,
    pub height: f64,
}

#[derive(Debug)]
pub struct LocationLockStatusUpdateEvent {
    pub id: Uuid,
    pub tenant_id: Uuid,
    pub location_lock_status: LocationLockStatus,
}

#[derive(Debug)]
pub struct LocationStatusUpdateEvent {
    pub id: Uuid,
    pub tenant_id: Uuid,
    pub location_status: LocationStatus,
}
// specifies the storage type of the location, e.g. ambient, chilled, frozen, hazardous.
#[derive(Debug)]
pub struct StorageTypeUpdateEvent {
    pub id: Uuid,
    pub tenant_id: Uuid,
    pub storage_type: StorageType,
}

// Type differs from storage type, so we need to separate them.
#[derive(Debug)]
pub struct LocationTypeUpdateEvent {
    pub id: Uuid,
    pub tenant_id: Uuid,
    pub location_type: LocationType,
}