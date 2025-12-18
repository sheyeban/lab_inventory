#[derive(Debug, Clone, PartialEq)]
pub enum Role {
    LabManager,
    Assistant,
}

impl Role {
    pub fn to_db(&self) -> &'static str {
        match self {
            Role::LabManager => "lab_manager",
            Role::Assistant => "assistant",
        }
    }
    pub fn from_db(s: &str) -> Self {
        match s {
            "lab_manager" => Role::LabManager,
            _ => Role::Assistant,
        }
    }
}

#[derive(Debug, Clone)]
pub struct User {
    pub id: i64,
    pub username: String,
    pub role: Role,
}

#[derive(Debug, Clone, PartialEq)]
pub enum ItemStatus {
    Available,
    InService,
    WrittenOff,
}

impl ItemStatus {
    pub fn to_db(&self) -> &'static str {
        match self {
            ItemStatus::Available => "available",
            ItemStatus::InService => "in_service",
            ItemStatus::WrittenOff => "written_off",
        }
    }
    pub fn from_db(s: &str) -> Self {
        match s {
            "available" => ItemStatus::Available,
            "in_service" => ItemStatus::InService,
            "written_off" => ItemStatus::WrittenOff,
            _ => ItemStatus::Available,
        }
    }
}

#[derive(Debug, Clone)]
pub struct Category {
    pub id: i64,
    pub name: String,
}

#[derive(Debug, Clone)]
pub struct Vendor {
    pub id: i64,
    pub name: String,
}

#[derive(Debug, Clone)]
pub struct InventoryItem {
    pub id: i64,
    pub name: String,
    pub inv_num: String,
    pub status: ItemStatus,
    pub category_id: i64,
    pub vendor_id: i64,
}

#[derive(Debug, Clone)]
pub struct MaintenanceLog {
    pub id: i64,
    pub item_id: i64,
    pub timestamp: String,
    pub description: String,
    pub by_user_id: i64,
}

#[derive(Debug, Clone)]
pub struct AuditLog {
    pub id: i64,
    pub timestamp: String,
    pub user_id: Option<i64>,
    pub action: String,
    pub details: String,
}
