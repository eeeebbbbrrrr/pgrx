//LICENSE Portions Copyright 2019-2021 ZomboDB, LLC.
//LICENSE
//LICENSE Portions Copyright 2021-2023 Technology Concepts & Design, Inc.
//LICENSE
//LICENSE Portions Copyright 2023-2023 PgCentral Foundation, Inc. <contact@pgcentral.org>
//LICENSE
//LICENSE All rights reserved.
//LICENSE
//LICENSE Use of this source code is governed by the MIT license that can be found in the LICENSE file. 
use crate::NotBuiltinOid;
#[derive(Copy, Clone, Eq, PartialEq, Hash, Ord, PartialOrd, Debug)]
pub enum BuiltinOid {
    ACLITEMARRAYOID = 1034,
    ACLITEMOID = 1033,
    ANYARRAYOID = 2277,
    ANYCOMPATIBLEARRAYOID = 5078,
    ANYCOMPATIBLENONARRAYOID = 5079,
    ANYCOMPATIBLEOID = 5077,
    ANYCOMPATIBLERANGEOID = 5080,
    ANYELEMENTOID = 2283,
    ANYENUMOID = 3500,
    ANYNONARRAYOID = 2776,
    ANYOID = 2276,
    ANYRANGEOID = 3831,
    AttributeRelationId = 1249,
    AuthIdRelationId = 1260,
    BITARRAYOID = 1561,
    BITOID = 1560,
    BOOLARRAYOID = 1000,
    BOOLOID = 16,
    BOXARRAYOID = 1020,
    BOXOID = 603,
    BPCHARARRAYOID = 1014,
    BPCHAROID = 1042,
    BYTEAARRAYOID = 1001,
    BYTEAOID = 17,
    CASHOID = 790,
    CHARARRAYOID = 1002,
    CHAROID = 18,
    CIDARRAYOID = 1012,
    CIDOID = 29,
    CIDRARRAYOID = 651,
    CIDROID = 650,
    CIRCLEARRAYOID = 719,
    CIRCLEOID = 718,
    CSTRINGARRAYOID = 1263,
    CSTRINGOID = 2275,
    DATEARRAYOID = 1182,
    DATEOID = 1082,
    DATERANGEARRAYOID = 3913,
    DATERANGEOID = 3912,
    DEFAULTTABLESPACE_OID = 1663,
    DatabaseRelationId = 1262,
    EVTTRIGGEROID = 3838,
    EnumRelationId = 3501,
    EventTriggerRelationId = 3466,
    ExtensionRelationId = 3079,
    FDW_HANDLEROID = 3115,
    FLOAT4ARRAYOID = 1021,
    FLOAT4OID = 700,
    FLOAT8ARRAYOID = 1022,
    FLOAT8OID = 701,
    GLOBALTABLESPACE_OID = 1664,
    GTSVECTORARRAYOID = 3644,
    GTSVECTOROID = 3642,
    HEAP_TABLE_AM_HANDLER_OID = 3,
    INDEX_AM_HANDLEROID = 325,
    INETARRAYOID = 1041,
    INETOID = 869,
    INT2ARRAYOID = 1005,
    INT2OID = 21,
    INT2VECTORARRAYOID = 1006,
    INT2VECTOROID = 22,
    INT4ARRAYOID = 1007,
    INT4OID = 23,
    INT4RANGEARRAYOID = 3905,
    INT4RANGEOID = 3904,
    INT8ARRAYOID = 1016,
    INT8OID = 20,
    INT8RANGEARRAYOID = 3927,
    INT8RANGEOID = 3926,
    INTERNALOID = 2281,
    INTERVALARRAYOID = 1187,
    INTERVALOID = 1186,
    IndexRelationId = 2610,
    JSONARRAYOID = 199,
    JSONBARRAYOID = 3807,
    JSONBOID = 3802,
    JSONOID = 114,
    JSONPATHARRAYOID = 4073,
    JSONPATHOID = 4072,
    LANGUAGE_HANDLEROID = 2280,
    LINEARRAYOID = 629,
    LINEOID = 628,
    LSEGARRAYOID = 1018,
    LSEGOID = 601,
    LSNOID = 3220,
    MACADDR8ARRAYOID = 775,
    MACADDR8OID = 774,
    MACADDRARRAYOID = 1040,
    MACADDROID = 829,
    MONEYARRAYOID = 791,
    NAMEARRAYOID = 1003,
    NAMEOID = 19,
    NUMERICARRAYOID = 1231,
    NUMERICOID = 1700,
    NUMRANGEARRAYOID = 3907,
    NUMRANGEOID = 3906,
    NamespaceRelationId = 2615,
    OIDARRAYOID = 1028,
    OIDOID = 26,
    OIDVECTORARRAYOID = 1013,
    OIDVECTOROID = 30,
    OperatorRelationId = 2617,
    PATHARRAYOID = 1019,
    PATHOID = 602,
    PGDDLCOMMANDOID = 32,
    PGDEPENDENCIESOID = 3402,
    PGMCVLISTOID = 5017,
    PGNDISTINCTOID = 3361,
    PGNODETREEOID = 194,
    PG_LSNARRAYOID = 3221,
    PG_SNAPSHOTARRAYOID = 5039,
    PG_SNAPSHOTOID = 5038,
    POINTARRAYOID = 1017,
    POINTOID = 600,
    POLYGONARRAYOID = 1027,
    POLYGONOID = 604,
    ProcedureRelationId = 1255,
    PublicationRelationId = 6104,
    RECORDARRAYOID = 2287,
    RECORDOID = 2249,
    REFCURSORARRAYOID = 2201,
    REFCURSOROID = 1790,
    REGCLASSARRAYOID = 2210,
    REGCLASSOID = 2205,
    REGCOLLATIONARRAYOID = 4192,
    REGCOLLATIONOID = 4191,
    REGCONFIGARRAYOID = 3735,
    REGCONFIGOID = 3734,
    REGDICTIONARYARRAYOID = 3770,
    REGDICTIONARYOID = 3769,
    REGNAMESPACEARRAYOID = 4090,
    REGNAMESPACEOID = 4089,
    REGOPERARRAYOID = 2208,
    REGOPERATORARRAYOID = 2209,
    REGOPERATOROID = 2204,
    REGOPEROID = 2203,
    REGPROCARRAYOID = 1008,
    REGPROCEDUREARRAYOID = 2207,
    REGPROCEDUREOID = 2202,
    REGPROCOID = 24,
    REGROLEARRAYOID = 4097,
    REGROLEOID = 4096,
    REGTYPEARRAYOID = 2211,
    REGTYPEOID = 2206,
    RelationRelationId = 1259,
    StatisticRelationId = 2619,
    TABLE_AM_HANDLEROID = 269,
    TEXTARRAYOID = 1009,
    TEXTOID = 25,
    TIDARRAYOID = 1010,
    TIDOID = 27,
    TIMEARRAYOID = 1183,
    TIMEOID = 1083,
    TIMESTAMPARRAYOID = 1115,
    TIMESTAMPOID = 1114,
    TIMESTAMPTZARRAYOID = 1185,
    TIMESTAMPTZOID = 1184,
    TIMETZARRAYOID = 1270,
    TIMETZOID = 1266,
    TRIGGEROID = 2279,
    TSM_HANDLEROID = 3310,
    TSQUERYARRAYOID = 3645,
    TSQUERYOID = 3615,
    TSRANGEARRAYOID = 3909,
    TSRANGEOID = 3908,
    TSTZRANGEARRAYOID = 3911,
    TSTZRANGEOID = 3910,
    TSVECTORARRAYOID = 3643,
    TSVECTOROID = 3614,
    TXID_SNAPSHOTARRAYOID = 2949,
    TXID_SNAPSHOTOID = 2970,
    TableSpaceRelationId = 1213,
    TemplateDbOid = 1,
    TriggerRelationId = 2620,
    TypeRelationId = 1247,
    UNKNOWNOID = 705,
    UUIDARRAYOID = 2951,
    UUIDOID = 2950,
    VARBITARRAYOID = 1563,
    VARBITOID = 1562,
    VARCHARARRAYOID = 1015,
    VARCHAROID = 1043,
    VOIDOID = 2278,
    XID8ARRAYOID = 271,
    XID8OID = 5069,
    XIDARRAYOID = 1011,
    XIDOID = 28,
    XMLARRAYOID = 143,
    XMLOID = 142,
}
impl BuiltinOid {
    pub const fn from_u32(uint: u32) -> Result<BuiltinOid, NotBuiltinOid> {
        match uint {
            0 => Err(NotBuiltinOid::Invalid),
            1034 => Ok(BuiltinOid::ACLITEMARRAYOID),
            1033 => Ok(BuiltinOid::ACLITEMOID),
            2277 => Ok(BuiltinOid::ANYARRAYOID),
            5078 => Ok(BuiltinOid::ANYCOMPATIBLEARRAYOID),
            5079 => Ok(BuiltinOid::ANYCOMPATIBLENONARRAYOID),
            5077 => Ok(BuiltinOid::ANYCOMPATIBLEOID),
            5080 => Ok(BuiltinOid::ANYCOMPATIBLERANGEOID),
            2283 => Ok(BuiltinOid::ANYELEMENTOID),
            3500 => Ok(BuiltinOid::ANYENUMOID),
            2776 => Ok(BuiltinOid::ANYNONARRAYOID),
            2276 => Ok(BuiltinOid::ANYOID),
            3831 => Ok(BuiltinOid::ANYRANGEOID),
            1249 => Ok(BuiltinOid::AttributeRelationId),
            1260 => Ok(BuiltinOid::AuthIdRelationId),
            1561 => Ok(BuiltinOid::BITARRAYOID),
            1560 => Ok(BuiltinOid::BITOID),
            1000 => Ok(BuiltinOid::BOOLARRAYOID),
            16 => Ok(BuiltinOid::BOOLOID),
            1020 => Ok(BuiltinOid::BOXARRAYOID),
            603 => Ok(BuiltinOid::BOXOID),
            1014 => Ok(BuiltinOid::BPCHARARRAYOID),
            1042 => Ok(BuiltinOid::BPCHAROID),
            1001 => Ok(BuiltinOid::BYTEAARRAYOID),
            17 => Ok(BuiltinOid::BYTEAOID),
            790 => Ok(BuiltinOid::CASHOID),
            1002 => Ok(BuiltinOid::CHARARRAYOID),
            18 => Ok(BuiltinOid::CHAROID),
            1012 => Ok(BuiltinOid::CIDARRAYOID),
            29 => Ok(BuiltinOid::CIDOID),
            651 => Ok(BuiltinOid::CIDRARRAYOID),
            650 => Ok(BuiltinOid::CIDROID),
            719 => Ok(BuiltinOid::CIRCLEARRAYOID),
            718 => Ok(BuiltinOid::CIRCLEOID),
            1263 => Ok(BuiltinOid::CSTRINGARRAYOID),
            2275 => Ok(BuiltinOid::CSTRINGOID),
            1182 => Ok(BuiltinOid::DATEARRAYOID),
            1082 => Ok(BuiltinOid::DATEOID),
            3913 => Ok(BuiltinOid::DATERANGEARRAYOID),
            3912 => Ok(BuiltinOid::DATERANGEOID),
            1663 => Ok(BuiltinOid::DEFAULTTABLESPACE_OID),
            1262 => Ok(BuiltinOid::DatabaseRelationId),
            3838 => Ok(BuiltinOid::EVTTRIGGEROID),
            3501 => Ok(BuiltinOid::EnumRelationId),
            3466 => Ok(BuiltinOid::EventTriggerRelationId),
            3079 => Ok(BuiltinOid::ExtensionRelationId),
            3115 => Ok(BuiltinOid::FDW_HANDLEROID),
            1021 => Ok(BuiltinOid::FLOAT4ARRAYOID),
            700 => Ok(BuiltinOid::FLOAT4OID),
            1022 => Ok(BuiltinOid::FLOAT8ARRAYOID),
            701 => Ok(BuiltinOid::FLOAT8OID),
            1664 => Ok(BuiltinOid::GLOBALTABLESPACE_OID),
            3644 => Ok(BuiltinOid::GTSVECTORARRAYOID),
            3642 => Ok(BuiltinOid::GTSVECTOROID),
            3 => Ok(BuiltinOid::HEAP_TABLE_AM_HANDLER_OID),
            325 => Ok(BuiltinOid::INDEX_AM_HANDLEROID),
            1041 => Ok(BuiltinOid::INETARRAYOID),
            869 => Ok(BuiltinOid::INETOID),
            1005 => Ok(BuiltinOid::INT2ARRAYOID),
            21 => Ok(BuiltinOid::INT2OID),
            1006 => Ok(BuiltinOid::INT2VECTORARRAYOID),
            22 => Ok(BuiltinOid::INT2VECTOROID),
            1007 => Ok(BuiltinOid::INT4ARRAYOID),
            23 => Ok(BuiltinOid::INT4OID),
            3905 => Ok(BuiltinOid::INT4RANGEARRAYOID),
            3904 => Ok(BuiltinOid::INT4RANGEOID),
            1016 => Ok(BuiltinOid::INT8ARRAYOID),
            20 => Ok(BuiltinOid::INT8OID),
            3927 => Ok(BuiltinOid::INT8RANGEARRAYOID),
            3926 => Ok(BuiltinOid::INT8RANGEOID),
            2281 => Ok(BuiltinOid::INTERNALOID),
            1187 => Ok(BuiltinOid::INTERVALARRAYOID),
            1186 => Ok(BuiltinOid::INTERVALOID),
            2610 => Ok(BuiltinOid::IndexRelationId),
            199 => Ok(BuiltinOid::JSONARRAYOID),
            3807 => Ok(BuiltinOid::JSONBARRAYOID),
            3802 => Ok(BuiltinOid::JSONBOID),
            114 => Ok(BuiltinOid::JSONOID),
            4073 => Ok(BuiltinOid::JSONPATHARRAYOID),
            4072 => Ok(BuiltinOid::JSONPATHOID),
            2280 => Ok(BuiltinOid::LANGUAGE_HANDLEROID),
            629 => Ok(BuiltinOid::LINEARRAYOID),
            628 => Ok(BuiltinOid::LINEOID),
            1018 => Ok(BuiltinOid::LSEGARRAYOID),
            601 => Ok(BuiltinOid::LSEGOID),
            3220 => Ok(BuiltinOid::LSNOID),
            775 => Ok(BuiltinOid::MACADDR8ARRAYOID),
            774 => Ok(BuiltinOid::MACADDR8OID),
            1040 => Ok(BuiltinOid::MACADDRARRAYOID),
            829 => Ok(BuiltinOid::MACADDROID),
            791 => Ok(BuiltinOid::MONEYARRAYOID),
            1003 => Ok(BuiltinOid::NAMEARRAYOID),
            19 => Ok(BuiltinOid::NAMEOID),
            1231 => Ok(BuiltinOid::NUMERICARRAYOID),
            1700 => Ok(BuiltinOid::NUMERICOID),
            3907 => Ok(BuiltinOid::NUMRANGEARRAYOID),
            3906 => Ok(BuiltinOid::NUMRANGEOID),
            2615 => Ok(BuiltinOid::NamespaceRelationId),
            1028 => Ok(BuiltinOid::OIDARRAYOID),
            26 => Ok(BuiltinOid::OIDOID),
            1013 => Ok(BuiltinOid::OIDVECTORARRAYOID),
            30 => Ok(BuiltinOid::OIDVECTOROID),
            2617 => Ok(BuiltinOid::OperatorRelationId),
            1019 => Ok(BuiltinOid::PATHARRAYOID),
            602 => Ok(BuiltinOid::PATHOID),
            32 => Ok(BuiltinOid::PGDDLCOMMANDOID),
            3402 => Ok(BuiltinOid::PGDEPENDENCIESOID),
            5017 => Ok(BuiltinOid::PGMCVLISTOID),
            3361 => Ok(BuiltinOid::PGNDISTINCTOID),
            194 => Ok(BuiltinOid::PGNODETREEOID),
            3221 => Ok(BuiltinOid::PG_LSNARRAYOID),
            5039 => Ok(BuiltinOid::PG_SNAPSHOTARRAYOID),
            5038 => Ok(BuiltinOid::PG_SNAPSHOTOID),
            1017 => Ok(BuiltinOid::POINTARRAYOID),
            600 => Ok(BuiltinOid::POINTOID),
            1027 => Ok(BuiltinOid::POLYGONARRAYOID),
            604 => Ok(BuiltinOid::POLYGONOID),
            1255 => Ok(BuiltinOid::ProcedureRelationId),
            6104 => Ok(BuiltinOid::PublicationRelationId),
            2287 => Ok(BuiltinOid::RECORDARRAYOID),
            2249 => Ok(BuiltinOid::RECORDOID),
            2201 => Ok(BuiltinOid::REFCURSORARRAYOID),
            1790 => Ok(BuiltinOid::REFCURSOROID),
            2210 => Ok(BuiltinOid::REGCLASSARRAYOID),
            2205 => Ok(BuiltinOid::REGCLASSOID),
            4192 => Ok(BuiltinOid::REGCOLLATIONARRAYOID),
            4191 => Ok(BuiltinOid::REGCOLLATIONOID),
            3735 => Ok(BuiltinOid::REGCONFIGARRAYOID),
            3734 => Ok(BuiltinOid::REGCONFIGOID),
            3770 => Ok(BuiltinOid::REGDICTIONARYARRAYOID),
            3769 => Ok(BuiltinOid::REGDICTIONARYOID),
            4090 => Ok(BuiltinOid::REGNAMESPACEARRAYOID),
            4089 => Ok(BuiltinOid::REGNAMESPACEOID),
            2208 => Ok(BuiltinOid::REGOPERARRAYOID),
            2209 => Ok(BuiltinOid::REGOPERATORARRAYOID),
            2204 => Ok(BuiltinOid::REGOPERATOROID),
            2203 => Ok(BuiltinOid::REGOPEROID),
            1008 => Ok(BuiltinOid::REGPROCARRAYOID),
            2207 => Ok(BuiltinOid::REGPROCEDUREARRAYOID),
            2202 => Ok(BuiltinOid::REGPROCEDUREOID),
            24 => Ok(BuiltinOid::REGPROCOID),
            4097 => Ok(BuiltinOid::REGROLEARRAYOID),
            4096 => Ok(BuiltinOid::REGROLEOID),
            2211 => Ok(BuiltinOid::REGTYPEARRAYOID),
            2206 => Ok(BuiltinOid::REGTYPEOID),
            1259 => Ok(BuiltinOid::RelationRelationId),
            2619 => Ok(BuiltinOid::StatisticRelationId),
            269 => Ok(BuiltinOid::TABLE_AM_HANDLEROID),
            1009 => Ok(BuiltinOid::TEXTARRAYOID),
            25 => Ok(BuiltinOid::TEXTOID),
            1010 => Ok(BuiltinOid::TIDARRAYOID),
            27 => Ok(BuiltinOid::TIDOID),
            1183 => Ok(BuiltinOid::TIMEARRAYOID),
            1083 => Ok(BuiltinOid::TIMEOID),
            1115 => Ok(BuiltinOid::TIMESTAMPARRAYOID),
            1114 => Ok(BuiltinOid::TIMESTAMPOID),
            1185 => Ok(BuiltinOid::TIMESTAMPTZARRAYOID),
            1184 => Ok(BuiltinOid::TIMESTAMPTZOID),
            1270 => Ok(BuiltinOid::TIMETZARRAYOID),
            1266 => Ok(BuiltinOid::TIMETZOID),
            2279 => Ok(BuiltinOid::TRIGGEROID),
            3310 => Ok(BuiltinOid::TSM_HANDLEROID),
            3645 => Ok(BuiltinOid::TSQUERYARRAYOID),
            3615 => Ok(BuiltinOid::TSQUERYOID),
            3909 => Ok(BuiltinOid::TSRANGEARRAYOID),
            3908 => Ok(BuiltinOid::TSRANGEOID),
            3911 => Ok(BuiltinOid::TSTZRANGEARRAYOID),
            3910 => Ok(BuiltinOid::TSTZRANGEOID),
            3643 => Ok(BuiltinOid::TSVECTORARRAYOID),
            3614 => Ok(BuiltinOid::TSVECTOROID),
            2949 => Ok(BuiltinOid::TXID_SNAPSHOTARRAYOID),
            2970 => Ok(BuiltinOid::TXID_SNAPSHOTOID),
            1213 => Ok(BuiltinOid::TableSpaceRelationId),
            1 => Ok(BuiltinOid::TemplateDbOid),
            2620 => Ok(BuiltinOid::TriggerRelationId),
            1247 => Ok(BuiltinOid::TypeRelationId),
            705 => Ok(BuiltinOid::UNKNOWNOID),
            2951 => Ok(BuiltinOid::UUIDARRAYOID),
            2950 => Ok(BuiltinOid::UUIDOID),
            1563 => Ok(BuiltinOid::VARBITARRAYOID),
            1562 => Ok(BuiltinOid::VARBITOID),
            1015 => Ok(BuiltinOid::VARCHARARRAYOID),
            1043 => Ok(BuiltinOid::VARCHAROID),
            2278 => Ok(BuiltinOid::VOIDOID),
            271 => Ok(BuiltinOid::XID8ARRAYOID),
            5069 => Ok(BuiltinOid::XID8OID),
            1011 => Ok(BuiltinOid::XIDARRAYOID),
            28 => Ok(BuiltinOid::XIDOID),
            143 => Ok(BuiltinOid::XMLARRAYOID),
            142 => Ok(BuiltinOid::XMLOID),
            _ => Err(NotBuiltinOid::Ambiguous),
        }
    }
}
