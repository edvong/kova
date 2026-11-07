use crate::types::Postgres;
use kova_kernel::dsl::Operand;

pub enum ExtOperator<'a> {
    // Terminal operators that directly produce boolean conditions.
    JsonContains(Operand<'a, Postgres>, Operand<'a, Postgres>), // @>
    JsonContainedBy(Operand<'a, Postgres>, Operand<'a, Postgres>), // <@
    JsonHasKey(Operand<'a, Postgres>, Operand<'a, Postgres>),   // ?
    JsonHasAnyKeys(Operand<'a, Postgres>, Operand<'a, Postgres>), // ?|
    JsonHasAllKeys(Operand<'a, Postgres>, Operand<'a, Postgres>), // ?&

    // JSONPath Advanced Matching Operators
    JsonPathExists(Operand<'a, Postgres>, Operand<'a, Postgres>), // @? (Check if JSONPath returns any data)
    JsonPathPredicate(Operand<'a, Postgres>, Operand<'a, Postgres>), // @@ (Determine the JSONPath result as a boolean value)

    // Text Retrieval Boolean Operators
    ILike(Operand<'a, Postgres>, Operand<'a, Postgres>), // ILIKE
    NotILike(Operand<'a, Postgres>, Operand<'a, Postgres>), // NOT ILIKE
    Matches(Operand<'a, Postgres>, Operand<'a, Postgres>), // ~ (regular match)
    NotMatches(Operand<'a, Postgres>, Operand<'a, Postgres>), // !~
    IMatches(Operand<'a, Postgres>, Operand<'a, Postgres>), // ~* (case-insensitive regular)
    NotIMatches(Operand<'a, Postgres>, Operand<'a, Postgres>), // !~*

    // Array Contains and Overlap
    ArrayContains(Operand<'a, Postgres>, Operand<'a, Postgres>), // @>
    ArrayContainedBy(Operand<'a, Postgres>, Operand<'a, Postgres>), // <@
    ArrayOverlap(Operand<'a, Postgres>, Operand<'a, Postgres>),  // &&
}

// Tiling nodes in chain operations
pub enum JsonOp<'a> {
    Path(Operand<'a, Postgres>),          // ->
    PathText(Operand<'a, Postgres>),      // ->>
    PathArray(Operand<'a, Postgres>),     // #>
    PathTextArray(Operand<'a, Postgres>), // #>>
    Concat(Operand<'a, Postgres>),        // ||
    Delete(Operand<'a, Postgres>),        // -
    DeletePath(Operand<'a, Postgres>),    // #-
}
