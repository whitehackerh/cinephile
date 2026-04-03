pub(crate) struct SignInInput {
    pub email: String,
    pub password: String,
}

pub(crate) struct SignInOutput {
    pub token: String
}