#[macro_export]
#[doc(hidden)]
macro_rules! impl_function_into_arg {
    ($StructName:ident) => {
        impl<'a, T, DB: $crate::types::Database> $crate::dsl::IntoArg<$crate::dsl::Projection<'a, DB>>
            for $StructName<$crate::dsl::Column<'a, T, DB>>
        {
            #[inline(always)]
            fn into_arg(self) -> $crate::dsl::Projection<'a, DB> {
                $crate::dsl::Projection {
                    expr: $crate::dsl::ProjectionExpr::Function(
                        <Self as $crate::dsl::function::builder::FunctionBuilder<'a, DB>>::build_func(self)
                    ),
                    alias: None,
                }
            }
        }
    };
}

// ==========================================
// General Condition Builder (for all functions)
// ==========================================
#[macro_export]
#[doc(hidden)]
macro_rules! impl_condition_builder {
    ($StructName:ident) => {
        impl<'a, T, DB: $crate::types::Database> $crate::dsl::condition::ConditionBuilder<'a, DB>
            for $StructName<$crate::dsl::Column<'a, T, DB>>
        {
            #[inline(always)]
            fn build_operand(self) -> $crate::dsl::Operand<'a, DB> {
                $crate::dsl::Operand::Function(
                    <Self as $crate::dsl::function::builder::FunctionBuilder<'a, DB>>::build_func(self)
                )
            }
        }
    };
}

// ==========================================
// Scalar function-specific builder
// ==========================================
#[macro_export]
#[doc(hidden)]
macro_rules! impl_scalar_builder {
    ($StructName:ident, $Variant:ident) => {
        impl<'a, T, DB: $crate::types::Database> $crate::dsl::function::builder::FunctionBuilder<'a, DB>
            for $StructName<$crate::dsl::Column<'a, T, DB>>
        {
            #[inline(always)]
            fn build_func(self) -> $crate::dsl::function::Function<'a, DB> {
                <
                    $crate::dsl::function::Function<'a, DB>
                    as $crate::dsl::WithComponent
                >::from_std(
                    $crate::dsl::function::StandardFunction::$Variant(self.0.meta())
                )
            }
        }
    };
}

// ==========================================
// Aggregate function-specific builder
// ==========================================
#[macro_export]
#[doc(hidden)]
macro_rules! impl_aggregate_builder {
    ($StructName:ident, $Variant:ident) => {
        impl<'a, T, DB: $crate::types::Database> $crate::dsl::function::builder::FunctionBuilder<'a, DB>
            for $StructName<$crate::dsl::Column<'a, T, DB>>
        {
            #[inline(always)]
            fn build_func(self) -> $crate::dsl::function::Function<'a, DB> {
                <Self as $crate::dsl::function::builder::AggregateBuilder<'a, DB>>::build_aggr(
                    self,
                    $crate::dsl::function::Quantifier::All
                )
            }
        }

        impl<'a, T, DB: $crate::types::Database> $crate::dsl::function::builder::AggregateBuilder<'a, DB>
            for $StructName<$crate::dsl::Column<'a, T, DB>>
        {
            #[inline(always)]
            fn build_aggr(self, quantifier: $crate::dsl::function::Quantifier) -> $crate::dsl::function::Function<'a, DB> {
                <
                    $crate::dsl::function::Function<'a, DB>
                    as $crate::dsl::WithComponent
                >::from_std(
                    $crate::dsl::function::StandardFunction::$Variant(self.0.meta(), quantifier)
                )
            }
        }
    };
}