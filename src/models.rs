#[derive(Debug, Clone)]
pub enum Role {
    LabManager,
    Assistant
}

#[derive(Debug, Clone)]
pub struct User {
    pub id: i64,
    pub username: String,
    pub role: Role
}

#[derive(Debug, Clone)]
pub enum ItemStatus {
    Available,
    InService,
    WrittenOff
}

#[derive(Debug, Clone)]
pub struct InventoryItem {
    pub id: i64,
    pub name: String,
    pub inv_num: String,
    pub status: ItemStatus,
    pub vendor: String
}

#[derive(Debug, Clone)]
pub struct MaintenanceLog {
    pub id: i64,
    pub item_id: i64,
    pub timestamp: String,
    pub description: String,
    pub by_user_id: i64
}

#[derive(Debug, Clone)]
pub struct AuditLog {
    pub id: i64,
    pub timestamp: String,
    pub user_id: i64,
    pub action: String,
    pub details: String
}
