mod dsl;
mod query;

use crate::Result;
use crate::dsl::func::{Quantifier, Window};
use crate::dsl::{Condition, OnConflictClause, Ordering, PaginationClause};
use crate::types::{Database, Extension, IdentifierMeta, TableMeta};
use std::any::TypeId;
use std::fmt::Write;

// Core non-destructive AST rendering protocol.
pub trait Render<'a, 's, DB: Database> {
    fn render(&'s self, ctx: &mut RenderContext<'a, 's, DB>) -> Result<()>;
}

// Platform-specific rendering protocol (bind placeholders, pagination, conflict clauses, SRFs).
pub trait PlatformRender<'a, 's, T>: Database {
    fn render(node: T, ctx: &mut RenderContext<'a, 's, Self>) -> Result<()>;
}

// Inverts `T: Render<'a, 's, DB>` into a supertrait bound `DB: RenderDispatch<'a, 's, T>`.
pub trait RenderDispatch<'a, 's, T>: Database {
    fn dispatch(node: &'s T, ctx: &mut RenderContext<'a, 's, Self>) -> Result<()>;
}

impl<'a, 's, DB: Database, T: Render<'a, 's, DB>> RenderDispatch<'a, 's, T> for DB {
    #[inline(always)]
    fn dispatch(node: &'s T, ctx: &mut RenderContext<'a, 's, Self>) -> Result<()> {
        node.render(ctx)
    }
}

// Unified platform rendering capability bound.
pub trait Renderable<'a, 's>:
    Database
    + PlatformRender<'a, 's, <Self as Database>::Value<'s>>
    + PlatformRender<'a, 's, &'s <Self as Database>::Value<'a>>
    + PlatformRender<'a, 's, &'s PaginationClause>
    + PlatformRender<'a, 's, &'s OnConflictClause<'a, Self>>
    + PlatformRender<
        'a,
        's,
        (&'s <<Self as Database>::Extension as Extension>::TableFunction<'a>, Option<TypeId>),
    > + RenderDispatch<'a, 's, <<Self as Database>::Extension as Extension>::Operator<'a>>
    + RenderDispatch<'a, 's, <<Self as Database>::Extension as Extension>::Function<'a>>
    + RenderDispatch<'a, 's, <<Self as Database>::Extension as Extension>::Assignment<'a>>
    + RenderDispatch<'a, 's, <<Self as Database>::Extension as Extension>::Locking>
    where
        Self: 'a,
        'a: 's {
}

impl<'a, 's, DB> Renderable<'a, 's> for DB
    where
        DB: Database
            + PlatformRender<'a, 's, <DB as Database>::Value<'s>>
            + PlatformRender<'a, 's, &'s <DB as Database>::Value<'a>>
            + PlatformRender<'a, 's, &'s PaginationClause>
            + PlatformRender<'a, 's, &'s OnConflictClause<'a, DB>>
            + PlatformRender<
                'a,
                's,
                (&'s <<DB as Database>::Extension as Extension>::TableFunction<'a>, Option<TypeId>),
            > + RenderDispatch<'a, 's, <<DB as Database>::Extension as Extension>::Operator<'a>>
            + RenderDispatch<'a, 's, <<DB as Database>::Extension as Extension>::Function<'a>>
            + RenderDispatch<'a, 's, <<DB as Database>::Extension as Extension>::Assignment<'a>>
            + RenderDispatch<'a, 's, <<DB as Database>::Extension as Extension>::Locking>
            + 'a,
        'a: 's {
}

const INLINE_TAGS: usize = 8;

// Stack-first tag registry supporting subquery scope isolation and outer query correlation.
pub struct TagRegistry {
    inline_ids: [TypeId; INLINE_TAGS],
    inline_idxs: [u32; INLINE_TAGS],
    len: usize,
    counter: u32,
    overflow: Vec<(TypeId, u32)>,
}

impl Default for TagRegistry {
    #[inline(always)]
    fn default() -> Self {
        Self {
            inline_ids: [TypeId::of::<()>(); INLINE_TAGS],
            inline_idxs: [0; INLINE_TAGS],
            len: 0,
            counter: 0,
            overflow: Vec::new(),
        }
    }
}

impl TagRegistry {
    #[inline(always)]
    pub fn enter_scope(&mut self) -> usize {
        self.len
    }

    #[inline(always)]
    pub fn exit_scope(&mut self, saved_len: usize) {
        if self.len > INLINE_TAGS {
            self.overflow.truncate(saved_len.saturating_sub(INLINE_TAGS));
        }
        self.len = saved_len;
    }

    #[inline(always)]
    pub fn register_in_current_scope(&mut self, scope_start: usize, raw_id: TypeId) -> usize {
        let inline_end = self.len.min(INLINE_TAGS);
        if scope_start < inline_end {
            for i in scope_start..inline_end {
                if self.inline_ids[i] == raw_id {
                    return self.inline_idxs[i] as usize;
                }
            }
        }
        if self.len > INLINE_TAGS {
            let ov_start = scope_start.saturating_sub(INLINE_TAGS);
            for &(id, idx) in &self.overflow[ov_start..] {
                if id == raw_id {
                    return idx as usize;
                }
            }
        }

        let idx = self.counter;
        self.counter += 1;
        if self.len < INLINE_TAGS {
            self.inline_ids[self.len] = raw_id;
            self.inline_idxs[self.len] = idx;
        } else {
            self.overflow.push((raw_id, idx));
        }
        self.len += 1;
        idx as usize
    }

    #[inline(always)]
    pub fn resolve(&mut self, raw_id: TypeId) -> usize {
        if self.len > INLINE_TAGS {
            for &(id, idx) in self.overflow.iter().rev() {
                if id == raw_id {
                    return idx as usize;
                }
            }
        }
        let inline_end = self.len.min(INLINE_TAGS);
        for i in (0..inline_end).rev() {
            if self.inline_ids[i] == raw_id {
                return self.inline_idxs[i] as usize;
            }
        }
        self.register_in_current_scope(self.len, raw_id)
    }
}

pub(crate) type AggModifiers<'a, 's, DB> = (
    Option<Quantifier>,
    &'s [Ordering<'a, DB, ()>],
    Option<&'s Condition<'a, DB, ()>>,
    Option<&'s Window<'a, DB, ()>>,
);

pub struct RenderContext<'a, 's, DB: Database> {
    pub sql: String,
    pub params: Vec<DB::Value<'s>>,
    pub(crate) tags: TagRegistry,
    pub(crate) agg_modifiers: Option<AggModifiers<'a, 's, DB>>,
    pub unqualify_columns: bool,
}

impl<'a, 's, DB: Database> Default for RenderContext<'a, 's, DB> {
    fn default() -> Self {
        Self::new()
    }
}

impl<'a, 's, DB: Database> RenderContext<'a, 's, DB> {
    #[inline(always)]
    pub fn new() -> Self {
        Self::with_capacity(128, 4)
    }

    #[inline(always)]
    pub fn with_capacity(sql_capacity: usize, params_capacity: usize) -> Self {
        Self {
            sql: String::with_capacity(sql_capacity),
            params: Vec::with_capacity(params_capacity),
            tags: TagRegistry::default(),
            agg_modifiers: None,
            unqualify_columns: false,
        }
    }

    #[inline(always)]
    pub fn into_data(self) -> RenderData<'s, DB> {
        RenderData { sql: self.sql, params: self.params }
    }

    #[inline(always)]
    pub fn push_ident(&mut self, name: &str) {
        self.sql.push('"');
        self.sql.push_str(name);
        self.sql.push('"');
    }

    #[inline(always)]
    pub fn push_single_quoted(&mut self, s: &str) {
        self.sql.push('\'');
        if !s.contains('\'') {
            self.sql.push_str(s);
        } else {
            for ch in s.chars() {
                if ch == '\'' {
                    self.sql.push_str("''");
                } else {
                    self.sql.push(ch);
                }
            }
        }
        self.sql.push('\'');
    }

    #[inline(always)]
    pub fn push_usize(&mut self, n: usize) {
        if n < 10 {
            self.sql.push((b'0' + n as u8) as char);
        } else {
            let _ = write!(self.sql, "{n}");
        }
    }

    #[inline(always)]
    pub fn push_i32(&mut self, n: i32) {
        let _ = write!(self.sql, "{n}");
    }

    #[inline(always)]
    pub fn push_table_ref(&mut self, meta: TableMeta) {
        self.sql.push('"');
        self.sql.push_str(meta.name);
        if let Some(raw_id) = meta.tag {
            let idx = self.tags.resolve(raw_id);
            self.sql.push('_');
            self.push_usize(idx);
        }
        self.sql.push('"');
    }

    #[inline(always)]
    pub fn push_table_def(&mut self, table: &'static str, tag: Option<TypeId>) {
        self.push_ident(table);
        if let Some(raw_id) = tag {
            let idx = self.tags.resolve(raw_id);
            self.sql.push_str(" as \"");
            self.sql.push_str(table);
            self.sql.push('_');
            self.push_usize(idx);
            self.sql.push('"');
        }
    }

    #[inline(always)]
    pub fn push_srf_alias(
        &mut self, func_name: &'static str, tag: Option<TypeId>, explicit_col: Option<&'static str>,
    ) {
        if let Some(raw_id) = tag {
            let idx = self.tags.resolve(raw_id);
            self.sql.push_str(" as \"");
            self.sql.push_str(func_name);
            self.sql.push('_');
            self.push_usize(idx);
            self.sql.push('"');
            if let Some(col) = explicit_col {
                self.sql.push('(');
                self.push_ident(col);
                self.sql.push(')');
            }
        }
    }

    #[inline(always)]
    pub fn push_column(&mut self, meta: IdentifierMeta) {
        if self.unqualify_columns || meta.table.name.is_empty() {
            self.push_ident(meta.name);
        } else {
            self.push_table_ref(meta.table);
            self.sql.push('.');
            self.push_ident(meta.name);
        }
    }

    #[inline(always)]
    pub fn render_agg<F>(&mut self, name: &str, f: F) -> Result<()>
        where
            'a: 's,
            DB: Renderable<'a, 's>,
            F: FnOnce(&mut Self) -> Result<()> {
        // Take modifiers immediately before invoking `f(self)` so inner expressions are never affected.
        let mods = self.agg_modifiers.take();
        self.sql.push_str(name);
        self.sql.push('(');
        if let Some((Some(q), _, _, _)) = mods {
            match q {
                Quantifier::Distinct => self.sql.push_str("distinct "),
                Quantifier::All => self.sql.push_str("all "),
            }
        }
        f(self)?;
        if let Some((_, order_by, _, _)) = mods {
            order_by.render(self)?;
        }
        self.sql.push(')');

        if let Some((_, _, filter, window)) = mods {
            if let Some(cond) = filter {
                self.sql.push_str(" filter (where ");
                cond.render(self)?;
                self.sql.push(')');
            }
            if let Some(win) = window {
                self.sql.push_str(" over (");
                win.render(self)?;
                self.sql.push(')');
            }
        }
        Ok(())
    }
}

pub struct RenderData<'a, DB: Database> {
    pub sql: String,
    pub params: Vec<DB::Value<'a>>,
}

impl<'a, DB: Database> Default for RenderData<'a, DB> {
    fn default() -> Self {
        Self { sql: String::new(), params: Vec::new() }
    }
}

impl<'a, DB: Database> RenderData<'a, DB> {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn with_capacity(sql_capacity: usize, params_capacity: usize) -> Self {
        Self {
            sql: String::with_capacity(sql_capacity),
            params: Vec::with_capacity(params_capacity),
        }
    }
}

impl<'a, 's, DB: Database> Render<'a, 's, DB> for () {
    #[inline(always)]
    fn render(&'s self, _: &mut RenderContext<'a, 's, DB>) -> Result<()> {
        Ok(())
    }
}
