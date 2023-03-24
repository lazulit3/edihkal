use sea_orm::{DbErr, RuntimeErr};

#[derive(thiserror::Error, Debug)]
#[non_exhaustive]
pub enum DatabaseError {
    #[error("Duplicate key violates unique constraint")]
    UniqueViolation,
    #[error("Unknown database error")]
    Unknown,
}

impl From<DbErr> for DatabaseError {
    fn from(error: DbErr) -> Self {
        match error {
            // TODO: Behavior for other databases
            // #[cfg(feature = "sea-orm-postgres")]
            // TODO: Check e.kind() for ErrorKind::UniqueViolation after sqlx 0.7 release:
            //       https://github.com/launchbadge/sqlx/pull/2109
            DbErr::Query(RuntimeErr::SqlxError(sqlx::Error::Database(e)))
                if e.code().unwrap().eq("23505") =>
            {
                Self::UniqueViolation
            }
            _ => Self::Unknown,
        }
    }
}
