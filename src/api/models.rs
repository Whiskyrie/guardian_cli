use serde::{Deserialize, Serialize};

// --- API Response Wrapper ---

#[derive(Debug, Deserialize)]
pub struct GraphqlResponse<T> {
    pub data: Option<T>,
    pub errors: Option<Vec<GraphqlError>>,
}

#[derive(Debug, Deserialize)]
#[allow(dead_code)]
pub struct GraphqlError {
    pub message: String,
    #[serde(rename = "extensions", default)]
    pub extensions: Option<serde_json::Value>,
}

// --- User ---

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct User {
    pub id: String,
    pub email: String,
    #[serde(rename = "firstName")]
    pub first_name: Option<String>,
    #[serde(rename = "lastName")]
    pub last_name: Option<String>,
    pub role: Option<String>,
    pub roles: Option<Vec<String>>,
    #[serde(rename = "lastLoginAt")]
    pub last_login_at: Option<String>,
    #[serde(rename = "createdAt")]
    pub created_at: String,
    #[serde(rename = "updatedAt")]
    pub updated_at: String,
    #[serde(rename = "fullName")]
    pub full_name: Option<String>,
    #[serde(rename = "displayName")]
    pub display_name: String,
}

// --- User Error ---

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserError {
    pub message: String,
    pub code: Option<String>,
    pub field: Option<String>,
}

// --- Auth Payloads ---

#[derive(Debug, Deserialize)]
pub struct LoginResponse {
    #[serde(rename = "loginUser")]
    pub login_user: AuthPayload,
}

#[derive(Debug, Deserialize)]
pub struct RegisterResponse {
    #[serde(rename = "registerUser")]
    pub register_user: AuthPayload,
}

#[derive(Debug, Deserialize)]
pub struct RefreshTokenResponse {
    #[serde(rename = "refreshToken")]
    pub refresh_token: RefreshPayload,
}

#[derive(Debug, Deserialize)]
pub struct LogoutResponse {
    #[serde(rename = "logoutUser")]
    pub logout_user: MessagePayload,
}

#[derive(Debug, Deserialize)]
pub struct AuthPayload {
    pub success: bool,
    pub message: Option<String>,
    pub token: Option<String>,
    pub user: Option<User>,
    pub errors: Vec<UserError>,
}

#[derive(Debug, Deserialize)]
#[allow(dead_code)]
pub struct RefreshPayload {
    pub success: bool,
    pub message: Option<String>,
    pub token: Option<String>,
    pub user: Option<User>,
    pub errors: Vec<UserError>,
}

#[derive(Debug, Deserialize)]
pub struct MessagePayload {
    pub success: bool,
    pub message: Option<String>,
    pub errors: Vec<UserError>,
}

// --- Current User ---

#[derive(Debug, Deserialize)]
pub struct CurrentUserResponse {
    #[serde(rename = "currentUser")]
    pub current_user: Option<User>,
}

// --- Users List ---

#[derive(Debug, Deserialize)]
pub struct UsersResponse {
    pub users: UsersConnection,
}

#[derive(Debug, Deserialize)]
#[allow(dead_code)]
pub struct UsersConnection {
    pub edges: Vec<UserEdge>,
    #[serde(rename = "pageInfo")]
    pub page_info: PageInfo,
}

#[derive(Debug, Deserialize)]
pub struct UserEdge {
    pub node: User,
}

#[derive(Debug, Deserialize)]
#[allow(dead_code)]
pub struct PageInfo {
    #[serde(rename = "hasNextPage")]
    pub has_next_page: bool,
    #[serde(rename = "hasPreviousPage")]
    pub has_previous_page: bool,
    #[serde(rename = "startCursor")]
    pub start_cursor: Option<String>,
    #[serde(rename = "endCursor")]
    pub end_cursor: Option<String>,
}

// --- Delete User ---

#[derive(Debug, Deserialize)]
pub struct DeleteUserResponse {
    #[serde(rename = "deleteUser")]
    pub delete_user: MessagePayload,
}

// --- Update User Role ---

#[derive(Debug, Deserialize)]
pub struct UpdateUserRoleResponse {
    #[serde(rename = "updateUserRole")]
    pub update_user_role: UpdateRolePayload,
}

#[derive(Debug, Deserialize)]
pub struct UpdateRolePayload {
    pub user: Option<User>,
    pub success: bool,
    pub message: String,
    pub errors: Vec<UserError>,
}

// --- Audit Log ---

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuditLog {
    pub id: String,
    pub action: String,
    pub resource: String,
    #[serde(rename = "resourceId")]
    pub resource_id: Option<String>,
    pub result: String,
    #[serde(rename = "createdAt")]
    pub created_at: String,
    #[serde(rename = "ipAddress")]
    pub ip_address: Option<String>,
    #[serde(rename = "userAgent")]
    pub user_agent: Option<String>,
    #[serde(rename = "failureReason")]
    pub failure_reason: Option<String>,
    pub user: Option<User>,
}

#[derive(Debug, Deserialize)]
pub struct AuditLogsResponse {
    #[serde(rename = "auditLogs")]
    pub audit_logs: AuditLogsConnection,
}

#[derive(Debug, Deserialize)]
pub struct AuditLogsConnection {
    pub edges: Vec<AuditLogEdge>,
}

#[derive(Debug, Deserialize)]
pub struct AuditLogEdge {
    pub node: AuditLog,
}

// --- GraphQL Request Body ---

#[derive(Serialize)]
pub struct GraphqlBody {
    pub query: String,
    #[serde(rename = "variables", skip_serializing_if = "Option::is_none")]
    pub variables: Option<serde_json::Value>,
}
