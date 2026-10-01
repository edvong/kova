#[macro_export]
macro_rules! impl_into_func_for_scalar {
    ($name:ident) => {
        impl<E> $name<E> {
            #[inline(always)]
            pub fn into_func<'a, DB: $crate::types::Database, X>(self) -> $crate::dsl::func::Function<'a, DB>
                where
                    Self: $crate::dsl::OpNode,
                    E: $crate::dsl::IntoOperand<'a, <Self as $crate::dsl::OpNode>::Input, DB, $crate::dsl::OpAst, X> {
                let inner_op = self.0.into_operand();
                $crate::dsl::WithComponent::from_std($crate::dsl::function::StandardFunction::$name(Box::new(inner_op)))
            }
        }
    };
}

#[macro_export]
macro_rules! impl_into_func_for_agg {
    ($name:ident) => {
        impl<E> $name<E> {
            #[inline(always)]
            pub(crate) fn into_func_with<'a, DB: $crate::types::Database, X>(
                self, quantifier: $crate::dsl::function::Quantifier
            ) -> $crate::dsl::func::Function<'a, DB>
                where
                    Self: $crate::dsl::OpNode,
                    E: $crate::dsl::IntoOperand<'a, <Self as $crate::dsl::OpNode>::Input, DB, $crate::dsl::OpAst, X> {
                let inner_op = self.0.into_operand();
                $crate::dsl::WithComponent::from_std($crate::dsl::function::StandardFunction::$name(Box::new(inner_op), quantifier))
            }

            #[inline(always)]
            pub(crate) fn into_func<'a, DB: $crate::types::Database, X>(self) -> $crate::dsl::func::Function<'a, DB>
                where
                    Self: $crate::dsl::OpNode,
                    E: $crate::dsl::IntoOperand<'a, <Self as $crate::dsl::OpNode>::Input, DB, $crate::dsl::OpAst, X> {
                self.into_func_with($crate::dsl::function::Quantifier::All)
            }
        }
    };
}

#[macro_export]
macro_rules! impl_base_traits_for_std_func {
    ($name:ident) => {
        impl<E> $crate::dsl::SingleArgMarker for $name<E> {}

        impl<'a, E, DB: $crate::types::Database, X> $crate::dsl::IntoOperand<'a, <Self as $crate::dsl::OpNode>::Output, DB, $crate::dsl::OpAst, X> for $name<E>
            where
                Self: $crate::dsl::OpNode,
                E: $crate::dsl::OpNode,
                E: $crate::dsl::IntoOperand<'a, <Self as $crate::dsl::OpNode>::Input, DB, $crate::dsl::OpAst, X> {
            #[inline(always)]
            fn into_operand(self) -> $crate::dsl::Operand<'a, DB> {
                $crate::dsl::Operand::from_function(self.into_func())
            }
        }

        impl<'a, E, DB: $crate::types::Database, X> $crate::dsl::BuildProjection<'a, DB, X> for $name<E>
            where
                Self: $crate::dsl::OpNode,
                E: $crate::dsl::OpNode,
                E: $crate::dsl::IntoOperand<'a, <Self as $crate::dsl::OpNode>::Input, DB, $crate::dsl::OpAst, X> {
            #[inline(always)]
            fn build_projection<A, U>(self, alias: A) -> $crate::dsl::Projection<'a, DB, X>
                where
                    A: $crate::dsl::IntoIdentifier<U> {
                $crate::dsl::Projection::new(
                    $crate::dsl::ProjectionExpr::Function(self.into_func()),
                    Some(alias.into_identifier().meta),
                )
            }
        }

        impl<'a, E, DB: $crate::types::Database, X> $crate::dsl::IntoArg<$crate::dsl::Projection<'a, DB, X>> for $name<E>
            where
                Self: $crate::dsl::OpNode,
                E: $crate::dsl::OpNode,
                E: $crate::dsl::IntoOperand<'a, <Self as $crate::dsl::OpNode>::Input, DB, $crate::dsl::OpAst, X> {
            #[inline(always)]
            fn into_arg(self) -> $crate::dsl::Projection<'a, DB, X> {
                $crate::dsl::Projection::new($crate::dsl::ProjectionExpr::Function(self.into_func()), None)
            }
        }

        impl<'a, E, DB: $crate::types::Database, X> $crate::dsl::BuildOrdering<'a, DB, X> for $name<E>
            where
                Self: $crate::dsl::OpNode,
                E: $crate::dsl::OpNode,
                E: $crate::dsl::IntoOperand<'a, <Self as $crate::dsl::OpNode>::Input, DB, $crate::dsl::OpAst, X> {
            #[inline(always)]
            fn build_ordering(self, direction: Option<$crate::dsl::Direction>) -> $crate::dsl::Ordering<'a, DB, X> {
                $crate::dsl::Ordering {
                    operand: $crate::dsl::WithComponent::from_std($crate::dsl::StandardOperand::Function(self.into_func())),
                    direction,
                    nulls: None,
                    _marker: std::marker::PhantomData,
                }
            }
        }

        impl<'a, E, DB: $crate::types::Database, X> $crate::dsl::IntoArg<$crate::dsl::GroupBy<'a, DB, X>> for $name<E>
            where
                Self: $crate::dsl::OpNode,
                E: $crate::dsl::OpNode,
                E: $crate::dsl::IntoOperand<'a, <Self as $crate::dsl::OpNode>::Input, DB, $crate::dsl::OpAst, X> {
            #[inline(always)]
            fn into_arg(self) -> $crate::dsl::GroupBy<'a, DB, X> {
                $crate::dsl::GroupBy::standard($crate::dsl::StandardGroupBy::Function(self.into_func()))
            }
        }
    };
}