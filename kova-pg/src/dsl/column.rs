use crate::dsl::assignment::Assignment;
use crate::types::Postgres;
use kova_core::dsl::WithComponent;
use kova_core::types::ColumnMeta;
use std::marker::PhantomData;

pub struct Column<T> {
    meta: ColumnMeta,
    _marker: PhantomData<T>
}

impl<T> Column<T> {

    #[doc(hidden)]
    pub const fn create<'a>(meta: ColumnMeta) -> kova_core::dsl::Column<'a, T, Postgres> {
        kova_core::dsl::Column::new(meta, Self::new(meta))
    }

    const fn new(meta: ColumnMeta) -> Column<T> {
        Self { meta, _marker: PhantomData }
    }
}

impl<T> Column<T> {

    pub fn excluded<'a, X>(&self) -> kova_core::dsl::Assignment<'a, Postgres, X> {
        kova_core::dsl::Assignment::from_ext(Assignment::ExcludedColumn(self.meta))
    }


    // JSONB operators
    // fn json_extract_text(key) -> Maps to PG `->>` operator.
    // fn json_contains(value) -> Maps to PG `@>` operator (contains).
    // fn json_has_key(key) -> Maps to PG `?` operator (has key).
    // Example: User::metadata.json_extract_text("avatar")

    // Array operators
    // fn any(value) -> Maps to PG `value = ANY(column)`.
    // fn contains_array(arr) -> Maps to PG `&&` (overlap) or `@>` (contains) operators.

    // String & Full-text search operators
    // fn ilike(value) -> Maps to PG `ILIKE` (case-insensitive LIKE).
    // fn matches(query) -> Maps to PG `@@` operator (full-text search).
}
