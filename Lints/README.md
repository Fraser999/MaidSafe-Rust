# Lints

* [List of all lints](#list-of-all-lints)
* [Full details of all lints](#full-details-of-all-lints)
* [Lint groups](#lint-groups)

Current list derived from [rustc 1.34.0](Lints.1.34.0.txt) by running `rustc -W help`.

```rust
#![doc(test(attr(forbid(warnings))))]
#![warn(unused, missing_copy_implementations, missing_docs)]
#![deny(
    deprecated_in_future,
    future_incompatible,
    macro_use_extern_crate,
    rust_2018_idioms,
    nonstandard_style,
    single_use_lifetimes,
    trivial_casts,
    trivial_numeric_casts,
    unreachable_pub,
    unsafe_code,
    unstable_features,
    unused_import_braces,
    unused_lifetimes,
    unused_qualifications,
    unused_results,
    warnings
)]
#![forbid(
    const_err,
    duplicate_macro_exports,
    exceeding_bitshifts,
    incoherent_fundamental_impls,
    invalid_type_param_default,
    legacy_constructor_visibility,
    legacy_directory_ownership,
    macro_expanded_macro_exports_accessed_by_absolute_paths,
    missing_fragment_specifier,
    mutable_transmutes,
    no_mangle_const_items,
    order_dependent_trait_objects,
    overflowing_literals,
    parenthesized_params_in_types_and_modules,
    pub_use_of_private_extern_crate,
    safe_extern_statics,
    unknown_crate_types
)]
#![allow()]
```

---

## List of all lints

* [`absolute_paths_not_starting_with_crate`](#absolute_paths_not_starting_with_crate)
* [`ambiguous_associated_items`](#ambiguous_associated_items)
* [`anonymous_parameters`](#anonymous_parameters)
* [`bare_trait_objects`](#bare_trait_objects)
* [`box_pointers`](#box_pointers)
* [`const_err`](#const_err)
* [`dead_code`](#dead_code)
* [`deprecated`](#deprecated)
* [`deprecated_in_future`](#deprecated_in_future)
* [`duplicate_macro_exports`](#duplicate_macro_exports)
* [`elided_lifetimes_in_paths`](#elided_lifetimes_in_paths)
* [`ellipsis_inclusive_range_patterns`](#ellipsis_inclusive_range_patterns)
* [`exceeding_bitshifts`](#exceeding_bitshifts)
* [`explicit_outlives_requirements`](#explicit_outlives_requirements)
* [`exported_private_dependencies`](#exported_private_dependencies)
* [`ill_formed_attribute_input`](#ill_formed_attribute_input)
* [`illegal_floating_point_literal_pattern`](#illegal_floating_point_literal_pattern)
* [`improper_ctypes`](#improper_ctypes)
* [`incoherent_fundamental_impls`](#incoherent_fundamental_impls)
* [`intra_doc_link_resolution_failure`](#intra_doc_link_resolution_failure)
* [`invalid_type_param_default`](#invalid_type_param_default)
* [`irrefutable_let_patterns`](#irrefutable_let_patterns)
* [`keyword_idents`](#keyword_idents)
* [`late_bound_lifetime_arguments`](#late_bound_lifetime_arguments)
* [`legacy_constructor_visibility`](#legacy_constructor_visibility)
* [`legacy_directory_ownership`](#legacy_directory_ownership)
* [`macro_expanded_macro_exports_accessed_by_absolute_paths`](#macro_expanded_macro_exports_accessed_by_absolute_paths)
* [`macro_use_extern_crate`](#macro_use_extern_crate)
* [`missing_copy_implementations`](#missing_copy_implementations)
* [`missing_debug_implementations`](#missing_debug_implementations)
* [`missing_doc_code_examples`](#missing_doc_code_examples)
* [`missing_docs`](#missing_docs)
* [`missing_fragment_specifier`](#missing_fragment_specifier)
* [`mutable_transmutes`](#mutable_transmutes)
* [`nested_impl_trait`](#nested_impl_trait)
* [`no_mangle_const_items`](#no_mangle_const_items)
* [`no_mangle_generic_items`](#no_mangle_generic_items)
* [`non_camel_case_types`](#non_camel_case_types)
* [`non_shorthand_field_patterns`](#non_shorthand_field_patterns)
* [`non_snake_case`](#non_snake_case)
* [`non_upper_case_globals`](#non_upper_case_globals)
* [`order_dependent_trait_objects`](#order_dependent_trait_objects)
* [`overflowing_literals`](#overflowing_literals)
* [`parenthesized_params_in_types_and_modules`](#parenthesized_params_in_types_and_modules)
* [`path_statements`](#path_statements)
* [`patterns_in_fns_without_body`](#patterns_in_fns_without_body)
* [`plugin_as_library`](#plugin_as_library)
* [`private_doc_tests`](#private_doc_tests)
* [`private_in_public`](#private_in_public)
* [`proc_macro_derive_resolution_fallback`](#proc_macro_derive_resolution_fallback)
* [`pub_use_of_private_extern_crate`](#pub_use_of_private_extern_crate)
* [`question_mark_macro_sep`](#question_mark_macro_sep)
* [`renamed_and_removed_lints`](#renamed_and_removed_lints)
* [`safe_extern_statics`](#safe_extern_statics)
* [`safe_packed_borrows`](#safe_packed_borrows)
* [`single_use_lifetimes`](#single_use_lifetimes)
* [`stable_features`](#stable_features)
* [`trivial_bounds`](#trivial_bounds)
* [`trivial_casts`](#trivial_casts)
* [`trivial_numeric_casts`](#trivial_numeric_casts)
* [`type_alias_bounds`](#type_alias_bounds)
* [`tyvar_behind_raw_pointer`](#tyvar_behind_raw_pointer)
* [`unconditional_recursion`](#unconditional_recursion)
* [`unions_with_drop_fields`](#unions_with_drop_fields)
* [`unknown_crate_types`](#unknown_crate_types)
* [`unknown_lints`](#unknown_lints)
* [`unnameable_test_items`](#unnameable_test_items)
* [`unreachable_code`](#unreachable_code)
* [`unreachable_patterns`](#unreachable_patterns)
* [`unreachable_pub`](#unreachable_pub)
* [`unsafe_code`](#unsafe_code)
* [`unstable_features`](#unstable_features)
* [`unstable_name_collisions`](#unstable_name_collisions)
* [`unused_allocation`](#unused_allocation)
* [`unused_assignments`](#unused_assignments)
* [`unused_attributes`](#unused_attributes)
* [`unused_comparisons`](#unused_comparisons)
* [`unused_doc_comments`](#unused_doc_comments)
* [`unused_extern_crates`](#unused_extern_crates)
* [`unused_features`](#unused_features)
* [`unused_import_braces`](#unused_import_braces)
* [`unused_imports`](#unused_imports)
* [`unused_labels`](#unused_labels)
* [`unused_lifetimes`](#unused_lifetimes)
* [`unused_macros`](#unused_macros)
* [`unused_must_use`](#unused_must_use)
* [`unused_mut`](#unused_mut)
* [`unused_parens`](#unused_parens)
* [`unused_qualifications`](#unused_qualifications)
* [`unused_results`](#unused_results)
* [`unused_unsafe`](#unused_unsafe)
* [`unused_variables`](#unused_variables)
* [`variant_size_differences`](#variant_size_differences)
* [`warnings`](#warnings)
* [`where_clauses_object_safety`](#where_clauses_object_safety)
* [`while_true`](#while_true)



## Full details of all lints

#### `absolute_paths_not_starting_with_crate`

| Default | Use | Description | Groups |
|:--------|:----|:------------|:-------|
| allow | deny([`f_i`][f_i]) | fully qualified paths that start with a module name instead of `crate`, `self`, or an extern crate name | [`f_i`][f_i], [`r_c`][r_c] |


#### `ambiguous_associated_items`

| Default | Use | Description | Groups |
|:--------|:----|:------------|:-------|
| warn | deny([`f_i`][f_i]) | ambiguous associated items | [`f_i`][f_i] |


#### `anonymous_parameters`

| Default | Use | Description | Groups |
|:--------|:----|:------------|:-------|
| allow | deny([`f_i`][f_i]) | detects anonymous parameters | [`f_i`][f_i], [`r_c`][r_c] |


#### `bare_trait_objects`

| Default | Use | Description | Groups |
|:--------|:----|:------------|:-------|
| allow | deny([`r_i`][r_i]) | suggest using `dyn Trait` for trait objects | [`r_i`][r_i] |


#### `box_pointers`

| Default | Use | Description | Groups |
|:--------|:----|:------------|:-------|
| allow | default | use of owned (Box type) heap memory | |


#### `const_err`

| Default | Use | Description | Groups |
|:--------|:----|:------------|:-------|
| deny | forbid | constant evaluation detected erroneous expression | |


#### `dead_code`

| Default | Use | Description | Groups |
|:--------|:----|:------------|:-------|
| warn | warn([`u`][u]) | detect unused, unexported items | [`u`][u] |


#### `deprecated`

| Default | Use | Description | Groups |
|:--------|:----|:------------|:-------|
| warn | default | detects use of deprecated items | |


#### `deprecated_in_future`

| Default | Use | Description | Groups |
|:--------|:----|:------------|:-------|
| allow | deny | detects use of items that will be deprecated in a future version | |


#### `duplicate_macro_exports`

| Default | Use | Description | Groups |
|:--------|:----|:------------|:-------|
| deny | forbid | detects duplicate macro exports | [`f_i`][f_i], [`r_c`][r_c] |


#### `elided_lifetimes_in_paths`

| Default | Use | Description | Groups |
|:--------|:----|:------------|:-------|
| allow | deny([`r_i`][r_i]) | hidden lifetime parameters in types are deprecated | [`r_i`][r_i] |


#### `ellipsis_inclusive_range_patterns`

| Default | Use | Description | Groups |
|:--------|:----|:------------|:-------|
| allow | deny([`r_i`][r_i]) | `...` range patterns are deprecated | [`r_i`][r_i] |


#### `exceeding_bitshifts`

| Default | Use | Description | Groups |
|:--------|:----|:------------|:-------|
| deny | forbid | shift exceeds the type's number of bits | |


#### `explicit_outlives_requirements`

| Default | Use | Description | Groups |
|:--------|:----|:------------|:-------|
| allow | deny([`r_i`][r_i]) | outlives requirements can be inferred | [`r_i`][r_i] |


#### `exported_private_dependencies`

| Default | Use | Description | Groups |
|:--------|:----|:------------|:-------|
| warn | default | public interface leaks type from a private dependency | |


#### `ill_formed_attribute_input`

| Default | Use | Description | Groups |
|:--------|:----|:------------|:-------|
| warn | deny([`f_i`][f_i]) | ill-formed attribute inputs that were previously accepted and used in practice | [`f_i`][f_i] |


#### `illegal_floating_point_literal_pattern`

| Default | Use | Description | Groups |
|:--------|:----|:------------|:-------|
| warn | deny([`f_i`][f_i]) | floating-point literals cannot be used in patterns | [`f_i`][f_i] |


#### `improper_ctypes`

| Default | Use | Description | Groups |
|:--------|:----|:------------|:-------|
| warn | default | proper use of libc types in foreign modules | |


#### `incoherent_fundamental_impls`

| Default | Use | Description | Groups |
|:--------|:----|:------------|:-------|
| deny | forbid | potentially-conflicting impls were erroneously allowed | [`f_i`][f_i] |


#### `intra_doc_link_resolution_failure`

| Default | Use | Description | Groups |
|:--------|:----|:------------|:-------|
| warn | default | failures in resolving intra-doc link targets | [`rd`][rd] |


#### `invalid_type_param_default`

| Default | Use | Description | Groups |
|:--------|:----|:------------|:-------|
| deny | forbid | type parameter default erroneously allowed in invalid location | [`f_i`][f_i] |


#### `irrefutable_let_patterns`

| Default | Use | Description | Groups |
|:--------|:----|:------------|:-------|
| warn | default | detects irrefutable patterns in if-let and while-let statements | |


#### `keyword_idents`

| Default | Use | Description | Groups |
|:--------|:----|:------------|:-------|
| allow | deny([`f_i`][f_i]) | detects edition keywords being used as an identifier | [`f_i`][f_i], [`r_c`][r_c] |


#### `late_bound_lifetime_arguments`

| Default | Use | Description | Groups |
|:--------|:----|:------------|:-------|
| warn | deny([`f_i`][f_i]) | detects generic lifetime arguments in path segments with late bound lifetime parameters | [`f_i`][f_i] |


#### `legacy_constructor_visibility`

| Default | Use | Description | Groups |
|:--------|:----|:------------|:-------|
| deny | forbid | detects use of struct constructors that would be invisible with new visibility rules | [`f_i`][f_i] |


#### `legacy_directory_ownership`

| Default | Use | Description | Groups |
|:--------|:----|:------------|:-------|
| deny | forbid | non-inline, non-`#[path]` modules (e.g., `mod foo;`) were erroneously allowed in some files not named `mod.rs` | [`f_i`][f_i] |


#### `macro_expanded_macro_exports_accessed_by_absolute_paths`

| Default | Use | Description | Groups |
|:--------|:----|:------------|:-------|
| deny | forbid | macro-expanded `macro_export` macros from the current crate cannot be referred to by absolute paths | [`f_i`][f_i] |


#### `macro_use_extern_crate`

| Default | Use | Description | Groups |
|:--------|:----|:------------|:-------|
| allow | deny | the `#[macro_use]` attribute is now deprecated in favor of using macros via the module system | |


#### `missing_copy_implementations`

| Default | Use | Description | Groups |
|:--------|:----|:------------|:-------|
| allow | warn | detects potentially-forgotten implementations of `Copy` | |


#### `missing_debug_implementations`

| Default | Use | Description | Groups |
|:--------|:----|:------------|:-------|
| allow | default | detects missing implementations of fmt::Debug | |


#### `missing_doc_code_examples`

| Default | Use | Description | Groups |
|:--------|:----|:------------|:-------|
| allow | default | detects publicly-exported items without code samples in their documentation | [`rd`][rd] |


#### `missing_docs`

| Default | Use | Description | Groups |
|:--------|:----|:------------|:-------|
| allow | warn | detects missing documentation for public members | |


#### `missing_fragment_specifier`

| Default | Use | Description | Groups |
|:--------|:----|:------------|:-------|
| deny | forbid | detects missing fragment specifiers in unused `macro_rules!` patterns | [`f_i`][f_i] |


#### `mutable_transmutes`

| Default | Use | Description | Groups |
|:--------|:----|:------------|:-------|
| deny | forbid | mutating transmuted &mut T from &T may cause undefined behavior | |


#### `nested_impl_trait`

| Default | Use | Description | Groups |
|:--------|:----|:------------|:-------|
| warn | deny([`f_i`][f_i]) | nested occurrence of `impl Trait` type | [`f_i`][f_i] |


#### `no_mangle_const_items`

| Default | Use | Description | Groups |
|:--------|:----|:------------|:-------|
| deny | forbid | const items will not have their symbols exported | |


#### `no_mangle_generic_items`

| Default | Use | Description | Groups |
|:--------|:----|:------------|:-------|
| warn | default | generic items must be mangled | |


#### `non_camel_case_types`

| Default | Use | Description | Groups |
|:--------|:----|:------------|:-------|
| warn | deny([`n_s`][n_s]) | types, variants, traits and type parameters should have camel case names | [`n_s`][n_s] |


#### `non_shorthand_field_patterns`

| Default | Use | Description | Groups |
|:--------|:----|:------------|:-------|
| warn | default | using `Struct { x: x }` instead of `Struct { x }` in a pattern | |


#### `non_snake_case`

| Default | Use | Description | Groups |
|:--------|:----|:------------|:-------|
| warn | deny([`n_s`][n_s]) | variables, methods, functions, lifetime parameters and modules should have snake case names | [`n_s`][n_s] |


#### `non_upper_case_globals`

| Default | Use | Description | Groups |
|:--------|:----|:------------|:-------|
| warn | deny([`n_s`][n_s]) | static constants should have uppercase identifiers | [`n_s`][n_s] |


#### `order_dependent_trait_objects`

| Default | Use | Description | Groups |
|:--------|:----|:------------|:-------|
| deny | forbid | trait-object types were treated as different depending on marker-trait order | [`f_i`][f_i] |


#### `overflowing_literals`

| Default | Use | Description | Groups |
|:--------|:----|:------------|:-------|
| deny | forbid | literal out of range for its type | |


#### `parenthesized_params_in_types_and_modules`

| Default | Use | Description | Groups |
|:--------|:----|:------------|:-------|
| deny | forbid | detects parenthesized generic parameters in type and module names | [`f_i`][f_i] |


#### `path_statements`

| Default | Use | Description | Groups |
|:--------|:----|:------------|:-------|
| warn | warn([`u`][u]) | path statements with no effect | [`u`][u] |


#### `patterns_in_fns_without_body`

| Default | Use | Description | Groups |
|:--------|:----|:------------|:-------|
| warn | deny([`f_i`][f_i]) | patterns in functions without body were erroneously allowed | [`f_i`][f_i] |


#### `plugin_as_library`

| Default | Use | Description | Groups |
|:--------|:----|:------------|:-------|
| warn | default | compiler plugin used as ordinary library in non-plugin crate | |


#### `private_doc_tests`

| Default | Use | Description | Groups |
|:--------|:----|:------------|:-------|
| allow | default | detects code samples in docs of private items not documented by rustdoc | [`rd`][rd] |


#### `private_in_public`

| Default | Use | Description | Groups |
|:--------|:----|:------------|:-------|
| warn | deny([`f_i`][f_i]) | detect private items in public interfaces not caught by the old implementation | [`f_i`][f_i] |


#### `proc_macro_derive_resolution_fallback`

| Default | Use | Description | Groups |
|:--------|:----|:------------|:-------|
| warn | deny([`f_i`][f_i]) | detects proc macro derives using inaccessible names from parent modules | [`f_i`][f_i] |


#### `pub_use_of_private_extern_crate`

| Default | Use | Description | Groups |
|:--------|:----|:------------|:-------|
| deny | forbid | detect public re-exports of private extern crates | [`f_i`][f_i] |


#### `question_mark_macro_sep`

| Default | Use | Description | Groups |
|:--------|:----|:------------|:-------|
| allow | deny([`f_i`][f_i]) | detects the use of `?` as a macro separator | [`f_i`][f_i], [`r_c`][r_c] |


#### `renamed_and_removed_lints`

| Default | Use | Description | Groups |
|:--------|:----|:------------|:-------|
| warn | default | lints that have been renamed or removed | |


#### `safe_extern_statics`

| Default | Use | Description | Groups |
|:--------|:----|:------------|:-------|
| deny | forbid | safe access to extern statics was erroneously allowed | [`f_i`][f_i] |


#### `safe_packed_borrows`

| Default | Use | Description | Groups |
|:--------|:----|:------------|:-------|
| warn | deny([`f_i`][f_i]) | safe borrows of fields of packed structs were was erroneously allowed | [`f_i`][f_i] |


#### `single_use_lifetimes`

| Default | Use | Description | Groups |
|:--------|:----|:------------|:-------|
| allow | deny | detects lifetime parameters that are only used once | |


#### `stable_features`

| Default | Use | Description | Groups |
|:--------|:----|:------------|:-------|
| warn | default | stable features found in #[feature] directive | |


#### `trivial_bounds`

| Default | Use | Description | Groups |
|:--------|:----|:------------|:-------|
| warn | default | these bounds don't depend on an type parameters | |


#### `trivial_casts`

| Default | Use | Description | Groups |
|:--------|:----|:------------|:-------|
| allow | deny | detects trivial casts which could be removed | |


#### `trivial_numeric_casts`

| Default | Use | Description | Groups |
|:--------|:----|:------------|:-------|
| allow | deny | detects trivial casts of numeric types which could be removed | |


#### `type_alias_bounds`

| Default | Use | Description | Groups |
|:--------|:----|:------------|:-------|
| warn | default | bounds in type aliases are not enforced | |


#### `tyvar_behind_raw_pointer`

| Default | Use | Description | Groups |
|:--------|:----|:------------|:-------|
| warn | deny([`f_i`][f_i]) | raw pointer to an inference variable | [`f_i`][f_i], [`r_c`][r_c] |


#### `unconditional_recursion`

| Default | Use | Description | Groups |
|:--------|:----|:------------|:-------|
| warn | default | functions that cannot return without calling themselves | |


#### `unions_with_drop_fields`

| Default | Use | Description | Groups |
|:--------|:----|:------------|:-------|
| warn | default | use of unions that contain fields with possibly non-trivial drop code | |


#### `unknown_crate_types`

| Default | Use | Description | Groups |
|:--------|:----|:------------|:-------|
| deny | forbid | unknown crate type found in #[crate_type] directive | |


#### `unknown_lints`

| Default | Use | Description | Groups |
|:--------|:----|:------------|:-------|
| warn | default | unrecognized lint attribute | |


#### `unnameable_test_items`

| Default | Use | Description | Groups |
|:--------|:----|:------------|:-------|
| warn | default | detects an item that cannot be named being marked as #[test_case] | |


#### `unreachable_code`

| Default | Use | Description | Groups |
|:--------|:----|:------------|:-------|
| warn | warn([`u`][u]) | detects unreachable code paths | [`u`][u] |


#### `unreachable_patterns`

| Default | Use | Description | Groups |
|:--------|:----|:------------|:-------|
| warn | warn([`u`][u]) | detects unreachable patterns | [`u`][u] |


#### `unreachable_pub`

| Default | Use | Description | Groups |
|:--------|:----|:------------|:-------|
| allow | deny | `pub` items not reachable from crate root | |


#### `unsafe_code`

| Default | Use | Description | Groups |
|:--------|:----|:------------|:-------|
| allow | deny | usage of `unsafe` code | |


#### `unstable_features`

| Default | Use | Description | Groups |
|:--------|:----|:------------|:-------|
| allow | deny | enabling unstable features (deprecated. do not use) | |


#### `unstable_name_collisions`

| Default | Use | Description | Groups |
|:--------|:----|:------------|:-------|
| warn | deny([`f_i`][f_i]) | detects name collision with an existing but unstable method | [`f_i`][f_i] |


#### `unused_allocation`

| Default | Use | Description | Groups |
|:--------|:----|:------------|:-------|
| warn | warn([`u`][u]) | detects unnecessary allocations that can be eliminated | [`u`][u] |


#### `unused_assignments`

| Default | Use | Description | Groups |
|:--------|:----|:------------|:-------|
| warn | warn([`u`][u]) | detect assignments that will never be read | [`u`][u] |


#### `unused_attributes`

| Default | Use | Description | Groups |
|:--------|:----|:------------|:-------|
| warn | warn([`u`][u]) | detects attributes that were not used by the compiler | [`u`][u] |


#### `unused_comparisons`

| Default | Use | Description | Groups |
|:--------|:----|:------------|:-------|
| warn | default | comparisons made useless by limits of the types involved | |


#### `unused_doc_comments`

| Default | Use | Description | Groups |
|:--------|:----|:------------|:-------|
| warn | warn([`u`][u]) | detects doc comments that aren't used by rustdoc | [`u`][u] |


#### `unused_extern_crates`

| Default | Use | Description | Groups |
|:--------|:----|:------------|:-------|
| allow | deny([`r_i`][r_i]) | extern crates that are never used | [`r_i`][r_i], [`u`][u] |


#### `unused_features`

| Default | Use | Description | Groups |
|:--------|:----|:------------|:-------|
| warn | warn([`u`][u]) | unused features found in crate-level #[feature] directives | [`u`][u] |


#### `unused_import_braces`

| Default | Use | Description | Groups |
|:--------|:----|:------------|:-------|
| allow | deny | unnecessary braces around an imported item | |


#### `unused_imports`

| Default | Use | Description | Groups |
|:--------|:----|:------------|:-------|
| warn | warn([`u`][u]) | imports that are never used | [`u`][u] |


#### `unused_labels`

| Default | Use | Description | Groups |
|:--------|:----|:------------|:-------|
| allow | warn([`u`][u]) | detects labels that are never used | [`u`][u] |


#### `unused_lifetimes`

| Default | Use | Description | Groups |
|:--------|:----|:------------|:-------|
| allow | deny | detects lifetime parameters that are never used | |


#### `unused_macros`

| Default | Use | Description | Groups |
|:--------|:----|:------------|:-------|
| warn | warn([`u`][u]) | detects macros that were not used | [`u`][u] |


#### `unused_must_use`

| Default | Use | Description | Groups |
|:--------|:----|:------------|:-------|
| warn | warn([`u`][u]) | unused result of a type flagged as #[must_use] | [`u`][u] |


#### `unused_mut`

| Default | Use | Description | Groups |
|:--------|:----|:------------|:-------|
| warn | warn([`u`][u]) | detect mut variables which don't need to be mutable | [`u`][u] |


#### `unused_parens`

| Default | Use | Description | Groups |
|:--------|:----|:------------|:-------|
| warn | warn([`u`][u]) | `if`, `match`, `while` and `return` do not need parentheses | [`u`][u] |


#### `unused_qualifications`

| Default | Use | Description | Groups |
|:--------|:----|:------------|:-------|
| allow | deny | detects unnecessarily qualified names | |


#### `unused_results`

| Default | Use | Description | Groups |
|:--------|:----|:------------|:-------|
| allow | deny | unused result of an expression in a statement | |


#### `unused_unsafe`

| Default | Use | Description | Groups |
|:--------|:----|:------------|:-------|
| warn | warn([`u`][u]) | unnecessary use of an `unsafe` block | [`u`][u] |


#### `unused_variables`

| Default | Use | Description | Groups |
|:--------|:----|:------------|:-------|
| warn | warn([`u`][u]) | detect variables which are not used in any way | [`u`][u] |


#### `variant_size_differences`

| Default | Use | Description | Groups |
|:--------|:----|:------------|:-------|
| allow | default | detects enums with widely varying variant sizes | |


#### `warnings`

| Default | Use | Description | Groups |
|:--------|:----|:------------|:-------|
| warn | deny | mass-change the level for lints which produce warnings | |


#### `where_clauses_object_safety`

| Default | Use | Description | Groups |
|:--------|:----|:------------|:-------|
| warn | deny([`f_i`][f_i]) | checks the object safety of where clauses | [`f_i`][f_i] |


#### `while_true`

| Default | Use | Description | Groups |
|:--------|:----|:------------|:-------|
| warn | default | suggest using `loop { }` instead of `while true { }` | |


---

## Lint groups

### `future_incompatible (f_i)`

* [`absolute_paths_not_starting_with_crate`](#absolute_paths_not_starting_with_crate)
* [`ambiguous_associated_items`](#ambiguous_associated_items)
* [`anonymous_parameters`](#anonymous_parameters)
* [`duplicate_macro_exports`](#duplicate_macro_exports)
* [`ill_formed_attribute_input`](#ill_formed_attribute_input)
* [`illegal_floating_point_literal_pattern`](#illegal_floating_point_literal_pattern)
* [`incoherent_fundamental_impls`](#incoherent_fundamental_impls)
* [`invalid_type_param_default`](#invalid_type_param_default)
* [`keyword_idents`](#keyword_idents)
* [`late_bound_lifetime_arguments`](#late_bound_lifetime_arguments)
* [`legacy_constructor_visibility`](#legacy_constructor_visibility)
* [`legacy_directory_ownership`](#legacy_directory_ownership)
* [`macro_expanded_macro_exports_accessed_by_absolute_paths`](#macro_expanded_macro_exports_accessed_by_absolute_paths)
* [`missing_fragment_specifier`](#missing_fragment_specifier)
* [`nested_impl_trait`](#nested_impl_trait)
* [`order_dependent_trait_objects`](#order_dependent_trait_objects)
* [`parenthesized_params_in_types_and_modules`](#parenthesized_params_in_types_and_modules)
* [`patterns_in_fns_without_body`](#patterns_in_fns_without_body)
* [`private_in_public`](#private_in_public)
* [`proc_macro_derive_resolution_fallback`](#proc_macro_derive_resolution_fallback)
* [`pub_use_of_private_extern_crate`](#pub_use_of_private_extern_crate)
* [`question_mark_macro_sep`](#question_mark_macro_sep)
* [`safe_extern_statics`](#safe_extern_statics)
* [`safe_packed_borrows`](#safe_packed_borrows)
* [`tyvar_behind_raw_pointer`](#tyvar_behind_raw_pointer)
* [`unstable_name_collisions`](#unstable_name_collisions)
* [`where_clauses_object_safety`](#where_clauses_object_safety)


### `nonstandard_style (n_s)`

* [`non_camel_case_types`](#non_camel_case_types)
* [`non_snake_case`](#non_snake_case)
* [`non_upper_case_globals`](#non_upper_case_globals)


### `rust_2018_compatibility (r_c)` (subset of `future_incompatible`)

* [`absolute_paths_not_starting_with_crate`](#absolute_paths_not_starting_with_crate)
* [`anonymous_parameters`](#anonymous_parameters)
* [`duplicate_macro_exports`](#duplicate_macro_exports)
* [`keyword_idents`](#keyword_idents)
* [`question_mark_macro_sep`](#question_mark_macro_sep)
* [`tyvar_behind_raw_pointer`](#tyvar_behind_raw_pointer)


### `rust_2018_idioms (r_i)`

* [`bare_trait_objects`](#bare_trait_objects)
* [`elided_lifetimes_in_paths`](#elided_lifetimes_in_paths)
* [`ellipsis_inclusive_range_patterns`](#ellipsis_inclusive_range_patterns)
* [`explicit_outlives_requirements`](#explicit_outlives_requirements)
* [`unused_extern_crates`](#unused_extern_crates)


### `rustdoc (rd)`

* [`intra_doc_link_resolution_failure`](#intra_doc_link_resolution_failure)
* [`missing_doc_code_examples`](#missing_doc_code_examples)
* [`private_doc_tests`](#private_doc_tests)


### `unused (u)`

* [`dead_code`](#dead_code)
* [`path_statements`](#path_statements)
* [`unreachable_code`](#unreachable_code)
* [`unreachable_patterns`](#unreachable_patterns)
* [`unused_allocation`](#unused_allocation)
* [`unused_assignments`](#unused_assignments)
* [`unused_attributes`](#unused_attributes)
* [`unused_doc_comments`](#unused_doc_comments)
* [`unused_extern_crates`](#unused_extern_crates)
* [`unused_features`](#unused_features)
* [`unused_imports`](#unused_imports)
* [`unused_labels`](#unused_labels)
* [`unused_macros`](#unused_macros)
* [`unused_must_use`](#unused_must_use)
* [`unused_mut`](#unused_mut)
* [`unused_parens`](#unused_parens)
* [`unused_unsafe`](#unused_unsafe)
* [`unused_variables`](#unused_variables)



[f_i]: #future_incompatible-f_i "future_incompatible"
[n_s]: #nonstandard_style-n_s "nonstandard_style"
[r_c]: #rust_2018_compatibility-r_c-subset-of-future_incompatible "rust_2018_compatibility"
[r_i]: #rust_2018_idioms-r_i "rust_2018_idioms"
[rd]: #rustdoc-rd "rustdoc"
[u]: #unused-u "unused"
