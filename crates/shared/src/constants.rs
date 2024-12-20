pub const AUTHORIZATION_HEADER: &str = "Authorization";

pub const IGNORE_ROUTES: [&str; 12] = [
    "/users/login",
    "/users/signup",
    "/users/refresh-token",
    "/oauth/github/login",
    "/rapidoc",
    "/redoc",
    "/swagger-ui",
    "/api-docs",
    "/rmq/auth/user",
    "/rmq/auth/vhost",
    "/rmq/auth/resource",
    "/rmq/auth/topic",
];

pub const IGNORE_PROJECT_OWNERSHIP_ROUTES: [&str; 3] =
    ["/projects/create", "/projects/list", "/projects/summarize"];

pub const MESSAGE_INVALID_TOKEN: &str = "Invalid token, please login again";

pub const EMPTY: &str = "";

pub const APPLICATION_NAME: &str = "syncflow/0.1.0";
