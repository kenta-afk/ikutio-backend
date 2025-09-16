use aws_sdk_dynamodb::error::SdkError;
use aws_sdk_dynamodb::operation::put_item::PutItemError;

use crate::internal::infrastructure::error::DbError;

impl From<SdkError<PutItemError>> for DbError {
    fn from(err: SdkError<PutItemError>) -> Self {
        match err {
            SdkError::ServiceError(service_err) => {
                let error = service_err.into_err();
                match error {
                    PutItemError::ConditionalCheckFailedException(details) => {
                        DbError::Conflict(details.message().unwrap_or("Conflict").to_string())
                    }
                    _ => DbError::Infrastructure(format!("Unknown service error: {error:?}")),
                }
            }
            _ => DbError::Infrastructure(format!("AWS SDK error: {err:?}")),
        }
    }
}
