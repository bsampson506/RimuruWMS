use uuid::Uuid;
use serde::{Deserialize};
use crate::{
    domain::location_events::LocationEvent
};

#[derive(Debug)]
pub struct LocationAggregate {
    pub id: Uuid,
    pub tenant_id: Uuid,
    pub barcode: String,
    pub warehouse_id: String,
    pub zone: String,
    pub aisle: String,
    pub bay: u16,
    pub level: u8,
    pub bin: u8,
    pub dimensions: LocationDimensions,
    pub location_type: LocationType,
    pub storage_type: StorageType,
    pub location_status: LocationStatus,
    pub location_lock_status: LocationLockStatus,
}

impl LocationAggregate {

    pub fn apply_event(&mut self, event: LocationEvent) {
        match event {
            LocationEvent::LocationCreated(event) => {
                self.id = event.id;
                self.tenant_id = event.tenant_id;
                self.barcode = event.barcode;
                self.warehouse_id = event.warehouse_id;
                self.zone = event.zone;
                self.aisle = event.aisle;
                self.bay = event.bay as u16;
                self.level = event.level as u8;
                self.bin = event.bin as u8;
                self.dimensions = LocationDimensions {
                    length: event.length,
                    width: event.width,
                    height: event.height,
                };
                self.location_type = event.location_type;
                self.storage_type = event.storage_type;
                self.location_status = event.location_status;
                self.location_lock_status = event.location_lock_status;
            },
            LocationEvent::LocationDeleted(_) => {}
            LocationEvent::LocationCapacityUpdated(event) => {
                self.dimensions = LocationDimensions {
                    length: event.length,
                    width: event.width,
                    height: event.height,
                };
            },
            LocationEvent::LocationLockStatusUpdate(event) => {
                self.location_lock_status = event.location_lock_status;
            },
            LocationEvent::LocationStatusUpdate(event) => {
                self.location_status = event.location_status;
            },
            LocationEvent::LocationTypeUpdate(event) => {
                self.location_type = event.location_type;
            },
        }
    }
}

#[derive(Debug, Deserialize)]
pub struct LocationDimensions {
    pub length: f64,
    pub width: f64,
    pub height: f64,
}

#[derive(Debug, Deserialize)]
pub enum StorageType {
    Ambient,
    Chilled,
    Frozen,
    Hazardous,
}

#[derive(Debug, Deserialize)]
pub enum LocationType {
    InboundDockDoor,
    OutboundDockDoor,
    PutawayStaging,
    ForwardPick,
    ReserveStorage,
    PackingStation,
    PickingStage,
    ShippingStage,
    ShippingQA, // Sometimes called Jackpot
    QaHold,
    DamageHold,
    NonInventory,
}

#[derive(Debug, Deserialize)]
pub enum LocationStatus {
    Active,
    Inactive,
    Maintenance,
}

#[derive(Debug, Deserialize)]
pub enum LocationLockStatus {
    Unlocked,
    InboundLocked,
    OutboundLocked,
    FullLock,
}
