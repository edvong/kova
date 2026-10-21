#![allow(unreachable_code, dead_code, unused)]

use kova::Error;
use kova::codec::{Decode, Encode};
use kova::dsl::{Column, Table};
use kova::types::{ColumnMeta, Database, Engine, FromRow, Model, Relation, Row};

#[derive(Default, Clone)]
pub struct User {
    pub id: i32,
    pub name: String,
    pub bio: Option<String>,
    pub balance: f64,
    pub is_active: bool,
}

#[allow(non_upper_case_globals)]
pub const User: Table<User> = Table::new("user", "user");

impl Relation for User {
    const NAME: &'static str = "user";
}

impl<X> FromRow<Engine, X> for User {
    fn from_row(row: &<Engine as Database>::Row<'_>) -> Result<Self, Error> {
        Ok(Self {
            id: row.get_column::<_, X>("user.id", "id")?,
            name: row.get_column::<_, X>("user.name", "name")?,
            bio: row.get_column::<_, X>("user.bio", "bio")?,
            balance: row.get_column::<_, X>("user.balance", "balance")?,
            is_active: row.get_column::<_, X>("user.is_active", "is_active")?,
        })
    }
}

impl<X> Model<Engine, X> for User {
    const COLUMNS_META: &'static [ColumnMeta] = &[
        ColumnMeta::new("user", "id", true),
        ColumnMeta::new("user", "name", false),
        ColumnMeta::new("user", "bio", false),
        ColumnMeta::new("user", "balance", false),
        ColumnMeta::new("user", "is_active", false),
    ];

    fn to_values<'a>(
        &'a self,
    ) -> Result<Vec<(&'static str, <Engine as Database>::Value<'a>)>, Error> {
        Ok(vec![
            ("id", Encode::<'a, X>::encode(&self.id)?),
            ("name", Encode::<'a, X>::encode(&self.name)?),
            ("bio", Encode::<'a, X>::encode(&self.bio)?),
            ("balance", Encode::<'a, X>::encode(&self.balance)?),
            ("is_active", Encode::<'a, X>::encode(&self.is_active)?),
        ])
    }
}

#[allow(non_upper_case_globals)]
impl User {
    pub const id: Column<i32> = Column::new("user", "id", true);
    pub const name: Column<String> = Column::new("user", "name", false);
    pub const bio: Column<Option<String>> = Column::new("user", "bio", false);
    pub const balance: Column<f64> = Column::new("user", "balance", false);
    pub const is_active: Column<bool> = Column::new("user", "is_active", false);
}