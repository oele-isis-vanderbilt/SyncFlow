pub const AUTHORIZATION_HEADER: &str = "Authorization";

pub const IGNORE_ROUTES: [&str; 6] = [
    "/users/login",
    "/users/refresh-token",
    "/rapidoc",
    "/redoc",
    "/swagger-ui",
    "/api-docs",
];

pub const MESSAGE_INVALID_TOKEN: &str = "Invalid token, please login again";

pub const EMPTY: &str = "";
