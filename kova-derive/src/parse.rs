use heck::ToSnakeCase;
use quote::ToTokens;
use syn::ext::IdentExt;
use syn::{Attribute, Data, DeriveInput, Error, Field, Fields, Ident, Meta, Type, Visibility};

#[derive(Clone, Copy, PartialEq, Eq)]
pub(crate) enum DeriveRole {
    Model,
    FromRow,
    Cte,
}

impl DeriveRole {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Model => "Model",
            Self::FromRow => "FromRow",
            Self::Cte => "Cte",
        }
    }
}

pub(crate) struct StructDef {
    pub vis: Visibility,
    pub ident: Ident,
    pub table_name: String,
    pub fields: Vec<FieldDef>,
}

pub(crate) struct FieldDef {
    pub ident: Ident,
    pub ty: Type,
    pub col_name: String,
    pub is_auto: bool,
}

pub(crate) fn parse_struct(ast: &DeriveInput, role: DeriveRole) -> Result<StructDef, Error> {
    check_mutual_exclusivity(&ast.attrs, role)?;
    reject_container_kova_attrs(&ast.attrs)?;

    if !ast.generics.params.is_empty() {
        return Err(Error::new_spanned(
            &ast.generics,
            format!(
                "`#[derive({})]` does not support generic parameters on structs",
                role.as_str()
            ),
        ));
    }

    let data_struct = match &ast.data {
        Data::Struct(s) => s,
        _ => {
            return Err(Error::new_spanned(
                &ast.ident,
                format!("`#[derive({})]` can only be applied to structs", role.as_str()),
            ));
        }
    };

    let named_fields = match &data_struct.fields {
        Fields::Named(named) if !named.named.is_empty() => &named.named,
        _ => {
            return Err(Error::new_spanned(
                &ast.ident,
                format!(
                    "`#[derive({})]` requires a struct with at least one named field",
                    role.as_str()
                ),
            ));
        }
    };

    let mut fields = Vec::with_capacity(named_fields.len());
    for field in named_fields {
        fields.push(parse_field(field, role)?);
    }

    // If every field is `#[kova(auto)]`, `to_values()` yields `vec![]` and `COLUMNS_META`
    // has no insertable columns, which would produce invalid `INSERT INTO tbl () VALUES ()` SQL.
    if role == DeriveRole::Model && fields.iter().all(|f| f.is_auto) {
        return Err(Error::new_spanned(
            &ast.ident,
            "`#[derive(Model)]` requires at least one non-`#[kova(auto)]` field for INSERT",
        ));
    }

    let table_name = ast.ident.unraw().to_string().to_snake_case();

    Ok(StructDef { vis: ast.vis.clone(), ident: ast.ident.clone(), table_name, fields })
}

// rustc strips the active `#[derive(...)]` attribute before invoking the proc-macro.
// This means:
// 1. Sibling derives inside a single `#[derive(Model, FromRow)]` are already gone from `ast.attrs`
//    and will fail later via duplicate column consts / conflicting trait impls.
// 2. Any `#[derive(...)]` still present in `ast.attrs` is a separate `#[derive]` attribute below us
//    (e.g. `#[derive(Model)] #[derive(FromRow)]` or `#[derive(Model)] #[derive(Model)]`).
//    We catch those right here on the first pass so the first macro aborts without emitting code.
fn check_mutual_exclusivity(attrs: &[Attribute], current: DeriveRole) -> Result<(), Error> {
    for attr in attrs {
        if !attr.path().is_ident("derive") {
            continue;
        }
        attr.parse_nested_meta(|meta| {
            let Some(last_seg) = meta.path.segments.last() else {
                return Ok(());
            };
            for other in ["Model", "FromRow", "Cte"] {
                if last_seg.ident == other {
                    let msg = if other == current.as_str() {
                        format!("duplicate `#[derive({})]` on the same struct", current.as_str())
                    } else {
                        format!(
                            "cannot derive both `{}` and `{}` on the same struct; \
                             choose exactly one of `Model`, `FromRow`, or `Cte`",
                            current.as_str(),
                            other
                        )
                    };
                    return Err(Error::new_spanned(&meta.path, msg));
                }
            }
            Ok(())
        })?;
    }
    Ok(())
}

// Table names are strictly inferred from the struct identifier in `snake_case`.
fn reject_container_kova_attrs(attrs: &[Attribute]) -> Result<(), Error> {
    for attr in attrs {
        if attr.path().is_ident("kova") {
            return Err(Error::new_spanned(
                attr,
                "`#[kova(...)]` is not allowed on structs; table names are strictly derived from the struct name in `snake_case`",
            ));
        }
    }
    Ok(())
}

fn parse_field(field: &Field, role: DeriveRole) -> Result<FieldDef, Error> {
    let ident = field.ident.clone().expect("named field must have an ident");
    let col_name = ident.unraw().to_string();
    let mut is_auto = false;

    for attr in &field.attrs {
        if !attr.path().is_ident("kova") {
            continue;
        }

        // Check `role` before validating syntax inside `#[kova(...)]`.
        // Otherwise putting `#[kova()]` on a `FromRow` field would tell the user to write
        // `#[kova(auto)]`, only to reject `#[kova(auto)]` on the next compile.
        if role != DeriveRole::Model {
            return Err(Error::new_spanned(
                attr,
                format!(
                    "`#[kova(...)]` field attributes are only supported on `#[derive(Model)]`, not `#[derive({})]`",
                    role.as_str()
                ),
            ));
        }

        // Reject bare `#[kova]`, `#[kova = "..."]`, or empty `#[kova()]`
        match &attr.meta {
            Meta::List(list) if list.tokens.is_empty() => {
                return Err(Error::new_spanned(
                    attr,
                    "empty `#[kova()]` attribute; expected `#[kova(auto)]`",
                ));
            }
            Meta::Path(_) | Meta::NameValue(_) => {
                return Err(Error::new_spanned(
                    attr,
                    "invalid `#[kova]` syntax; expected `#[kova(auto)]`",
                ));
            }
            Meta::List(_) => {}
        }

        attr.parse_nested_meta(|meta| {
            if meta.path.is_ident("auto") {
                // Reject `#[kova(auto = ...)]` or `#[kova(auto(...))]`
                if !meta.input.is_empty() && !meta.input.peek(syn::Token![,]) {
                    return Err(Error::new_spanned(
                        &meta.path,
                        "`#[kova(auto)]` does not accept any arguments",
                    ));
                }
                if is_auto {
                    return Err(Error::new_spanned(
                        &meta.path,
                        "duplicate `#[kova(auto)]` attribute on field",
                    ));
                }
                is_auto = true;
                Ok(())
            } else {
                // Format `meta.path` (rather than the whole outer `Meta::List`) so `#[kova(auto, abc)]`
                // specifically reports `#[kova(abc)]`. Strip spaces in case of paths like `a::b`.
                let unknown = meta.path.to_token_stream().to_string().replace(' ', "");
                Err(Error::new_spanned(
                    &meta.path,
                    format!(
                        "unknown `#[kova({})]` attribute; only `#[kova(auto)]` is supported",
                        unknown
                    ),
                ))
            }
        })?;
    }

    Ok(FieldDef { ident, ty: field.ty.clone(), col_name, is_auto })
}
