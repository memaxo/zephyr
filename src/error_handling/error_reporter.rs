use crate::error_handling::error::Error;
use log::{error, warn};
use sentry::{capture_error, configure_scope, init, Breadcrumb, Level};
use std::fmt;

pub struct ErrorReporter {
    sentry_dsn: Option<String>,
}

impl ErrorReporter {
    pub fn new(sentry_dsn: Option<String>) -> Self {
        Self { sentry_dsn }
    }

    pub fn init(&self) {
        if let Some(dsn) = &self.sentry_dsn {
            let _guard = init(dsn.clone());
            log::info!("Error reporter initialized with Sentry DSN");
        } else {
            log::info!("Error reporter initialized without Sentry DSN");
        }
    }

    pub fn report_error(&self, error: &Error, context: Option<&str>) {
        match error {
            Error::BlockchainError(e) => self.report_blockchain_error(e, context),
            Error::NetworkError(e) => self.report_network_error(e, context),
            Error::CryptoError(e) => self.report_crypto_error(e, context),
            Error::StorageError(e) => self.report_storage_error(e, context),
            Error::ConsensusError(e) => self.report_consensus_error(e, context),
            Error::SmartContractError(e) => self.report_smart_contract_error(e, context),
            Error::WalletError(e) => self.report_wallet_error(e, context),
            Error::ConfigurationError(e) => self.report_configuration_error(e, context),
            Error::IoError(e) => self.report_io_error(e, context),
            Error::SerializationError(e) => self.report_serialization_error(e, context),
            Error::InvalidDataError(e) => self.report_invalid_data_error(e, context),
            Error::PermissionDeniedError(e) => self.report_permission_denied_error(e, context),
            Error::ResourceNotFoundError(e) => self.report_resource_not_found_error(e, context),
            Error::UnexpectedError(e) => self.report_unexpected_error(e, context),
        }
    }

    fn report_blockchain_error(&self, error: &BlockchainError, context: Option<&str>) {
        self.log_error(error, context);
        self.capture_error(error, context);
    }

    fn report_network_error(&self, error: &NetworkError, context: Option<&str>) {
        self.log_error(error, context);
        self.capture_error(error, context);
    }

    fn report_crypto_error(&self, error: &CryptoError, context: Option<&str>) {
        self.log_error(error, context);
        self.capture_error(error, context);
    }

    fn report_storage_error(&self, error: &StorageError, context: Option<&str>) {
        self.log_error(error, context);
        self.capture_error(error, context);
    }

    fn report_consensus_error(&self, error: &ConsensusError, context: Option<&str>) {
        self.log_error(error, context);
        self.capture_error(error, context);
    }

    fn report_smart_contract_error(&self, error: &SmartContractError, context: Option<&str>) {
        self.log_error(error, context);
        self.capture_error(error, context);
    }

    fn report_wallet_error(&self, error: &WalletError, context: Option<&str>) {
        self.log_error(error, context);
        self.capture_error(error, context);
    }

    fn report_configuration_error(&self, error: &ConfigurationError, context: Option<&str>) {
        self.log_error(error, context);
        self.capture_error(error, context);
    }

    fn report_io_error(&self, error: &std::io::Error, context: Option<&str>) {
        self.log_error(error, context);
        self.capture_error(error, context);
    }

    fn report_serialization_error(&self, error: &serde_json::Error, context: Option<&str>) {
        self.log_error(error, context);
        self.capture_error(error, context);
    }

    fn report_invalid_data_error(&self, error: &str, context: Option<&str>) {
        self.log_error(error, context);
        self.capture_error(error, context);
    }

    fn report_permission_denied_error(&self, error: &str, context: Option<&str>) {
        self.log_error(error, context);
        self.capture_error(error, context);
    }

    fn report_resource_not_found_error(&self, error: &str, context: Option<&str>) {
        self.log_error(error, context);
        self.capture_error(error, context);
    }

    fn report_unexpected_error(&self, error: &str, context: Option<&str>) {
        self.log_error(error, context);
        self.capture_error(error, context);
    }

    fn log_error<E: fmt::Display>(&self, error: E, context: Option<&str>) {
        let context_msg = context.map(|c| format!(" in context: {}", c)).unwrap_or_default();
        error!("Error occurred{}: {}", context_msg, error);
    }

    fn capture_error<E: fmt::Debug + fmt::Display>(&self, error: E, context: Option<&str>) {
        if self.sentry_dsn.is_some() {
            configure_scope(|scope| {
                if let Some(context) = context {
                    scope.set_tag("context", context);
                }
                scope.set_extra("error_debug", format!("{:?}", error).into());
            });

            let breadcrumb = Breadcrumb {
                message: Some(format!("{}", error)),
                level: Level::Error,
                ..Default::default()
            };
            sentry::add_breadcrumb(breadcrumb);

            capture_error(&error);
        }
    }
}