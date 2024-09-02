//! The user deletion lives here

use rorm::transaction::Transaction;
use rorm::{delete, query, Database, Model};

use crate::models::User;

/**
Deletes a user in a transaction.
**Parameter**:
- `uuid`: Username of the user
- `db`: Reference of a [Database] instance
 */
pub async fn delete_user_transaction(
    username: String,
    db: &Database,
) -> Result<(), DeleteUserError> {
    let mut tx = db.start_transaction().await?;

    delete_user(username, &mut tx).await?;

    tx.commit().await?;

    Ok(())
}

/**
Deletes a user
**Parameter**:
- `username`: Username of the user
- `tx`: A mutable reference to a [Transaction]
 */
pub async fn delete_user(username: String, tx: &mut Transaction) -> Result<(), DeleteUserError> {
    query!(&mut *tx, (User::F.uuid,))
        .condition(User::F.username.equals(&username))
        .optional()
        .await?
        .ok_or(DeleteUserError::InvalidUsername)?;

    delete!(&mut *tx, User)
        .condition(User::F.username.equals(&username))
        .await?;

    Ok(())
}

/// Errors that can occur when deleting users
#[derive(Debug)]
pub enum DeleteUserError {
    /// Database error
    DatabaseError(rorm::Error),
    /// A user with that username does not exists
    InvalidUsername,
}

impl std::fmt::Display for DeleteUserError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            DeleteUserError::DatabaseError(err) => write!(f, "Database error {err}"),
            DeleteUserError::InvalidUsername => write!(f, "Invalid username"),
        }
    }
}

impl From<rorm::Error> for DeleteUserError {
    fn from(value: rorm::Error) -> Self {
        Self::DatabaseError(value)
    }
}
