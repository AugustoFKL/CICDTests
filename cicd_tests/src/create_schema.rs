use std::fmt;

/// `Postgresql` `CREATE SCHEMA` statement.
///
/// Check the [`Postgresql documentation`] for more information.
///
/// [`Postgresql documentation`]: https://www.postgresql.org/docs/14/sql-createschema.html

pub struct CreateSchema {
    if_not_exists: bool,
    schema_name:   SchemaName,
}

/// SchemaName is a wrapper for a schema name and an authorization name.
#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug,)]

pub enum SchemaName {
    /// Only the schema name is specified.
    Name { name: String, },
    /// Only the schema authorization is specified.
    Authorization { authorization: String, },
    /// Both the schema name and authorization are specified.
    SchemaNameAndAuthorization {
        name:          String,
        authorization: String,
    },
}

impl CreateSchema {
    pub fn new(
        if_not_exists: bool,
        schema_name: SchemaName,
    ) -> Self {
        Self {
            if_not_exists,
            schema_name,
        }
    }

    pub fn if_not_exists(&self,) -> bool { self.if_not_exists }

    pub fn schema_name(&self,) -> SchemaName { self.schema_name.clone() }
}

impl fmt::Display for CreateSchema {
    fn fmt(
        &self,
        f: &mut fmt::Formatter,
    ) -> fmt::Result {
        write!(
            f,
            "CREATE SCHEMA {if_not_exists}{schema_name}",
            if_not_exists = if self.if_not_exists {
                "IF NOT EXISTS "
            } else {
                ""
            },
            schema_name = self.schema_name,
        )
    }
}

impl fmt::Display for SchemaName {
    fn fmt(
        &self,
        f: &mut fmt::Formatter,
    ) -> fmt::Result {
        match self {
            SchemaName::Name {
                name,
            } => write!(f, "{name}"),
            SchemaName::Authorization {
                authorization,
            } => {
                write!(f, "AUTHORIZATION {authorization}")
            },
            SchemaName::SchemaNameAndAuthorization {
                name,
                authorization,
            } => {
                write!(f, "{name} AUTHORIZATION {authorization}")
            },
        }
    }
}
