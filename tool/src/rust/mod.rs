//! Experimental Safe Rust client backend over Diplomat's native C ABI.
//!
//! The backend is split the way the other backends are: this file is the entry
//! point (`RustConfig`, [`attr_support`], [`run`]), `formatter` owns naming,
//! `validate` owns every rejection, `lifetimes` renders HIR lifetime
//! environments, `type_map` is the type mapping layer, and `gen` renders the
//! generated package's files.

mod formatter;
mod gen;
mod lifetimes;
mod type_map;
mod validate;

use std::cell::Cell;

use diplomat_core::hir::{BackendAttrSupport, DocsUrlGenerator, TypeContext};
use serde::{Deserialize, Serialize};

use crate::{Config, ErrorStore, FileMap};

use formatter::{opaque_module_name, sanitize_package_component, valid_package_name};
use gen::{
    generate_ffi, generate_lib, generate_opaque_file, generate_opaques_index, generate_private,
    generate_types,
};
use validate::{validate, Reporter};

#[derive(Clone, Default, Debug, Serialize, Deserialize)]
pub struct RustConfig {
    /// Cargo package name for the generated bindings.
    pub crate_name: Option<String>,
    /// Native library name passed to Rust's `#[link]` attribute.
    pub dylib_name: Option<String>,
}

impl RustConfig {
    pub fn set(&mut self, key: &str, value: toml::Value) {
        let value = value
            .as_str()
            .unwrap_or_else(|| panic!("Rust config key `{key}` must be a string"))
            .to_string();
        match key {
            "crate_name" => self.crate_name = Some(value),
            "dylib_name" => self.dylib_name = Some(value),
            _ => panic!("Unrecognized Rust config key: {key}"),
        }
    }
}

pub(crate) fn attr_support() -> BackendAttrSupport {
    let mut support = BackendAttrSupport::default();
    // Generated code borrows directly out of provider-owned memory
    // (`from_raw_parts` over the provider's pointer), so memory_sharing-gated
    // shapes are ones this backend represents natively. Note the interaction with
    // the rejected primitive set: the shared corpus gates `&[f64]` constructors on
    // this flag, and floats are rejected, so pointing this backend at the shared
    // corpus fails on those methods until floats land (issue #10 on the fork).
    support.memory_sharing = true;
    support.constructors = true;
    // `named_constructor` maps to an associated function named after the declared
    // constructor name; see `method_name`.
    support.named_constructors = true;
    support.option = true;
    support.mutable_slices = true;
    support.static_slices = true;
    support.owned_byte_slice_returns = true;
    // The generated API surface is Rust, so it is UTF-8 by construction, and
    // `&DiplomatStr16` maps to `&[u16]`.
    support.utf8_strings = true;
    support.utf16_strings = true;
    support
}

pub(crate) fn run<'tcx>(
    tcx: &'tcx TypeContext,
    config: &Config,
    docs_url_gen: &DocsUrlGenerator,
) -> (FileMap, ErrorStore<'tcx, String>) {
    let files = FileMap::default();
    let errors = ErrorStore::default();
    let invalid = Cell::new(false);

    let reporter = Reporter::new(&errors, &invalid);
    validate(tcx, &reporter);
    if reporter.is_invalid() {
        return (files, errors);
    }

    let lib_name = config
        .shared_config
        .lib_name
        .as_deref()
        .unwrap_or("diplomat_native");
    let crate_name = config
        .rust_config
        .crate_name
        .clone()
        .unwrap_or_else(|| format!("{}-bindings", sanitize_package_component(lib_name)));
    let dylib_name = config
        .rust_config
        .dylib_name
        .clone()
        .unwrap_or_else(|| lib_name.to_string());

    if !valid_package_name(&crate_name) {
        if let Some((_, def)) = tcx.all_types().next() {
            let _guard = errors.set_context_ty(def.name_with_span().into());
            errors.push_error(format!(
                "[Rust backend] `{crate_name}` is not a valid generated Cargo package name"
            ));
        }
        return (files, errors);
    }
    if dylib_name.is_empty()
        || !dylib_name
            .chars()
            .all(|c| c.is_ascii_alphanumeric() || matches!(c, '_' | '-'))
    {
        if let Some((_, def)) = tcx.all_types().next() {
            let _guard = errors.set_context_ty(def.name_with_span().into());
            errors.push_error(format!(
                "[Rust backend] `{dylib_name}` is not a valid native library name"
            ));
        }
        return (files, errors);
    }

    let package = include_str!("../../templates/rust/Cargo.toml.template")
        .replace("{{crate_name}}", &crate_name);
    let build = include_str!("../../templates/rust/build.rs.template");
    files.add_file("Cargo.toml".into(), package);
    files.add_file("build.rs".into(), build.into());
    files.add_file("src/lib.rs".into(), generate_lib());
    files.add_file("src/ffi.rs".into(), generate_ffi(tcx, &dylib_name));
    files.add_file("src/private.rs".into(), generate_private(tcx));
    files.add_file("src/types.rs".into(), generate_types(tcx, docs_url_gen));
    files.add_file("src/opaques.rs".into(), generate_opaques_index(tcx));
    for opaque in tcx.opaques().iter().filter(|ty| !ty.attrs.disable) {
        files.add_file(
            format!("src/opaques/{}.rs", opaque_module_name(opaque)),
            generate_opaque_file(tcx, opaque, docs_url_gen),
        );
    }
    (files, errors)
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use diplomat_core::hir::{BasicAttributeValidator, DocsUrlGenerator, TypeContext};
    use quote::quote;

    use super::type_map::primitive_name;
    use crate::Config;

    /// Concatenates every generated Rust source, for assertions about *what* is
    /// generated rather than *which file* it lands in.
    fn all_rust_sources(files: &HashMap<String, String>) -> String {
        let mut paths: Vec<&String> = files.keys().filter(|path| path.ends_with(".rs")).collect();
        paths.sort();
        paths
            .into_iter()
            .map(|path| files[path].as_str())
            .collect::<Vec<_>>()
            .join("\n")
    }

    fn generate(tokens: proc_macro2::TokenStream) -> (HashMap<String, String>, Vec<String>) {
        let file = syn::parse2::<syn::File>(tokens).unwrap();
        let mut validator = BasicAttributeValidator::new("rust");
        validator.support = super::attr_support();
        let tcx = TypeContext::from_syn(
            &file,
            Default::default(),
            validator,
            None,
            &diplomat_core::ast::SpanLocation::None,
        )
        .unwrap_or_else(|errors| panic!("HIR errors: {errors:#?}"));
        let mut config = Config::default();
        config.shared_config.lib_name = Some("safe_rust_test".into());
        let docs = DocsUrlGenerator::with_base_urls(None, HashMap::new());
        let (files, errors) = super::run(&tcx, &config, &docs);
        (
            files.take_files(),
            errors
                .take_all()
                .into_iter()
                .map(|(context, error)| format!("{context}: {error}"))
                .collect(),
        )
    }

    #[test]
    fn opaque_api_is_safe_and_raw_layer_is_private() {
        let (files, errors) = generate(quote! {
            #[diplomat::bridge]
            mod ffi {
                #[diplomat::opaque_mut]
                pub struct Counter(u32);
                impl Counter {
                    pub fn new(value: u32) -> Box<Self> { unimplemented!() }
                    pub fn increment(&mut self) { unimplemented!() }
                    pub fn get(&self) -> u32 { unimplemented!() }
                    pub fn same(&self, other: &Counter) -> bool { unimplemented!() }
                }
            }
        });
        assert!(errors.is_empty(), "{errors:#?}");
        let safe = &all_rust_sources(&files);
        let raw = &files["src/ffi.rs"];
        assert!(safe.contains("mod ffi;"));
        assert!(safe.contains("pub struct Counter"));
        assert!(safe.contains("impl Drop for Counter"));
        assert!(safe.contains("pub fn increment(&mut self)"));
        assert!(safe.contains("other: &impl super::CounterSharedArg"));
        assert!(!safe.contains("pub mod ffi"));
        assert!(raw.contains("extern \"C\""));
        assert!(raw.contains("Counter_destroy"));
    }

    #[test]
    fn borrowed_return_preserves_hir_lifetime() {
        let (files, errors) = generate(quote! {
            #[diplomat::bridge]
            mod ffi {
                #[diplomat::opaque]
                pub struct Child(u32);
                #[diplomat::opaque]
                pub struct Parent(Child);
                impl Parent {
                    pub fn child<'a>(&'a self) -> &'a Child { unimplemented!() }
                }
            }
        });
        assert!(errors.is_empty(), "{errors:#?}");
        assert!(
            all_rust_sources(&files).contains("pub fn child<'a>(&'a self) -> super::ChildRef<'a>")
        );
    }

    #[test]
    fn borrowed_return_preserves_transitive_input_bound() {
        let (files, errors) = generate(quote! {
            #[diplomat::bridge]
            mod ffi {
                #[diplomat::opaque]
                pub struct Child(u32);
                #[diplomat::opaque]
                pub struct Parent(Child);
                impl Parent {
                    pub fn child<'short, 'long: 'short>(&'long self) -> &'short Child {
                        unimplemented!()
                    }
                }
            }
        });
        assert!(errors.is_empty(), "{errors:#?}");
        let safe = &all_rust_sources(&files);
        assert!(safe.contains("'long: 'short"), "{safe}");
        assert!(
            safe.contains("&'long self) -> super::ChildRef<'short>"),
            "{safe}"
        );
        assert!(!safe.contains("'long, 'long:"), "{safe}");
    }

    #[test]
    fn value_types_and_options_are_lowered() {
        let (files, errors) = generate(quote! {
            #[diplomat::bridge]
            mod ffi {
                pub enum Mode { First = 0, Second = 2 }
                pub struct Snapshot { pub value: u32, pub enabled: bool, pub mode: Mode }
                #[diplomat::opaque]
                pub struct Counter(u32);
                impl Counter {
                    pub fn snapshot(&self) -> Snapshot { unimplemented!() }
                    pub fn maybe_value(&self, value: Option<u32>) -> Option<u32> { unimplemented!() }
                }
            }
        });
        assert!(errors.is_empty(), "{errors:#?}");
        let safe = &all_rust_sources(&files);
        assert!(safe.contains("pub enum Mode"));
        assert!(safe.contains("pub struct Snapshot"));
        assert!(safe.contains("value: Option<u32>"));
        assert!(files["src/ffi.rs"].contains("DiplomatOption<u32>"));
    }

    #[test]
    fn unsupported_slice_produces_no_files() {
        let (files, errors) = generate(quote! {
            #[diplomat::bridge]
            mod ffi {
                #[diplomat::opaque]
                pub struct Counter(u32);
                impl Counter {
                    // `i128` stays outside the supported primitive subset, so a slice of
                    // it is still rejected. (This case used `f64` until floats were
                    // added to the subset.)
                    pub fn consume(&self, samples: &[i128]) { unimplemented!() }
                }
            }
        });
        assert!(files.is_empty());
        assert!(errors
            .iter()
            .any(|error| error.contains("unsupported parameter type")));
    }

    #[test]
    fn primitive_subset_is_explicit() {
        use diplomat_core::hir::{FloatType, Int128Type, IntSizeType, IntType, PrimitiveType};

        for primitive in [
            PrimitiveType::Bool,
            PrimitiveType::Byte,
            PrimitiveType::Int(IntType::I8),
            PrimitiveType::Int(IntType::I16),
            PrimitiveType::Int(IntType::I32),
            PrimitiveType::Int(IntType::I64),
            PrimitiveType::Int(IntType::U8),
            PrimitiveType::Int(IntType::U16),
            PrimitiveType::Int(IntType::U32),
            PrimitiveType::Int(IntType::U64),
            PrimitiveType::IntSize(IntSizeType::Isize),
            PrimitiveType::IntSize(IntSizeType::Usize),
            PrimitiveType::Float(FloatType::F32),
            PrimitiveType::Float(FloatType::F64),
        ] {
            assert!(primitive_name(primitive).is_some());
        }
        // Deliberately still out of the subset: `char` awaits a code-point validity
        // decision, `Ordering` has no agreed ABI shape, and 128-bit integers are not
        // FFI-safe on every target. A `Some` here would be a promise we cannot keep.
        for primitive in [
            PrimitiveType::Char,
            PrimitiveType::Ordering,
            PrimitiveType::Int128(Int128Type::I128),
            PrimitiveType::Int128(Int128Type::U128),
        ] {
            assert!(primitive_name(primitive).is_none());
        }
    }

    #[test]
    fn rust_config_accepts_package_and_dylib_names() {
        let mut config = super::RustConfig::default();
        config.set("crate_name", toml::Value::String("safe-bindings".into()));
        config.set("dylib_name", toml::Value::String("native_owner".into()));
        assert_eq!(config.crate_name.as_deref(), Some("safe-bindings"));
        assert_eq!(config.dylib_name.as_deref(), Some("native_owner"));
    }

    #[test]
    fn optional_owned_and_borrowed_opaques_are_distinct() {
        let (files, errors) = generate(quote! {
            #[diplomat::bridge]
            mod ffi {
                #[diplomat::opaque]
                pub struct Child(u32);
                #[diplomat::opaque]
                pub struct Parent(Child);
                impl Parent {
                    pub fn maybe_new(present: bool) -> Option<Box<Self>> { unimplemented!() }
                    pub fn maybe_child<'a>(&'a self, present: bool) -> Option<&'a Child> { unimplemented!() }
                }
            }
        });
        assert!(errors.is_empty(), "{errors:#?}");
        let safe = &all_rust_sources(&files);
        assert!(safe.contains("-> Option<super::Parent>"));
        assert!(safe.contains("-> Option<super::ChildRef<'a>>"));
        assert!(safe.contains("NonNull::new(result as *mut _).map"));
    }

    #[test]
    fn fallible_results_are_rejected_without_partial_output() {
        let (files, errors) = generate(quote! {
            #[diplomat::bridge]
            mod ffi {
                #[diplomat::opaque]
                pub struct Counter(u32);
                impl Counter {
                    pub fn get(&self) -> Result<u32, u32> { unimplemented!() }
                }
            }
        });
        assert!(files.is_empty());
        assert!(errors
            .iter()
            .any(|error| error.contains("unsupported return type")));
    }

    #[test]
    fn str_parameters_are_lowered_to_byte_slices() {
        let (files, errors) = generate(quote! {
            #[diplomat::bridge]
            mod ffi {
                #[diplomat::opaque]
                pub struct Echo(String);
                impl Echo {
                    pub fn new(v: &str) -> Box<Self> { unimplemented!() }
                }
            }
        });
        assert!(errors.is_empty(), "{errors:#?}");
        let safe = &all_rust_sources(&files);
        assert!(safe.contains("pub fn new(v: &str)"), "{safe}");
        // A `&str` cannot become a `DiplomatSlice<u8>` by conversion alone — the
        // runtime's `DiplomatUtf8StrSlice` field is private — so it goes via bytes.
        assert!(
            safe.contains("ffi::DiplomatSlice::from(v.as_bytes())"),
            "{safe}"
        );
    }

    #[test]
    fn borrowed_slices_and_strings_are_lowered() {
        let (files, errors) = generate(quote! {
            #[diplomat::bridge]
            mod ffi {
                use diplomat_runtime::{DiplomatStr, DiplomatStrSlice};

                #[diplomat::opaque_mut]
                pub struct U32Vec(Vec<u32>);
                impl U32Vec {
                    pub fn new(v: &[u32]) -> Box<Self> { unimplemented!() }
                    pub fn borrow<'a>(&'a self) -> &'a [u32] { unimplemented!() }
                    pub fn fill_slice(&self, v: &mut [u32]) { unimplemented!() }
                }

                #[diplomat::opaque_mut]
                pub struct MyString(String);
                impl MyString {
                    pub fn new(v: &DiplomatStr) -> Box<Self> { unimplemented!() }
                    pub fn get<'a>(&'a self) -> DiplomatStrSlice<'a> { unimplemented!() }
                    pub fn as_str<'a>(&'a self) -> &'a str { unimplemented!() }
                }
            }
        });
        assert!(errors.is_empty(), "{errors:#?}");
        let safe = &all_rust_sources(&files);
        assert!(
            safe.contains("pub fn borrow<'a>(&'a self) -> &'a [u32]"),
            "{safe}"
        );
        assert!(
            safe.contains("pub fn fill_slice(&self, v: &mut [u32])"),
            "{safe}"
        );
        assert!(
            safe.contains("pub fn get<'a>(&'a self) -> &'a [u8]"),
            "{safe}"
        );
        assert!(
            safe.contains("pub fn as_str<'a>(&'a self) -> &'a str"),
            "{safe}"
        );
        let ffi = &files["src/ffi.rs"];
        assert!(ffi.contains("DiplomatSlice<u32>"), "{ffi}");
        assert!(ffi.contains("DiplomatSliceMut<u32>"), "{ffi}");
        assert!(
            ffi.contains("pub(super) use diplomat_runtime::{"),
            "the FFI layer must re-export the runtime ABI types: {ffi}"
        );
        assert!(
            !ffi.contains("struct DiplomatSlice"),
            "the FFI layer must not re-declare ABI types: {ffi}"
        );
        assert!(
            safe.contains("crate::private::utf8_str_from_slice(result)"),
            "{safe}"
        );
        assert!(!safe.contains("slice_from_raw_parts"), "{safe}");
    }

    #[test]
    fn slice_output_lifetime_is_tied_to_receiver() {
        let (files, errors) = generate(quote! {
            #[diplomat::bridge]
            mod ffi {
                #[diplomat::opaque_mut]
                pub struct Buf(Vec<u8>);
                impl Buf {
                    pub fn bytes<'a>(&'a self) -> &'a [u8] { unimplemented!() }
                    pub fn bytes_mut<'a>(&'a mut self) -> &'a mut [u8] { unimplemented!() }
                }
            }
        });
        assert!(errors.is_empty(), "{errors:#?}");
        let safe = &all_rust_sources(&files);
        assert!(
            safe.contains("pub fn bytes<'a>(&'a self) -> &'a [u8]"),
            "{safe}"
        );
        assert!(
            safe.contains("pub fn bytes_mut<'a>(&'a mut self) -> &'a mut [u8]"),
            "{safe}"
        );
    }
    #[test]
    fn owned_slice_returns_are_boxed() {
        let (files, errors) = generate(quote! {
            #[diplomat::bridge]
            mod ffi {
                #[diplomat::opaque]
                pub struct OwnedSliceReturn;
                impl OwnedSliceReturn {
                    pub fn make(len: u32) -> Box<[u8]> { unimplemented!() }
                }
            }
        });
        assert!(errors.is_empty(), "{errors:#?}");
        let safe = &all_rust_sources(&files);
        let ffi = &files["src/ffi.rs"];
        assert!(
            safe.contains("pub fn make(len: u32) -> Box<[u8]>"),
            "{safe}"
        );
        assert!(safe.contains("Box::from(result)"), "{safe}");
        assert!(ffi.contains("DiplomatOwnedSlice<u8>"), "{ffi}");
    }

    #[test]
    fn lifetime_structs_are_converted() {
        let (files, errors) = generate(quote! {
            #[diplomat::bridge]
            mod ffi {
                use diplomat_runtime::{DiplomatStr, DiplomatStrSlice};

                pub struct BorrowedFields<'a> {
                    a: DiplomatStrSlice<'a>,
                    b: u32,
                }

                #[diplomat::opaque]
                pub struct Foo<'a>(&'a DiplomatStr);

                impl<'a> Foo<'a> {
                    pub fn as_fields(&self) -> BorrowedFields<'a> { unimplemented!() }
                    pub fn from_fields(fields: BorrowedFields<'a>) -> Box<Self> { unimplemented!() }
                }
            }
        });
        assert!(errors.is_empty(), "{errors:#?}");
        let safe = &all_rust_sources(&files);
        let ffi = &files["src/ffi.rs"];
        assert!(safe.contains("pub struct BorrowedFields<'a>"), "{safe}");
        assert!(safe.contains("pub a: &'a [u8]"), "{safe}");
        assert!(safe.contains("pub b: u32"), "{safe}");
        assert!(
            safe.contains("pub fn as_fields<'anon_0>(&'anon_0 self) -> BorrowedFields<'a>")
                || safe.contains("-> BorrowedFields<'a>"),
            "{safe}"
        );
        assert!(ffi.contains("pub struct BorrowedFields<'a> {"), "{ffi}");
        assert!(ffi.contains("a: DiplomatSlice<'a, u8>"), "{ffi}");
    }

    #[test]
    fn opaque_type_lifetimes_are_threaded() {
        let (files, errors) = generate(quote! {
            #[diplomat::bridge]
            mod ffi {
                use diplomat_runtime::DiplomatStr;
                #[diplomat::opaque]
                pub struct Foo<'a>(&'a DiplomatStr);
                #[diplomat::opaque]
                pub struct Bar<'b, 'a: 'b>(&'b Foo<'a>);
                impl<'a> Foo<'a> {
                    pub fn new(x: &'a DiplomatStr) -> Box<Self> { unimplemented!() }
                    pub fn get_bar<'b>(&'b self) -> Box<Bar<'b, 'a>> { unimplemented!() }
                }
            }
        });
        assert!(errors.is_empty(), "{errors:#?}");
        let safe = &all_rust_sources(&files);
        assert!(safe.contains("pub struct Foo<'a>"), "{safe}");
        assert!(safe.contains("impl<'a> Drop for Foo<'a>"), "{safe}");
        assert!(safe.contains("pub struct Bar<'b, 'a: 'b>"), "{safe}");
        assert!(safe.contains("impl<'a> Foo<'a> {"), "{safe}");
        assert!(
            safe.contains("pub fn new(x: &'a [u8]) -> super::Foo<'a>"),
            "{safe}"
        );
        assert!(
            safe.contains("pub fn get_bar<'b>(&'b self) -> super::Bar<'b, 'a>"),
            "{safe}"
        );
    }

    /// `#[diplomat::attr(rust, disable)]` types are never generated, so a signature
    /// that mentions one has to be rejected. It used to be accepted — enums were
    /// checked but structs and opaques were not — and the emitted package then
    /// referenced types that do not exist and could not compile.
    #[test]
    fn disabled_types_in_signatures_are_rejected() {
        let (files, errors) = generate(quote! {
            #[diplomat::bridge]
            mod ffi {
                #[diplomat::opaque]
                #[diplomat::attr(rust, disable)]
                pub struct HiddenOpaque(u32);

                #[diplomat::attr(rust, disable)]
                pub struct HiddenStruct {
                    pub value: u32,
                }

                #[diplomat::attr(rust, disable)]
                pub enum HiddenEnum {
                    Only,
                }

                pub struct Holder {
                    pub hidden: HiddenEnum,
                }

                #[diplomat::opaque]
                pub struct Visible(u32);

                impl Visible {
                    pub fn takes_disabled_opaque(&self, hidden: &HiddenOpaque) { unimplemented!() }
                    pub fn takes_disabled_struct(&self, hidden: HiddenStruct) { unimplemented!() }
                    pub fn takes_optional_disabled(&self, hidden_opt: Option<HiddenStruct>) { unimplemented!() }
                    pub fn returns_disabled_opaque(&self) -> Box<HiddenOpaque> { unimplemented!() }
                }
            }
        });
        for expected in [
            "disabled type `HiddenOpaque` as parameter `hidden`",
            "disabled type `HiddenStruct` as parameter `hidden_opt`",
            "disabled type `HiddenOpaque` in the return type",
            "structs may contain only supported primitives, enums, and borrowed slices",
        ] {
            assert!(
                errors.iter().any(|error| error.contains(expected)),
                "missing {expected:?} in {errors:#?}"
            );
        }
        assert!(
            files.is_empty(),
            "nothing may be emitted for a rejected signature: {:#?}",
            files.keys()
        );
    }

    /// An explicit `named_constructor` name is the emitted function name; a bare
    /// `named_constructor` keeps the Rust method name. The name used to be discarded
    /// silently, so the declaration and the generated API disagreed.
    #[test]
    fn named_constructor_names_are_honoured() {
        let (files, errors) = generate(quote! {
            #[diplomat::bridge]
            mod ffi {
                #[diplomat::opaque]
                pub struct Thing(u32);
                impl Thing {
                    #[diplomat::attr(auto, named_constructor = "with_value")]
                    pub fn new_named(value: u32) -> Box<Self> { unimplemented!() }
                    #[diplomat::attr(auto, named_constructor)]
                    pub fn new_plain(value: u32) -> Box<Self> { unimplemented!() }
                }
            }
        });
        assert!(errors.is_empty(), "{errors:#?}");
        let safe = &all_rust_sources(&files);
        assert!(safe.contains("pub fn with_value(value: u32)"), "{safe}");
        assert!(safe.contains("pub fn new_plain(value: u32)"), "{safe}");
        assert!(!safe.contains("pub fn new_named("), "{safe}");
    }

    /// A `#[diplomat::cfg(supports = ...)]` method is generated only when the backend
    /// declares that flag. This ties each declaration in `attr_support` to an
    /// observable effect, so a flag cannot be dropped or added without changing what
    /// comes out.
    #[test]
    fn capability_flags_gate_generated_apis() {
        let (files, errors) = generate(quote! {
            #[diplomat::bridge]
            mod ffi {
                #[diplomat::opaque]
                pub struct Thing(Vec<u16>);
                impl Thing {
                    #[diplomat::cfg(supports = utf16_strings)]
                    pub fn utf16_len(&self) -> u32 { unimplemented!() }
                    #[diplomat::cfg(supports = callbacks)]
                    pub fn needs_callbacks(&self) { unimplemented!() }
                }
            }
        });
        assert!(errors.is_empty(), "{errors:#?}");
        let safe = &all_rust_sources(&files);
        assert!(safe.contains("pub fn utf16_len("), "{safe}");
        assert!(!safe.contains("needs_callbacks"), "{safe}");
    }

    /// Generated module names follow the repo-wide `heck` snake_case convention, the
    /// one every sibling backend's formatter uses. This is deliberately different from
    /// a naive per-underscore split for acronyms: `HTTPServer` becomes `http_server`,
    /// not `h_t_t_p_server`.
    #[test]
    fn opaque_module_names_use_snake_case() {
        let (files, errors) = generate(quote! {
            #[diplomat::bridge]
            mod ffi {
                #[diplomat::opaque]
                pub struct HTTPServer(u32);
                impl HTTPServer {
                    pub fn new() -> Box<Self> { unimplemented!() }
                }
            }
        });
        assert!(errors.is_empty(), "{errors:#?}");
        assert!(
            files.contains_key("src/opaques/http_server.rs"),
            "{:#?}",
            files.keys()
        );
    }

    /// Floats are FFI-safe scalars and the backend is Rust-to-Rust, so they need no
    /// conversion: the same `f32`/`f64` is the ABI type and the safe type.
    #[test]
    fn float_scalars_and_slices_are_lowered() {
        let (files, errors) = generate(quote! {
            #[diplomat::bridge]
            mod ffi {
                #[diplomat::opaque_mut]
                pub struct Floats(Vec<f64>);
                impl Floats {
                    pub fn new(values: &[f64]) -> Box<Self> { unimplemented!() }
                    pub fn scale(&mut self, factor: f64) -> f64 { unimplemented!() }
                    pub fn narrow(&self) -> f32 { unimplemented!() }
                    pub fn values<'a>(&'a self) -> &'a [f64] { unimplemented!() }
                }
            }
        });
        assert!(errors.is_empty(), "{errors:#?}");
        let safe = &all_rust_sources(&files);
        assert!(safe.contains("pub fn new(values: &[f64])"), "{safe}");
        assert!(
            safe.contains("pub fn scale(&mut self, factor: f64) -> f64"),
            "{safe}"
        );
        assert!(safe.contains("pub fn narrow(&self) -> f32"), "{safe}");
        assert!(
            safe.contains("pub fn values<'a>(&'a self) -> &'a [f64]"),
            "{safe}"
        );
    }

    /// The primitives left out of the subset must still stop generation outright,
    /// rather than emitting bindings that quietly drop the method.
    #[test]
    fn unsupported_primitives_still_reject_without_output() {
        for tokens in [
            quote! {
                #[diplomat::bridge]
                mod ffi {
                    #[diplomat::opaque]
                    pub struct Wide(u128);
                    impl Wide {
                        pub fn get(&self) -> u128 { unimplemented!() }
                    }
                }
            },
            quote! {
                #[diplomat::bridge]
                mod ffi {
                    use diplomat_runtime::DiplomatChar;

                    #[diplomat::opaque]
                    pub struct Letter(u32);
                    impl Letter {
                        pub fn get(&self) -> DiplomatChar { unimplemented!() }
                    }
                }
            },
        ] {
            let (files, errors) = generate(tokens);
            assert!(!errors.is_empty(), "expected a diagnostic");
            assert!(files.is_empty(), "{:#?}", files.keys());
        }
    }

    /// A plain `#[diplomat::attr(auto, constructor)]` is accepted and emitted. For Rust
    /// that is an ordinary associated function, so its generated name is just the
    /// method's own name — the flag is a promise that such methods are lowered at all.
    #[test]
    fn plain_constructors_are_accepted() {
        let (files, errors) = generate(quote! {
            #[diplomat::bridge]
            mod ffi {
                #[diplomat::opaque]
                pub struct Thing(u32);
                impl Thing {
                    #[diplomat::attr(auto, constructor)]
                    pub fn create(value: u32) -> Box<Self> { unimplemented!() }
                }
            }
        });
        assert!(errors.is_empty(), "{errors:#?}");
        let safe = &all_rust_sources(&files);
        assert!(safe.contains("pub fn create(value: u32)"), "{safe}");
    }

    /// How each capability flag claimed by `attr_support` is covered.
    ///
    /// `attr_support` is a promise to HIR lowering, not documentation: a provider
    /// guarded by `#[diplomat::cfg(supports = <flag>)]` is accepted and lowered, so an
    /// untested promise can yield bindings that compile and misbehave. A `gated:` row
    /// means the flag guards a fixture API, so dropping the flag removes the API and the
    /// consumer tests stop compiling; the other rows name the generator test plus the
    /// fixture method that exercises the shape at runtime.
    const FLAG_COVERAGE: &[(&str, &str)] = &[
        (
            "constructors",
            "plain_constructors_are_accepted; gated: Counter::from_value; Counter::new at runtime",
        ),
        (
            "memory_sharing",
            "gated: Numbers::from_slice, Float64Vec::new",
        ),
        (
            "mutable_slices",
            "borrowed_slices_and_strings_are_lowered; gated: Numbers::values_mut, Numbers::fill",
        ),
        (
            "named_constructors",
            "named_constructor_names_are_honoured; gated: Counter::with_value",
        ),
        (
            "option",
            "value_types_and_options_are_lowered; gated: Counter::add, Counter::maybe_snapshot",
        ),
        (
            "owned_byte_slice_returns",
            "owned_slice_returns_are_boxed; gated: Bytes::make, Bytes::join",
        ),
        ("static_slices", "gated: Numbers::from_static"),
        (
            "utf16_strings",
            "gated: WideMessage; capability_flags_gate_generated_apis",
        ),
        ("utf8_strings", "gated: Message::utf8_len"),
    ];

    /// Every `support.<flag> = true` in `attr_support` needs a [`FLAG_COVERAGE`] row, so
    /// a capability cannot be claimed without covering it — the mechanical half of the
    /// rule whose human half is the table itself.
    #[test]
    fn every_claimed_flag_is_covered() {
        let source = include_str!("mod.rs");
        let body = source
            .split_once("pub(crate) fn attr_support")
            .expect("attr_support is defined")
            .1
            .split_once("\n}\n")
            .expect("attr_support has a body")
            .0;
        let mut claimed: Vec<&str> = body
            .lines()
            .filter_map(|line| line.trim().strip_prefix("support."))
            .filter_map(|rest| rest.strip_suffix(" = true;"))
            .collect();
        claimed.sort_unstable();
        let mut covered: Vec<&str> = FLAG_COVERAGE.iter().map(|(flag, _)| *flag).collect();
        covered.sort_unstable();
        assert_eq!(
            claimed, covered,
            "every claimed flag needs a FLAG_COVERAGE row, and every row needs a claim"
        );
    }
}
