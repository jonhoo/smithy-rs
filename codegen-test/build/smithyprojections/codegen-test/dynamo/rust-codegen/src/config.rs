// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

pub struct Config {
    pub(crate) token_provider: Box<dyn crate::idempotency_token::ProvideIdempotencyToken>,
}
impl Config {
    pub fn builder() -> ConfigBuilder {
        ConfigBuilder::default()
    }
}
#[derive(Default)]
pub struct ConfigBuilder {
    token_provider: Option<Box<dyn crate::idempotency_token::ProvideIdempotencyToken>>,
}
impl ConfigBuilder {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn token_provider(
        mut self,
        token_provider: impl crate::idempotency_token::ProvideIdempotencyToken + 'static,
    ) -> Self {
        self.token_provider = Some(Box::new(token_provider));
        self
    }

    pub fn build(self) -> Config {
        Config {
            token_provider: self
                .token_provider
                .unwrap_or_else(|| Box::new(crate::idempotency_token::default_provider())),
        }
    }
}
