use crate::dsl::Component;
use crate::types::{Database, Extension};

pub type Pagination<'a, DB> = Component<
    (),
    <<DB as Database>::Extension<'a> as Extension<'a>>::Pagination
>;