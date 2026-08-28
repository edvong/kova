use crate::dsl::Function;
use crate::types::Postgres;
use kova_core::dsl::func::Function as CoreFunction;
use kova_core::dsl::{
    Column, Condition, GroupBy, IntoArg, IntoOperand, Operand, Operator, Ordering, Projection,
    ProjectionExpr, StandardOperator, WithComponent,
};
use kova_core::types::ColumnMeta;

/// Builder for the PostgreSQL `array_agg()` aggregate function.
///
/// Constructs an AST node that translates to variants of the `array_agg` SQL function,
/// fully supporting `DISTINCT`, intra-aggregate `ORDER BY`, and the `FILTER` clause.
pub struct ArrayAgg<'a> {
    meta: ColumnMeta,
    distinct: bool,
    order_by: Vec<Ordering<'a, Postgres>>,
    filter: Option<Condition<'a, Postgres>>,
}

// array_agg(x)
// array_agg(ALL x)
// array_agg(DISTINCT x)
// array_agg(x ORDER BY y)
// array_agg(DISTINCT x ORDER BY y)
// array_agg(x) FILTER (WHERE ...)
// array_agg(DISTINCT x ORDER BY y) FILTER (WHERE ...)

impl<'a> ArrayAgg<'a> {
    /// Initializes a new `array_agg` function call targeting the specified column.
    /// Translates to: `array_agg(column)`
    pub fn new<T>(target: Column<'a, T, Postgres>) -> Self {
        Self {
            meta: target.meta(),
            distinct: false,
            order_by: vec![],
            filter: None,
        }
    }

    /// Adds the `DISTINCT` keyword to the aggregate expression.
    /// Translates to: `array_agg(DISTINCT column)`
    pub fn distinct(mut self) -> Self {
        self.distinct = true;
        self
    }

    /// Appends an `ORDER BY` clause inside the aggregate function.
    /// Translates to: `array_agg(column ORDER BY ...)`
    pub fn order_by(mut self, ordering: Ordering<'a, Postgres>) -> Self {
        self.order_by.push(ordering);
        self
    }

    /// Attaches a `FILTER (WHERE ...)` clause to selectively aggregate rows.
    /// Translates to: `array_agg(...) FILTER (WHERE condition)`
    pub fn filter(mut self, condition: Condition<'a, Postgres>) -> Self {
        self.filter = Some(condition);
        self
    }
}

/// Comparison operators for the aggregate function.
/// These methods allow the `array_agg` expression to be used directly in
/// conditional clauses (e.g., `HAVING array_agg(id) = '{1, 2}'`).
impl<'a> ArrayAgg<'a> {
    /// Creates an equality (`=`) condition.
    pub fn eq<V>(&self, v: V) -> Condition<'a, Postgres>
    where
        V: IntoOperand<'a, V, Postgres>,
    {
        Condition::Op(Operator::from_std(StandardOperator::Eq(
            Operand::Column(self.meta),
            v.into_operand(),
        )))
    }

    /// Creates an inequality (`!=` or `<>`) condition.
    pub fn ne<V>(&self, v: V) -> Condition<'a, Postgres>
    where
        V: IntoOperand<'a, V, Postgres>,
    {
        Condition::Op(Operator::from_std(StandardOperator::Ne(
            Operand::Column(self.meta),
            v.into_operand(),
        )))
    }

    /// Creates a greater-than (`>`) condition.
    pub fn gt<V>(&self, v: V) -> Condition<'a, Postgres>
    where
        V: IntoOperand<'a, V, Postgres>,
    {
        Condition::Op(Operator::from_std(StandardOperator::Gt(
            Operand::Column(self.meta),
            v.into_operand(),
        )))
    }

    /// Creates a greater-than-or-equal-to (`>=`) condition.
    pub fn ge<V>(&self, v: V) -> Condition<'a, Postgres>
    where
        V: IntoOperand<'a, V, Postgres>,
    {
        Condition::Op(Operator::from_std(StandardOperator::Ge(
            Operand::Column(self.meta),
            v.into_operand(),
        )))
    }

    /// Creates a less-than (`<`) condition.
    pub fn lt<V>(&self, v: V) -> Condition<'a, Postgres>
    where
        V: IntoOperand<'a, V, Postgres>,
    {
        Condition::Op(Operator::from_std(StandardOperator::Lt(
            Operand::Column(self.meta),
            v.into_operand(),
        )))
    }

    /// Creates a less-than-or-equal-to (`<=`) condition.
    pub fn le<V>(&self, v: V) -> Condition<'a, Postgres>
    where
        V: IntoOperand<'a, V, Postgres>,
    {
        Condition::Op(Operator::from_std(StandardOperator::Le(
            Operand::Column(self.meta),
            v.into_operand(),
        )))
    }

    /// Creates an `IS NULL` condition.
    pub fn is_null(&self) -> Condition<'a, Postgres> {
        Condition::Op(Operator::from_std(StandardOperator::IsNull(
            Operand::Column(self.meta),
        )))
    }

    /// Creates an `IS NOT NULL` condition.
    pub fn is_not_null(&self) -> Condition<'a, Postgres> {
        Condition::Op(Operator::from_std(StandardOperator::IsNotNull(
            Operand::Column(self.meta),
        )))
    }

    /// Creates a `BETWEEN` condition.
    pub fn between<L, U>(&self, lower: L, upper: U) -> Condition<'a, Postgres>
    where
        L: IntoOperand<'a, L, Postgres>,
        U: IntoOperand<'a, U, Postgres>,
    {
        Condition::Op(Operator::from_std(StandardOperator::Between(
            Operand::Column(self.meta),
            lower.into_operand(),
            upper.into_operand(),
        )))
    }

    /// Creates a `NOT BETWEEN` condition.
    pub fn not_between<L, U>(&self, lower: L, upper: U) -> Condition<'a, Postgres>
    where
        L: IntoOperand<'a, L, Postgres>,
        U: IntoOperand<'a, U, Postgres>,
    {
        Condition::Op(Operator::from_std(StandardOperator::NotBetween(
            Operand::Column(self.meta),
            lower.into_operand(),
            upper.into_operand(),
        )))
    }

    /// Creates an `IN (...)` condition.
    pub fn in_list<I, V>(&self, values: I) -> Condition<'a, Postgres>
    where
        I: IntoIterator<Item = V>,
        V: IntoOperand<'a, V, Postgres>,
    {
        let values = values
            .into_iter()
            .map(|value| value.into_operand())
            .collect();
        Condition::Op(Operator::from_std(StandardOperator::In(
            Operand::Column(self.meta),
            values,
        )))
    }

    /// Creates a `NOT IN (...)` condition.
    pub fn not_in<I, V>(&self, values: I) -> Condition<'a, Postgres>
    where
        I: IntoIterator<Item = V>,
        V: IntoOperand<'a, V, Postgres>,
    {
        let values = values
            .into_iter()
            .map(|value| value.into_operand())
            .collect();
        Condition::Op(Operator::from_std(StandardOperator::NotIn(
            Operand::Column(self.meta),
            values,
        )))
    }
}

impl<'a> ArrayAgg<'a> {
    /// Finalizes the builder state, consuming the instance and transforming it
    /// into a Core AST `Function` node that the base query engine understands.
    pub fn into_func(self) -> CoreFunction<'a, Postgres> {
        CoreFunction::from_ext(Function::ArrayAgg(self))
    }

    /// Convenience wrapper: Converts the aggregate function directly into an aliased `Projection`.
    /// Similar in behavior to standard aggregate bindings (e.g., `Count::alias()`).
    pub fn alias<T>(self, col: Column<'a, T, Postgres>) -> Projection<'a, Postgres> {
        let func = self.into_func();
        Projection::new(ProjectionExpr::Function(func), Some(col.meta()))
    }
}

// ==========================================
// Trait Implementations for AST Integration
// ==========================================

/// Allows the dialect-specific function to be used dynamically as a general operand.
impl<'a> IntoOperand<'a, ArrayAgg<'a>, Postgres> for ArrayAgg<'a> {
    fn into_operand(self) -> Operand<'a, Postgres> {
        Operand::Function(self.into_func())
    }
}

/// Allows the dialect function to be passed directly into a `SELECT` clause (without an explicit alias).
impl<'a> IntoArg<Projection<'a, Postgres>> for ArrayAgg<'a> {
    fn into_arg(self) -> Projection<'a, Postgres> {
        Projection::new(ProjectionExpr::Function(self.into_func()), None)
    }
}

/// Allows the dialect function to be passed directly into an `ORDER BY` clause.
impl<'a> IntoArg<Ordering<'a, Postgres>> for ArrayAgg<'a> {
    fn into_arg(self) -> Ordering<'a, Postgres> {
        // Delegates directly to the CoreFunction's pre-existing IntoArg implementation.
        self.into_func().into_arg()
    }
}

/// Allows the dialect function to be passed directly into a `GROUP BY` clause.
impl<'a> IntoArg<GroupBy<'a, Postgres>> for ArrayAgg<'a> {
    fn into_arg(self) -> GroupBy<'a, Postgres> {
        // Delegates directly to the CoreFunction's pre-existing IntoArg implementation.
        self.into_func().into_arg()
    }
}
