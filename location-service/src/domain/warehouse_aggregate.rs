use uuid::Uuid;
use std::collections::HashMap;

#[derive(Debug)]
pub struct FacilityAggregate {
    pub id: Uuid,
    pub tenant_id: Uuid,
    pub warehouse_id: String,
    pub warehouse_metadata: WarehouseMetadata,
    pub warehouse_config: WarehouseConfig,
}

#[derive(Debug)]
pub struct WarehouseMetadata {
    pub warehouse_name: String,
    pub warehouse_address: String,
    pub warehouse_city: String,
    pub warehouse_state: String,
    pub warehouse_zip: String,
    pub warehouse_country: String,
    pub warehouse_timezone: String,
    pub warehouse_currency: String,
    pub warehouse_language: String,
    pub warehouse_email: String,
    pub warehouse_phone: String,
    pub warehouse_fax: String,
    pub warehouse_website: String,
    pub warehouse_logo: String,
    pub warehouse_banner: String,
}

#[derive(Debug)]
pub struct WarehouseConfig {
    pub is_multi_tentant: bool,
    pub erp_mappings: HashMap<String, String>,
    pub blind_receiving_enabled: bool,
    pub over_receiving_tolerance: f64,
    pub stage_scan_required: bool,
    pub putaway_stragegy: PutawayStrategy,
    pub default_inventory_lock: LockCodeRule,
    pub shipping_policy: ShippingPolicy,
    pub verify_scan_type: VerifyScanType,
    pub short_pick_action: ShortPickAction,
}

#[derive(Debug)]
pub enum PutawayStrategy {
    FIFO,
    LIFO,
    FEFO,
}

#[derive(Debug)]
pub enum LockCodeRule {
    Available,
    Unavailable,
}

#[derive(Debug)]
pub enum ShippingPolicy {
    ShipComplete,
    ShipPartial,
}

#[derive(Debug)]
pub enum VerifyScanType {
    LocationScan,
    Fullscan,
}

#[derive(Debug)]
pub enum ShortPickAction {
    TaskLevel,
    BinLevel,
    NoAdjustment,
}