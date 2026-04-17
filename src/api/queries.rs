// --- Queries ---

pub const CURRENT_USER: &str = r#"
query {
    currentUser {
        id
        email
        firstName
        lastName
        role
        roles
        fullName
        displayName
        lastLoginAt
        createdAt
        updatedAt
    }
}
"#;

pub const USERS: &str = r#"
query($role: UserRoleEnum, $search: String, $after: String, $before: String, $first: Int, $last: Int) {
    users(role: $role, search: $search, after: $after, before: $before, first: $first, last: $last) {
        edges {
            node {
                id
                email
                firstName
                lastName
                role
                roles
                fullName
                displayName
                lastLoginAt
                createdAt
                updatedAt
            }
        }
        pageInfo {
            hasNextPage
            hasPreviousPage
            startCursor
            endCursor
        }
    }
}
"#;

pub const AUDIT_LOGS: &str = r#"
query($userId: ID, $action: String, $resource: String, $result: String, $recentHours: Int, $after: String, $first: Int) {
    auditLogs(userId: $userId, action: $action, resource: $resource, result: $result, recentHours: $recentHours, after: $after, first: $first) {
        edges {
            node {
                id
                action
                resource
                resourceId
                result
                createdAt
                ipAddress
                userAgent
                failureReason
                user {
                    id
                    email
                    displayName
                }
            }
        }
    }
}
"#;

// --- Mutations ---

pub const LOGIN_USER: &str = r#"
mutation($email: String!, $password: String!) {
    loginUser(email: $email, password: $password) {
        success
        message
        token
        user {
            id
            email
            firstName
            lastName
            role
            roles
            fullName
            displayName
        }
        errors {
            message
            code
            field
        }
    }
}
"#;

pub const REGISTER_USER: &str = r#"
mutation($email: String!, $password: String!, $firstName: String!, $lastName: String!) {
    registerUser(input: { email: $email, password: $password, firstName: $firstName, lastName: $lastName }) {
        success
        message
        token
        user {
            id
            email
            firstName
            lastName
            role
            roles
            fullName
            displayName
        }
        errors {
            message
            code
            field
        }
    }
}
"#;

pub const REFRESH_TOKEN: &str = r#"
mutation($token: String!) {
    refreshToken(token: $token) {
        success
        message
        token
        user {
            id
            email
            firstName
            lastName
            displayName
        }
        errors {
            message
            code
            field
        }
    }
}
"#;

pub const LOGOUT_USER: &str = r#"
mutation {
    logoutUser {
        success
        message
        errors {
            message
            code
            field
        }
    }
}
"#;

pub const DELETE_USER: &str = r#"
mutation($id: ID!) {
    deleteUser(id: $id) {
        success
        message
        errors {
            message
            code
            field
        }
    }
}
"#;

pub const UPDATE_USER_ROLE: &str = r#"
mutation($userId: ID!, $roleNames: [UserRoleEnum!]!) {
    updateUserRole(userId: $userId, roleNames: $roleNames) {
        user {
            id
            email
            firstName
            lastName
            role
            roles
            displayName
        }
        success
        message
        errors {
            message
            code
            field
        }
    }
}
"#;
