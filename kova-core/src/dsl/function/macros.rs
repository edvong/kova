#[macro_export]
macro_rules! impl_into_func_for_scalar {
    // For general standard scalar function
    ($name:ident) => {
        impl<E> $name<E> {
            #[inline(always)]
            pub fn into_func<'a, DB: $crate::types::Database, X>(self) -> $crate::dsl::func::Function<'a, DB>
                where
                    Self: $crate::dsl::OpNode,
                    E: $crate::dsl::IntoOperand<'a, <Self as $crate::dsl::OpNode>::Input, DB, $crate::dsl::OpAst, X> {
                let inner_op = self.0.into_operand();
                $crate::dsl::func::Function::from($crate::dsl::function::StandardFunction::$name(Box::new(inner_op)))
            }
        }
    };

    // For Dialect-specific extension functions
    // impl_into_func_for_scalar!(Json, Postgres, ExtFunction::Json)
    ($name:ident, $db:ty, $ext_path:path) => {
        impl<E> $name<E> {
            #[inline(always)]
            pub fn into_func<'a, X>(self) -> $crate::dsl::func::Function<'a, $db>
                where
                    Self: $crate::dsl::OpNode,
                    E: $crate::dsl::IntoOperand<'a, <Self as $crate::dsl::OpNode>::Input, $db, $crate::dsl::OpAst, X> {
                let inner_op = self.0.into_operand();
                $crate::dsl::func::Function::from($ext_path(Box::new(inner_op)))
            }
        }
    };
}

#[macro_export]
macro_rules! impl_into_func_for_agg {
    // For general standard agg functions
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
                $crate::dsl::func::Function::from($crate::dsl::function::StandardFunction::$name(Box::new(inner_op), quantifier))
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

    // For extension agg functions
    ($name:ident, $db:ty, $ext_path:path) => {
        impl<E> $name<E> {
            #[inline(always)]
            pub(crate) fn into_func_with<'a, X>(
                self, quantifier: $crate::dsl::function::Quantifier
            ) -> $crate::dsl::func::Function<'a, $db>
                where
                    Self: $crate::dsl::OpNode,
                    E: $crate::dsl::IntoOperand<'a, <Self as $crate::dsl::OpNode>::Input, $db, $crate::dsl::OpAst, X> {
                let inner_op = self.0.into_operand();
                $crate::dsl::func::Function::from($ext_path(Box::new(inner_op), quantifier))
            }

            #[inline(always)]
            pub(crate) fn into_func<'a, X>(self) -> $crate::dsl::func::Function<'a, $db>
                where
                    Self: $crate::dsl::OpNode,
                    E: $crate::dsl::IntoOperand<'a, <Self as $crate::dsl::OpNode>::Input, $db, $crate::dsl::OpAst, X> {
                self.into_func_with($crate::dsl::function::Quantifier::All)
            }
        }
    };
}

#[macro_export]
macro_rules! impl_base_traits_for_func {
    // For standard functions
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

        impl<'a, E, DB: $crate::types::Database, X> $crate::dsl::IntoProjection<'a, DB, X> for $name<E>
            where
                Self: $crate::dsl::OpNode,
                E: $crate::dsl::OpNode,
                E: $crate::dsl::IntoOperand<'a, <Self as $crate::dsl::OpNode>::Input, DB, $crate::dsl::OpAst, X> {
            #[inline(always)]
            fn into_projection(self) -> $crate::dsl::Projection<'a, DB, X> {
                $crate::dsl::Projection::new(self.into_operand())
            }
        }

        impl<'a, DB: $crate::types::Database, E, X> $crate::dsl::IntoOrdering<'a, DB, X> for $name<E>
            where
                Self: $crate::dsl::OpNode,
                E: $crate::dsl::OpNode,
                E: $crate::dsl::IntoOperand<'a, <Self as $crate::dsl::OpNode>::Input, DB, $crate::dsl::OpAst, X> {
            fn into_ordering(self) -> $crate::dsl::Ordering<'a, DB, X> {
                $crate::dsl::Ordering::from_operand(self.into_operand())
            }
        }

        impl<'a, DB: $crate::types::Database, E, X> $crate::dsl::IntoGroupBy<'a, DB, X> for $name<E>
            where
                Self: $crate::dsl::OpNode,
                E: $crate::dsl::OpNode,
                E: $crate::dsl::IntoOperand<'a, <Self as $crate::dsl::OpNode>::Input, DB, $crate::dsl::OpAst, X> {
            #[inline(always)]
            fn into_group_by(self) -> $crate::dsl::GroupBy<'a, DB, X> {
                $crate::dsl::GroupBy::new(self.into_operand())
            }
        }

        impl<'a, E, DB: $crate::types::Database, X> $crate::dsl::IntoArg<$crate::dsl::Projection<'a, DB, X>> for $name<E>
            where
                Self: $crate::dsl::OpNode,
                E: $crate::dsl::OpNode,
                E: $crate::dsl::IntoOperand<'a, <Self as $crate::dsl::OpNode>::Input, DB, $crate::dsl::OpAst, X> {
            #[inline(always)]
            fn into_arg(self) -> $crate::dsl::Projection<'a, DB, X> {
                $crate::dsl::IntoProjection::<DB, X>::into_projection(self)
            }
        }

        impl<'a, E, DB: $crate::types::Database, X> $crate::dsl::IntoArg<$crate::dsl::GroupBy<'a, DB, X>> for $name<E>
            where
                Self: $crate::dsl::OpNode,
                E: $crate::dsl::OpNode,
                E: $crate::dsl::IntoOperand<'a, <Self as $crate::dsl::OpNode>::Input, DB, $crate::dsl::OpAst, X> {
            #[inline(always)]
            fn into_arg(self) -> $crate::dsl::GroupBy<'a, DB, X> {
                $crate::dsl::IntoGroupBy::<DB, X>::into_group_by(self)
            }
        }
    };

    // For extension functions
    ($name:ident, $db:ty) => {
        impl<E> $crate::dsl::SingleArgMarker for $name<E> {}

        impl<'a, E, X> $crate::dsl::IntoOperand<'a, <Self as $crate::dsl::OpNode>::Output, $db, $crate::dsl::OpAst, X> for $name<E>
            where
                Self: $crate::dsl::OpNode,
                E: $crate::dsl::OpNode,
                E: $crate::dsl::IntoOperand<'a, <Self as $crate::dsl::OpNode>::Input, $db, $crate::dsl::OpAst, X> {
            #[inline(always)]
            fn into_operand(self) -> $crate::dsl::Operand<'a, $db> {
                $crate::dsl::Operand::from_function(self.into_func())
            }
        }

        impl<'a, E, X> $crate::dsl::IntoProjection<'a, $db, X> for $name<E>
            where
                Self: $crate::dsl::OpNode,
                E: $crate::dsl::OpNode,
                E: $crate::dsl::IntoOperand<'a, <Self as $crate::dsl::OpNode>::Input, $db, $crate::dsl::OpAst, X> {
            #[inline(always)]
            fn into_projection(self) -> $crate::dsl::Projection<'a, $db, X> {
                $crate::dsl::Projection::new(self.into_operand())
            }
        }

        impl<'a, E, X> $crate::dsl::IntoOrdering<'a, $db, X> for $name<E>
            where
                Self: $crate::dsl::OpNode,
                E: $crate::dsl::OpNode,
                E: $crate::dsl::IntoOperand<'a, <Self as $crate::dsl::OpNode>::Input, $db, $crate::dsl::OpAst, X> {
            fn into_ordering(self) -> $crate::dsl::Ordering<'a, $db, X> {
                $crate::dsl::Ordering::from_operand(self.into_operand())
            }
        }

        impl<'a, E, X> $crate::dsl::IntoGroupBy<'a, $db, X> for $name<E>
            where
                Self: $crate::dsl::OpNode,
                E: $crate::dsl::OpNode,
                E: $crate::dsl::IntoOperand<'a, <Self as $crate::dsl::OpNode>::Input, $db, $crate::dsl::OpAst, X> {
            #[inline(always)]
            fn into_group_by(self) -> $crate::dsl::GroupBy<'a, $db, X> {
                $crate::dsl::GroupBy::new(self.into_operand())
            }
        }

        impl<'a, E, X> $crate::dsl::IntoArg<$crate::dsl::Projection<'a, $db, X>> for $name<E>
            where
                Self: $crate::dsl::OpNode,
                E: $crate::dsl::OpNode,
                E: $crate::dsl::IntoOperand<'a, <Self as $crate::dsl::OpNode>::Input, $db, $crate::dsl::OpAst, X> {
            #[inline(always)]
            fn into_arg(self) -> $crate::dsl::Projection<'a, $db, X> {
                $crate::dsl::IntoProjection::<$db, X>::into_projection(self)
            }
        }

        impl<'a, E, X> $crate::dsl::IntoArg<$crate::dsl::GroupBy<'a, $db, X>> for $name<E>
            where
                Self: $crate::dsl::OpNode,
                E: $crate::dsl::OpNode,
                E: $crate::dsl::IntoOperand<'a, <Self as $crate::dsl::OpNode>::Input, $db, $crate::dsl::OpAst, X> {
            #[inline(always)]
            fn into_arg(self) -> $crate::dsl::GroupBy<'a, $db, X> {
                $crate::dsl::IntoGroupBy::<$db, X>::into_group_by(self)
            }
        }
    };
}