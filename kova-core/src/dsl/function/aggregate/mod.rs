#![allow(clippy::wrong_self_convention)]

mod count_all;
mod sum;
mod max;
mod min;
mod avg;
mod count;


#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Quantifier {
    All,
    Distinct,
}

pub use {
    count_all::CountAll,
    count::Count,
    sum::Sum,
    avg::Avg,
    max::Max,
    min::Min,
};


// #[doc(hidden)]
// #[macro_export]
// macro_rules! impl_aggregate_function {
//     ($StructName:ident, $Variant:ident) => {
//         impl<'a, T, DB: Database> $StructName<$crate::dsl::Column<'a, T, DB>> {
//             pub fn distinct(self) -> $crate::dsl::function::Function<'a, DB> {
//                 $crate::dsl::function::Function::from_std(
//                     $crate::dsl::function::StandardFunction::$Variant(
//                         self.0.meta,
//                         $crate::dsl::function::aggregate::Quantifier::Distinct,
//                     )
//                 )
//             }
// 
//             pub fn alias<U>(self, col: $crate::dsl::Column<'a, U, DB>) -> Projection<'a, DB> {
//                 let func = $crate::dsl::function::Function::from_std(
//                     $crate::dsl::function::StandardFunction::$Variant(
//                         self.0.meta, 
//                         $crate::dsl::function::aggregate::Quantifier::All
//                     )
//                 );
//                 $crate::dsl::clause::projection::Projection {
//                     expr: $crate::dsl::clause::projection::ProjectionExpr::Function(func),
//                     alias: Some(col.meta),
//                 }
//             }
// 
//             pub fn asc(self) -> $crate::dsl::clause::order::Ordering<'a, DB> {
//                 let func = $crate::dsl::function::Function::from_std(
//                     $crate::dsl::function::StandardFunction::$Variant(
//                         self.0.meta, 
//                         $crate::dsl::function::aggregate::Quantifier::All
//                     )
//                 );
//                 $crate::dsl::clause::order::Ordering {
//                     operand: $crate::dsl::clause::order::OrderOperand::from_std(
//                         $crate::dsl::function::StandardOperand::Function(func)
//                     ),
//                     direction: Some($crate::dsl::clause::order::Direction::Asc),
//                     nulls: None,
//                 }
//             }
// 
//             pub fn desc(self) -> $crate::dsl::clause::order::Ordering<'a, DB> {
//                 let func = $crate::dsl::function::Function::from_std(
//                     $crate::dsl::function::StandardFunction::$Variant(
//                         self.0.meta, 
//                         $crate::dsl::function::aggregate::Quantifier::All
//                     )
//                 );
//                 $crate::dsl::clause::order::Ordering {
//                     operand: $crate::dsl::clause::order::OrderOperand::from_std(
//                         $crate::dsl::function::StandardOperand::Function(func)
//                     ),
//                     direction: Some($crate::dsl::clause::order::Direction::Desc),
//                     nulls: None,
//                 }
//             }
//         }
// 
//         impl<'a, T, DB: Database> IntoArg<$crate::dsl::clause::projection::Projection<'a, DB>> for $StructName<$crate::dsl::Column<'a, T, DB>> {
//             fn into_arg(self) -> $crate::dsl::clause::projection::Projection<'a, DB> {
//                 let func =
//                     $crate::dsl::function::Function::from_std($crate::dsl::function::StandardFunction::$Variant(self.0.meta, $crate::dsl::function::aggregate::Quantifier::All));
//                 $crate::dsl::clause::projection::Projection {
//                     expr: $crate::dsl::clause::projection::ProjectionExpr::Function(func),
//                     alias: None,
//                 }
//             }
//         }
//     };
// }