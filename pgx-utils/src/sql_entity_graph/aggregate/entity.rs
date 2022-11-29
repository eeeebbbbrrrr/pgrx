/*
Portions Copyright 2019-2021 ZomboDB, LLC.
Portions Copyright 2021-2022 Technology Concepts & Design, Inc. <support@tcdi.com>

All rights reserved.

Use of this source code is governed by the MIT license that can be found in the LICENSE file.
*/
/*!

`#[pg_aggregate]` related entities for Rust to SQL translation

> Like all of the [`sql_entity_graph`][crate::sql_entity_graph] APIs, this is considered **internal**
to the `pgx` framework and very subject to change between versions. While you may use this, please do it with caution.


*/
use crate::sql_entity_graph::aggregate::options::{FinalizeModify, ParallelOption};
use crate::sql_entity_graph::metadata::SqlMapping;
use crate::sql_entity_graph::pgx_sql::PgxSql;
use crate::sql_entity_graph::to_sql::entity::ToSqlConfigEntity;
use crate::sql_entity_graph::to_sql::ToSql;
use crate::sql_entity_graph::{SqlGraphEntity, SqlGraphIdentifier, TyId, UsedTypeEntity};

use eyre::{eyre, WrapErr};

#[derive(Debug, Clone, Hash, PartialEq, Eq, PartialOrd, Ord, serde::Serialize, serde::Deserialize)]
pub struct AggregateTypeEntity {
    pub used_ty: UsedTypeEntity,
    pub name: Option<String>,
}

#[derive(Debug, Clone, Hash, PartialEq, Eq, PartialOrd, Ord, serde::Serialize, serde::Deserialize)]
pub struct PgAggregateEntity {
    pub full_path: String,
    pub module_path: String,
    pub file: String,
    pub line: u32,
    pub ty_id: TyId,

    pub name: String,

    /// If the aggregate is an ordered set aggregate.
    ///
    /// See [the PostgreSQL ordered set docs](https://www.postgresql.org/docs/current/xaggr.html#XAGGR-ORDERED-SET-AGGREGATES).
    pub ordered_set: bool,

    /// The `arg_data_type` list.
    ///
    /// Corresponds to `Args` in [`pgx::aggregate::Aggregate`].
    pub args: Vec<AggregateTypeEntity>,

    /// The direct argument list, appearing before `ORDER BY` in ordered set aggregates.
    ///
    /// Corresponds to `OrderBy` in [`pgx::aggregate::Aggregate`].
    pub direct_args: Option<Vec<AggregateTypeEntity>>,

    /// The `STYPE` and `name` parameter for [`CREATE AGGREGATE`](https://www.postgresql.org/docs/current/sql-createaggregate.html)
    ///
    /// The implementor of an [`pgx::aggregate::Aggregate`].
    pub stype: AggregateTypeEntity,

    /// The `SFUNC` parameter for [`CREATE AGGREGATE`](https://www.postgresql.org/docs/current/sql-createaggregate.html)
    ///
    /// Corresponds to `state` in [`pgx::aggregate::Aggregate`].
    pub sfunc: String,

    /// The `FINALFUNC` parameter for [`CREATE AGGREGATE`](https://www.postgresql.org/docs/current/sql-createaggregate.html)
    ///
    /// Corresponds to `finalize` in [`pgx::aggregate::Aggregate`].
    pub finalfunc: Option<String>,

    /// The `FINALFUNC_MODIFY` parameter for [`CREATE AGGREGATE`](https://www.postgresql.org/docs/current/sql-createaggregate.html)
    ///
    /// Corresponds to `FINALIZE_MODIFY` in [`pgx::aggregate::Aggregate`].
    pub finalfunc_modify: Option<FinalizeModify>,

    /// The `COMBINEFUNC` parameter for [`CREATE AGGREGATE`](https://www.postgresql.org/docs/current/sql-createaggregate.html)
    ///
    /// Corresponds to `combine` in [`pgx::aggregate::Aggregate`].
    pub combinefunc: Option<String>,

    /// The `SERIALFUNC` parameter for [`CREATE AGGREGATE`](https://www.postgresql.org/docs/current/sql-createaggregate.html)
    ///
    /// Corresponds to `serial` in [`pgx::aggregate::Aggregate`].
    pub serialfunc: Option<String>,

    /// The `DESERIALFUNC` parameter for [`CREATE AGGREGATE`](https://www.postgresql.org/docs/current/sql-createaggregate.html)
    ///
    /// Corresponds to `deserial` in [`pgx::aggregate::Aggregate`].
    pub deserialfunc: Option<String>,

    /// The `INITCOND` parameter for [`CREATE AGGREGATE`](https://www.postgresql.org/docs/current/sql-createaggregate.html)
    ///
    /// Corresponds to `INITIAL_CONDITION` in [`pgx::aggregate::Aggregate`].
    pub initcond: Option<String>,

    /// The `MSFUNC` parameter for [`CREATE AGGREGATE`](https://www.postgresql.org/docs/current/sql-createaggregate.html)
    ///
    /// Corresponds to `moving_state` in [`pgx::aggregate::Aggregate`].
    pub msfunc: Option<String>,

    /// The `MINVFUNC` parameter for [`CREATE AGGREGATE`](https://www.postgresql.org/docs/current/sql-createaggregate.html)
    ///
    /// Corresponds to `moving_state_inverse` in [`pgx::aggregate::Aggregate`].
    pub minvfunc: Option<String>,

    /// The `MSTYPE` parameter for [`CREATE AGGREGATE`](https://www.postgresql.org/docs/current/sql-createaggregate.html)
    ///
    /// Corresponds to `MovingState` in [`pgx::aggregate::Aggregate`].
    pub mstype: Option<UsedTypeEntity>,

    // The `MSSPACE` parameter for [`CREATE AGGREGATE`](https://www.postgresql.org/docs/current/sql-createaggregate.html)
    //
    // TODO: Currently unused.
    // pub msspace: String,
    /// The `MFINALFUNC` parameter for [`CREATE AGGREGATE`](https://www.postgresql.org/docs/current/sql-createaggregate.html)
    ///
    /// Corresponds to `moving_state_finalize` in [`pgx::aggregate::Aggregate`].
    pub mfinalfunc: Option<String>,

    /// The `MFINALFUNC_MODIFY` parameter for [`CREATE AGGREGATE`](https://www.postgresql.org/docs/current/sql-createaggregate.html)
    ///
    /// Corresponds to `MOVING_FINALIZE_MODIFY` in [`pgx::aggregate::Aggregate`].
    pub mfinalfunc_modify: Option<FinalizeModify>,

    /// The `MINITCOND` parameter for [`CREATE AGGREGATE`](https://www.postgresql.org/docs/current/sql-createaggregate.html)
    ///
    /// Corresponds to `MOVING_INITIAL_CONDITION` in [`pgx::aggregate::Aggregate`].
    pub minitcond: Option<String>,

    /// The `SORTOP` parameter for [`CREATE AGGREGATE`](https://www.postgresql.org/docs/current/sql-createaggregate.html)
    ///
    /// Corresponds to `SORT_OPERATOR` in [`pgx::aggregate::Aggregate`].
    pub sortop: Option<String>,

    /// The `PARALLEL` parameter for [`CREATE AGGREGATE`](https://www.postgresql.org/docs/current/sql-createaggregate.html)
    ///
    /// Corresponds to `PARALLEL` in [`pgx::aggregate::Aggregate`].
    pub parallel: Option<ParallelOption>,

    /// The `HYPOTHETICAL` parameter for [`CREATE AGGREGATE`](https://www.postgresql.org/docs/current/sql-createaggregate.html)
    ///
    /// Corresponds to `hypothetical` in [`pgx::aggregate::Aggregate`].
    pub hypothetical: bool,
    pub to_sql_config: ToSqlConfigEntity,
}

impl From<PgAggregateEntity> for SqlGraphEntity {
    fn from(val: PgAggregateEntity) -> Self {
        SqlGraphEntity::Aggregate(val)
    }
}

impl SqlGraphIdentifier for PgAggregateEntity {
    fn dot_identifier(&self) -> String {
        format!("aggregate {}", self.full_path)
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

impl ToSql for PgAggregateEntity {
    #[tracing::instrument(level = "debug", err, skip(self, context), fields(identifier = %self.rust_identifier()))]
    fn to_sql(&self, context: &PgxSql) -> eyre::Result<String> {
        let self_index = context.aggregates[self];
        let mut optional_attributes = Vec::new();
        let schema = context.schema_prefix_for(&self_index);

        if let Some(value) = self.finalfunc.as_ref() {
            optional_attributes.push((
                format!("\tFINALFUNC = {}\"{}\"", schema, value),
                format!("/* {}::final */", self.full_path),
            ));
        }
        if let Some(value) = self.finalfunc_modify.as_ref() {
            optional_attributes.push((
                format!("\tFINALFUNC_MODIFY = {}", value.to_sql(context)?),
                format!("/* {}::FINALIZE_MODIFY */", self.full_path),
            ));
        }
        if let Some(value) = self.combinefunc.as_ref() {
            optional_attributes.push((
                format!("\tCOMBINEFUNC = {}\"{}\"", schema, value),
                format!("/* {}::combine */", self.full_path),
            ));
        }
        if let Some(value) = self.serialfunc.as_ref() {
            optional_attributes.push((
                format!("\tSERIALFUNC = {}\"{}\"", schema, value),
                format!("/* {}::serial */", self.full_path),
            ));
        }
        if let Some(value) = self.deserialfunc.as_ref() {
            optional_attributes.push((
                format!("\tDESERIALFUNC ={} \"{}\"", schema, value),
                format!("/* {}::deserial */", self.full_path),
            ));
        }
        if let Some(value) = self.initcond.as_ref() {
            optional_attributes.push((
                format!("\tINITCOND = '{}'", value),
                format!("/* {}::INITIAL_CONDITION */", self.full_path),
            ));
        }
        if let Some(value) = self.msfunc.as_ref() {
            optional_attributes.push((
                format!("\tMSFUNC = {}\"{}\"", schema, value),
                format!("/* {}::moving_state */", self.full_path),
            ));
        }
        if let Some(value) = self.minvfunc.as_ref() {
            optional_attributes.push((
                format!("\tMINVFUNC = {}\"{}\"", schema, value),
                format!("/* {}::moving_state_inverse */", self.full_path),
            ));
        }
        if let Some(value) = self.mfinalfunc.as_ref() {
            optional_attributes.push((
                format!("\tMFINALFUNC = {}\"{}\"", schema, value),
                format!("/* {}::moving_state_finalize */", self.full_path),
            ));
        }
        if let Some(value) = self.mfinalfunc_modify {
            optional_attributes.push((
                format!("\tMFINALFUNC_MODIFY = {}", value.to_sql(context)?),
                format!("/* {}::MOVING_FINALIZE_MODIFY */", self.full_path),
            ));
        }
        if let Some(value) = self.minitcond.as_ref() {
            optional_attributes.push((
                format!("\tMINITCOND = '{}'", value),
                format!("/* {}::MOVING_INITIAL_CONDITION */", self.full_path),
            ));
        }
        if let Some(value) = self.sortop.as_ref() {
            optional_attributes.push((
                format!("\tSORTOP = \"{}\"", value),
                format!("/* {}::SORT_OPERATOR */", self.full_path),
            ));
        }
        if let Some(value) = self.parallel {
            optional_attributes.push((
                format!("\tPARALLEL = {}", value.to_sql(context)?),
                format!("/* {}::PARALLEL */", self.full_path),
            ));
        }
        if self.hypothetical {
            optional_attributes.push((
                String::from("\tHYPOTHETICAL"),
                format!("/* {}::hypothetical */", self.full_path),
            ))
        }

        let map_ty = |used_ty: &UsedTypeEntity| -> eyre::Result<String> {
            match used_ty.metadata.argument_sql {
                Ok(SqlMapping::As(ref argument_sql)) => Ok(argument_sql.to_string()),
                Ok(SqlMapping::Composite { array_brackets }) => used_ty
                    .composite_type
                    .as_ref()
                    .map(|v| if array_brackets { format!("{v}[]") } else { format!("{v}") })
                    .ok_or_else(|| {
                        eyre!("Macro expansion time suggested a composite_type!() in return")
                    }),
                Ok(SqlMapping::Source { array_brackets }) => {
                    let sql = context
                        .source_only_to_sql_type(&used_ty.ty_source)
                        .map(|v| if array_brackets { format!("{v}[]") } else { format!("{v}") })
                        .ok_or_else(|| {
                            eyre!("Macro expansion time suggested a source only mapping in return")
                        })?;
                    Ok(sql)
                }
                Ok(SqlMapping::Skip) => {
                    Err(eyre!("Cannot use skipped SQL translatable type as aggregate const type"))
                }
                Err(err) => match context.source_only_to_sql_type(&used_ty.ty_source) {
                    Some(source_only_mapping) => Ok(source_only_mapping.to_string()),
                    None => return Err(err).wrap_err("While mapping argument"),
                },
            }
        };

        let stype_sql = map_ty(&self.stype.used_ty).wrap_err("Mapping state type")?;

        if let Some(value) = &self.mstype {
            let mstype_sql = map_ty(&value).wrap_err("Mapping moving state type")?;
            optional_attributes.push((
                format!("\tMSTYPE = {}", mstype_sql),
                format!("/* {}::MovingState = {} */", self.full_path, value.full_path),
            ));
        }

        let mut optional_attributes_string = String::new();
        for (index, (optional_attribute, comment)) in optional_attributes.iter().enumerate() {
            let optional_attribute_string = format!(
                "{optional_attribute}{maybe_comma} {comment}{maybe_newline}",
                optional_attribute = optional_attribute,
                maybe_comma = if index == optional_attributes.len() - 1 { "" } else { "," },
                comment = comment,
                maybe_newline = if index == optional_attributes.len() - 1 { "" } else { "\n" }
            );
            optional_attributes_string += &optional_attribute_string;
        }

        let sql = format!(
            "\n\
                -- {file}:{line}\n\
                -- {full_path}\n\
                CREATE AGGREGATE {schema}{name} ({direct_args}{maybe_order_by}{args})\n\
                (\n\
                    \tSFUNC = {schema}\"{sfunc}\", /* {full_path}::state */\n\
                    \tSTYPE = {schema}{stype}{maybe_comma_after_stype} /* {stype_full_path} */\
                    {optional_attributes}\
                );\
            ",
            schema = schema,
            name = self.name,
            full_path = self.full_path,
            file = self.file,
            line = self.line,
            sfunc = self.sfunc,
            stype = stype_sql,
            stype_full_path = self.stype.used_ty.full_path,
            maybe_comma_after_stype = if optional_attributes.len() == 0 { "" } else { "," },
            args = {
                let mut args = Vec::new();
                for (idx, arg) in self.args.iter().enumerate() {
                    let graph_index = context
                        .graph
                        .neighbors_undirected(self_index)
                        .find(|neighbor| match &context.graph[*neighbor] {
                            SqlGraphEntity::Type(ty) => ty.id_matches(&arg.used_ty.ty_id),
                            SqlGraphEntity::Enum(en) => en.id_matches(&arg.used_ty.ty_id),
                            SqlGraphEntity::BuiltinType(defined) => {
                                defined == &arg.used_ty.full_path
                            }
                            _ => false,
                        })
                        .ok_or_else(|| {
                            eyre!("Could not find arg type in graph. Got: {:?}", arg.used_ty)
                        })?;
                    let needs_comma = idx < (self.args.len() - 1);
                    let buf = format!("\
                           \t{name}{variadic}{schema_prefix}{sql_type}{maybe_comma}/* {full_path} */\
                       ",
                           schema_prefix = context.schema_prefix_for(&graph_index),
                           // First try to match on [`TypeId`] since it's most reliable.
                           sql_type = match arg.used_ty.metadata.argument_sql {
                                Ok(SqlMapping::As(ref argument_sql)) => {
                                    argument_sql.to_string()
                                }
                                Ok(SqlMapping::Composite {
                                    array_brackets,
                                }) => {
                                    arg.used_ty
                                        .composite_type
                                        .as_ref()
                                        .map(|v| {
                                            if array_brackets {
                                                format!("{v}[]")
                                            } else {
                                                format!("{v}")
                                            }
                                        })
                                        .ok_or_else(|| {
                                            eyre!(
                                            "Macro expansion time suggested a composite_type!() in return"
                                        )
                                        })?
                                }
                                Ok(SqlMapping::Source {
                                    array_brackets,
                                }) => {
                                    let sql = context
                                        .source_only_to_sql_type(&arg.used_ty.ty_source)
                                        .map(|v| {
                                            if array_brackets {
                                                format!("{v}[]")
                                            } else {
                                                format!("{v}")
                                            }
                                        })
                                        .ok_or_else(|| {
                                            eyre!(
                                            "Macro expansion time suggested a source only mapping in return"
                                        )
                                        })?;
                                    sql
                                }
                                Ok(SqlMapping::Skip) => return Err(eyre!("Got a skipped SQL translatable type in aggregate args, this is not permitted")),
                                Err(err) => {
                                    match context.source_only_to_sql_type(&arg.used_ty.ty_source) {
                                        Some(source_only_mapping) => {
                                            source_only_mapping.to_string()
                                        }
                                        None => return Err(err).wrap_err("While mapping argument"),
                                    }
                                }
                            },
                           variadic = if arg.used_ty.variadic { "VARIADIC " } else { "" },
                           maybe_comma = if needs_comma { ", " } else { " " },
                           full_path = arg.used_ty.full_path,
                           name = if let Some(name) = arg.name.as_ref() {
                               format!(r#""{}" "#, name)
                           } else { "".to_string() },
                    );
                    args.push(buf);
                }
                "\n".to_string() + &args.join("\n") + "\n"
            },
            direct_args = if let Some(direct_args) = &self.direct_args {
                let mut args = Vec::new();
                for (idx, arg) in direct_args.iter().enumerate() {
                    let graph_index = context
                        .graph
                        .neighbors_undirected(self_index)
                        .find(|neighbor| match &context.graph[*neighbor] {
                            SqlGraphEntity::Type(ty) => ty.id_matches(&arg.used_ty.ty_id),
                            SqlGraphEntity::Enum(en) => en.id_matches(&arg.used_ty.ty_id),
                            SqlGraphEntity::BuiltinType(defined) => {
                                defined == &arg.used_ty.full_path
                            }
                            _ => false,
                        })
                        .ok_or_else(|| eyre!("Could not find arg type in graph. Got: {:?}", arg))?;
                    let needs_comma = idx < (direct_args.len() - 1);
                    let buf = format!(
                        "\
                        \t{maybe_name}{schema_prefix}{sql_type}{maybe_comma}/* {full_path} */\
                       ",
                        schema_prefix = context.schema_prefix_for(&graph_index),
                        // First try to match on [`TypeId`] since it's most reliable.
                        sql_type = map_ty(&arg.used_ty).wrap_err("Mapping direct arg type")?,
                        maybe_name = if let Some(name) = arg.name.as_ref() {
                            "\"".to_string() + name + "\" "
                        } else {
                            "".to_string()
                        },
                        maybe_comma = if needs_comma { ", " } else { " " },
                        full_path = arg.used_ty.full_path,
                    );
                    args.push(buf);
                }
                "\n".to_string() + &args.join("\n,") + "\n"
            } else {
                String::default()
            },
            maybe_order_by = if self.ordered_set { "\tORDER BY" } else { "" },
            optional_attributes = if optional_attributes.len() == 0 {
                String::from("\n")
            } else {
                String::from("\n")
            } + &optional_attributes_string
                + if optional_attributes.len() == 0 { "" } else { "\n" },
        );
        tracing::trace!(%sql);
        Ok(sql)
    }
}
