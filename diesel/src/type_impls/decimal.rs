#![allow(dead_code)]

#[cfg(feature = "bigdecimal")]
mod bigdecimal {
    extern crate bigdecimal;
    use self::bigdecimal::BigDecimal;
    use crate::deserialize::FromSqlRow;
    use crate::expression::AsExpression;
    use crate::sql_types::Numeric;

    #[derive(FromSqlRow, AsExpression)]
    #[diesel(foreign_derive)]
    #[sql_type = "Numeric"]
    struct BigDecimalProxy(BigDecimal);
}
