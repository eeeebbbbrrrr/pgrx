/*
Portions Copyright 2019-2021 ZomboDB, LLC.
Portions Copyright 2021-2022 Technology Concepts & Design, Inc. <support@tcdi.com>

All rights reserved.

Use of this source code is governed by the MIT license that can be found in the LICENSE file.
*/
/*!

`#[derive(PostgresHash)]` related entities for Rust to SQL translation

> Like all of the [`sql_entity_graph`][crate::sql_entity_graph] APIs, this is considered **internal**
to the `pgx` framework and very subject to change between versions. While you may use this, please do it with caution.

*/
use crate::sql_entity_graph::pgx_sql::PgxSql;
use crate::sql_entity_graph::to_sql::entity::ToSqlConfigEntity;
use crate::sql_entity_graph::to_sql::ToSql;
use crate::sql_entity_graph::{SqlGraphEntity, SqlGraphIdentifier, TyId};

/// The output of a [`PostgresHash`](crate::sql_entity_graph::postgres_hash::PostgresHash) from `quote::ToTokens::to_tokens`.
#[derive(Debug, Clone, Hash, PartialEq, Eq, PartialOrd, Ord, serde::Serialize, serde::Deserialize)]
pub struct PostgresHashEntity {
    pub name: String,
    pub file: String,
    pub line: u32,
    pub full_path: String,
    pub module_path: String,
    pub id: TyId,
    pub to_sql_config: ToSqlConfigEntity,
}

impl PostgresHashEntity {
    pub(crate) fn fn_name(&self) -> String {
        format!("{}_hash", self.name.to_lowercase())
    }
}

impl From<PostgresHashEntity> for SqlGraphEntity {
    fn from(val: PostgresHashEntity) -> Self {
        SqlGraphEntity::Hash(val)
    }
}

impl SqlGraphIdentifier for PostgresHashEntity {
    fn dot_identifier(&self) -> String {
        format!("hash {}", self.full_path)
    }
    fn rust_identifier(&self) -> String {
        self.full_path.to_string()
    }

    fn file(&self) -> Option<&str> {
        Some(&self.file)
    }

    fn line(&self) -> Option<u32> {
        Some(self.line)
    }
}

impl ToSql for PostgresHashEntity {
    #[tracing::instrument(level = "debug", err, skip(self, _context), fields(identifier = %self.rust_identifier()))]
    fn to_sql(&self, _context: &PgxSql) -> eyre::Result<String> {
        let sql = format!("\n\
                            -- {file}:{line}\n\
                            -- {full_path}\n\
                            CREATE OPERATOR FAMILY {name}_hash_ops USING hash;\n\
                            CREATE OPERATOR CLASS {name}_hash_ops DEFAULT FOR TYPE {name} USING hash FAMILY {name}_hash_ops AS\n\
                                \tOPERATOR    1   =  ({name}, {name}),\n\
                                \tFUNCTION    1   {fn_name}({name});\
                            ",
                          name = self.name,
                          full_path = self.full_path,
                          file = self.file,
                          line = self.line,
                          fn_name = self.fn_name(),
        );
        tracing::trace!(%sql);
        Ok(sql)
    }
}
