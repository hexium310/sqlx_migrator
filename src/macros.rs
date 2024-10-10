/// Macro for vector of [`Box`]
#[macro_export]
macro_rules! vec_box {
    ($elem:expr; $n:expr) => (vec![Box::new($elem); $n]);
    ($($x:expr),*) => (vec![$(Box::new($x)),*]);
    ($($x:expr,)*) => (vec![$(Box::new($x)),*]);
    ($($x:expr,)*) => (sqlx_migrator::vec_box![$($x),*]);
}

/// Macro for implementing the `Migration` trait for the provided database.
///
/// This macro will use current file name as name for migration
///
/// This macro expects the following arguments:
/// - `$db:ty`: the type of database
/// - `$op:ty`: The type for which the migration is being implemented
/// - `$app_name:expr`: Name of app to be used for app variable
/// - `$parents:expr`: List of parents migration.
/// - `$operations:expr`: List of operations
#[macro_export]
macro_rules! migration {
    ($db:ty, $op:ty, $app_name:expr, $parents:expr, $operations:expr) => {
        #[async_trait::async_trait]
        impl sqlx_migrator::migration::Migration<$db> for $op {
            fn app(&self) -> &str {
                $app_name
            }

            fn name(&self) -> &str {
                std::path::Path::new(file!())
                    .file_stem()
                    .map(|stem_os_str| stem_os_str.to_str().unwrap_or_default())
                    .unwrap_or_default()
            }

            fn parents(&self) -> Vec<Box<dyn sqlx_migrator::migration::Migration<$db>>> {
                $parents
            }

            fn operations(&self) -> Vec<Box<dyn sqlx_migrator::operation::Operation<$db>>> {
                $operations
            }
        }
    };
}

/// Macro for implementing the [`migration`] macro for the `Any`.
///
/// This macro calls [`migration`] macro with db value already set asg
/// `sqlx::Any`
#[macro_export]
#[cfg(all(
    any(feature = "postgres", feature = "mysql", feature = "sqlite"),
    feature = "any"
))]
macro_rules! any_migration {
    ($op:ty, $app_name:expr, $parents:expr, $operations:expr) => {
        sqlx_migrator::migration!(sqlx::Any, $op, $app_name, $parents, $operations);
    };
}

/// Macro for implementing the [`migration`] macro for the `MySql`.
///
/// This macro calls [`migration`] macro with db value already set asg
/// `sqlx::MySql`
#[macro_export]
#[cfg(feature = "mysql")]
macro_rules! mysql_migration {
    ($op:ty, $app_name:expr, $parents:expr, $operations:expr) => {
        sqlx_migrator::migration!(sqlx::MySql, $op, $app_name, $parents, $operations);
    };
}

/// Macro for implementing the [`migration`] macro for the `Postgres`.
///
/// This macro calls [`migration`] macro with db value already set asg
/// `sqlx::Postgres`
#[macro_export]
#[cfg(feature = "postgres")]
macro_rules! postgres_migration {
    ($op:ty, $app_name:expr, $parents:expr, $operations:expr) => {
        sqlx_migrator::migration!(sqlx::Postgres, $op, $app_name, $parents, $operations);
    };
}

/// Macro for implementing the [`migration`] macro for the `Sqlite`.
///
/// This macro calls [`migration`] macro with db value already set as
/// `sqlx::Sqlite`
#[macro_export]
#[cfg(feature = "sqlite")]
macro_rules! sqlite_migration {
    ($op:ty, $app_name:expr, $parents:expr, $operations:expr) => {
        sqlx_migrator::migration!(sqlx::Sqlite, $op, $app_name, $parents, $operations);
    };
}
