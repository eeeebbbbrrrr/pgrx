/*!

Function and type level metadata entities for Rust to SQL translation

> Like all of the [`sql_entity_graph`][crate::sql_entity_graph] APIs, this is considered **internal**
to the `pgx` framework and very subject to change between versions. While you may use this, please do it with caution.


*/
use super::{ArgumentError, Returns, ReturnsError, SqlMapping};

#[derive(Clone, Debug, Hash, Eq, PartialEq, Ord, PartialOrd, serde::Serialize, serde::Deserialize)]
pub struct FunctionMetadataEntity {
    pub arguments: Vec<FunctionMetadataTypeEntity>,
    pub retval: Option<FunctionMetadataTypeEntity>,
    pub path: String,
}

#[derive(Clone, Debug, Hash, Eq, PartialEq, Ord, PartialOrd, serde::Serialize, serde::Deserialize)]
pub struct FunctionMetadataTypeEntity {
    pub type_name: String,
    pub argument_sql: Result<SqlMapping, ArgumentError>,
    pub return_sql: Result<Returns, ReturnsError>,
    pub variadic: bool,
    pub optional: bool,
}
