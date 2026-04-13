extern crate proc_macro;

use std::{cmp::Ordering, collections::BTreeSet};

use proc_macro::TokenStream;
use quote::{format_ident, quote};
use syn::{
    parse_macro_input, punctuated::Punctuated, spanned::Spanned, Data, DataStruct, DeriveInput,
    Fields, FieldsNamed, LitStr, Token,
};

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
struct Version {
    major: u16,
    minor: u16,
}

impl Default for Version {
    fn default() -> Self {
        Self { major: 1, minor: 1 }
    }
}

impl Ord for Version {
    fn cmp(&self, other: &Self) -> Ordering {
        match self.major.cmp(&other.major) {
            Ordering::Equal => self.minor.cmp(&other.minor),
            major_order => major_order,
        }
    }
}

impl PartialOrd for Version {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl TryFrom<LitStr> for Version {
    type Error = syn::Error;

    fn try_from(version: LitStr) -> Result<Self, Self::Error> {
        let span = version.span();
        let version = version.value();
        if version == "reserved" {
            // We special case version 999 later instead of making JniVersion an enum
            return Ok(Version {
                major: 999,
                minor: 0,
            });
        }
        let mut split = version.splitn(2, '.').map(str::parse::<u16>);
        const EXPECTED: &str = r#"Expected "major.minor" version number or "reserved""#;
        let major = match split.next() {
            Some(Ok(number)) => number,
            _ => return Err(syn::Error::new(span, EXPECTED)),
        };
        let minor = match split.next() {
            Some(Ok(num)) => num,
            None => 0,
            _ => return Err(syn::Error::new(span, EXPECTED)),
        };
        Ok(Version { major, minor })
    }
}

impl syn::parse::Parse for Version {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        input.parse::<LitStr>()?.try_into()
    }
}

struct Config {
    name: String,
    version_default: Version,
}

impl syn::parse::Parse for Config {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let mut args = Punctuated::<LitStr, Token![,]>::parse_terminated(input)?.into_iter();
        let name = args
            .next()
            .map(|s| s.value())
            .unwrap_or_else(|| "JNI".to_string());
        let version_default = args
            .next()
            .map(Version::try_from)
            .transpose()?
            .unwrap_or_default();
        Ok(Self {
            name,
            version_default,
        })
    }
}

fn jni_to_union_impl(cfg: Config, input: DeriveInput) -> syn::Result<TokenStream> {
    let original_name = &input.ident;
    let original_visibility = &input.vis;

    let mut versions = BTreeSet::new();
    let mut versioned_fields = vec![];

    let Config {
        name,
        version_default,
    } = cfg;

    let Data::Struct(DataStruct {
        fields: Fields::Named(FieldsNamed { named: fields, .. }),
        ..
    }) = input.data
    else {
        return Err(syn::Error::new(
            input.span(),
            "Expected a struct with fields",
        ));
    };

    for mut field in fields {
        let mut jni_added_attr = None;
        field.attrs.retain(|attr| {
            if attr.path().is_ident("jni_added") {
                jni_added_attr = Some(attr.clone());
                false
            } else {
                true
            }
        });

        let version;
        if let Some(attr) = jni_added_attr {
            version = attr.parse_args::<Version>()?;
        } else {
            version = version_default;
        }

        versions.insert(version);
        versioned_fields.push((version, field));
    }

    // Quote structs and union
    let mut expanded = quote!();
    let mut union_members = quote!();

    for version in versions {
        const RESERVED: &str = "reserved";
        let reserved = version.major == 999;
        let version_suffix = match version {
            Version { major: 999, .. } => RESERVED.to_string(),
            Version { major, minor: 0 } => major.to_string(),
            Version { major, minor } => format!("{major}_{minor}"),
        };
        let span = original_name.span();
        let version_ident = if reserved {
            format_ident!("{RESERVED}", span = span)
        } else {
            format_ident!("v{version_suffix}", span = span)
        };
        let struct_ident = format_ident!("{original_name}_{version_suffix}", span = span);

        let last = versioned_fields
            .iter()
            .rposition(|(v, _f)| v <= &version)
            .unwrap_or(versioned_fields.len());
        let mut padding_idx = 0u32;

        let mut version_field_tokens = quote!();
        for (i, (field_min_version, field)) in versioned_fields.iter().enumerate() {
            if i > last {
                break;
            }
            if field_min_version > &version {
                let reserved_ident = format_ident!("_padding_{}", padding_idx);
                padding_idx += 1;
                version_field_tokens.extend(quote! { #reserved_ident: *mut c_void, });
            } else {
                version_field_tokens.extend(quote! { #field, });
            }
        }
        expanded.extend(quote! {
            #[allow(non_snake_case, non_camel_case_types)]
            #[repr(C)]
            #[derive(Copy, Clone)]
            #original_visibility struct #struct_ident {
                #version_field_tokens
            }
        });

        let api_comment = if reserved {
            "All defined members regardless of version".to_string()
        } else {
            format!("API when {name} version is at least [{name}_VERSION_{version_suffix}]")
        };
        union_members.extend(quote! {
            #[doc = #api_comment]
            #original_visibility #version_ident: #struct_ident,
        });
    }

    expanded.extend(quote! {
        #[repr(C)]
        #original_visibility union #original_name {
            #union_members
        }
    });

    Ok(TokenStream::from(expanded))
}

#[proc_macro_attribute]
pub fn jni_to_union(attr: TokenStream, item: TokenStream) -> TokenStream {
    let input = parse_macro_input!(item as DeriveInput);
    let args = parse_macro_input!(attr as Config);

    match jni_to_union_impl(args, input) {
        Ok(tokens) => tokens,
        Err(err) => err.into_compile_error().into(),
    }
}
