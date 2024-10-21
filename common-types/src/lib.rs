use schemars::JsonSchema;

#[derive(serde::Serialize, Debug, JsonSchema)]
pub struct Message {
    pub message: String,
}


#[derive(serde::Serialize, Debug, JsonSchema)]
pub struct CurrentUser {
    pub email: String,
}
