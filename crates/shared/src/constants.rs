pub const AUTHORIZATION_HEADER: &str = "Authorization";

pub const IGNORE_ROUTES: [&str; 7] = [
    "/users/login",
    "/users/refresh-token",
    "/oauth/github/login",
    "/rapidoc",
    "/redoc",
    "/swagger-ui",
    "/api-docs",
];

pub const MESSAGE_INVALID_TOKEN: &str = "Invalid token, please login again";

pub const EMPTY: &str = "";

pub const APPLICATION_NAME: &str = "syncflow/0.1.0";
