use crate::{
    error::{BLOCKED_SYNTAX_ERROR, PARSE_ERROR, VerificationError},
    node_location::NodeLocation,
    policy::SyntaxPolicy,
    verifier_map::VerifierMap,
};
use std::collections::{BTreeMap, BTreeSet};

pub struct SyntaxVerifier {
    verifiers: VerifierMap,
    blocked_node_locations: BTreeMap<&'static str, BTreeSet<NodeLocation>>,
}

impl SyntaxVerifier {
    pub fn new(verifiers: VerifierMap) -> Self {
        Self {
            verifiers,
            blocked_node_locations: BTreeMap::new(),
        }
    }
    pub fn verify(&mut self, code: &str) -> Result<(), VerificationError> {
        match syn::parse_file(code) {
            Err(e) => Err(VerificationError {
                exit_code: PARSE_ERROR,
                errors: vec![format!("Error when parsing code: {}", e)],
            }),
            Ok(ast) => {
                syn::visit::Visit::visit_file(self, &ast);
                // let blocked_node_locations = self.verifiers.blocked_node_locations();
                if self.blocked_node_locations.is_empty() {
                    Ok(())
                } else {
                    let errors = self
                        .blocked_node_locations
                        .iter()
                        .map(|(&key, key_locations)| {
                            let error_locations_string = key_locations
                                .iter()
                                .map(|location| location.to_string())
                                .collect::<Vec<_>>()
                                .join(",")
                                .to_string();
                            format!(
                                "{:?} at {} locations: {}",
                                key,
                                key_locations.len(),
                                error_locations_string
                            )
                        })
                        .collect();
                    Err(VerificationError {
                        exit_code: BLOCKED_SYNTAX_ERROR,
                        errors,
                    })
                }
            }
        }
    }

    fn verify_node<N: syn::spanned::Spanned + 'static>(&mut self, node: &N) {
        let policy = self.verifiers.verify_node(node);
        if let SyntaxPolicy::Blocked(blocked_locations) = &policy {
            let node_name = self.verifiers.node_type_name::<N>();
            let locations = self.blocked_node_locations.entry(node_name).or_default();
            for location in &blocked_locations.locations {
                locations.insert(location.clone());
            }
        }
    }
}

impl syn::visit::Visit<'_> for SyntaxVerifier {
    fn visit_abi(&mut self, node: &syn::Abi) {
        self.verify_node(node);
        syn::visit::visit_abi(self, node);
    }

    fn visit_angle_bracketed_generic_arguments(
        &mut self,
        node: &syn::AngleBracketedGenericArguments,
    ) {
        self.verify_node(node);
        syn::visit::visit_angle_bracketed_generic_arguments(self, node);
    }

    fn visit_arm(&mut self, node: &syn::Arm) {
        self.verify_node(node);
        syn::visit::visit_arm(self, node);
    }

    fn visit_assoc_const(&mut self, node: &syn::AssocConst) {
        self.verify_node(node);
        syn::visit::visit_assoc_const(self, node);
    }

    fn visit_assoc_type(&mut self, node: &syn::AssocType) {
        self.verify_node(node);
        syn::visit::visit_assoc_type(self, node);
    }

    fn visit_attribute(&mut self, node: &syn::Attribute) {
        self.verify_node(node);
        syn::visit::visit_attribute(self, node);
    }

    fn visit_bare_fn_arg(&mut self, node: &syn::BareFnArg) {
        self.verify_node(node);
        syn::visit::visit_bare_fn_arg(self, node);
    }

    fn visit_bare_variadic(&mut self, node: &syn::BareVariadic) {
        self.verify_node(node);
        syn::visit::visit_bare_variadic(self, node);
    }

    fn visit_bin_op(&mut self, node: &syn::BinOp) {
        self.verify_node(node);
        syn::visit::visit_bin_op(self, node);
    }

    fn visit_block(&mut self, node: &syn::Block) {
        self.verify_node(node);
        syn::visit::visit_block(self, node);
    }

    fn visit_bound_lifetimes(&mut self, node: &syn::BoundLifetimes) {
        self.verify_node(node);
        syn::visit::visit_bound_lifetimes(self, node);
    }

    fn visit_captured_param(&mut self, node: &syn::CapturedParam) {
        self.verify_node(node);
        syn::visit::visit_captured_param(self, node);
    }

    fn visit_const_param(&mut self, node: &syn::ConstParam) {
        self.verify_node(node);
        syn::visit::visit_const_param(self, node);
    }

    fn visit_constraint(&mut self, node: &syn::Constraint) {
        self.verify_node(node);
        syn::visit::visit_constraint(self, node);
    }

    fn visit_derive_input(&mut self, node: &syn::DeriveInput) {
        self.verify_node(node);
        syn::visit::visit_derive_input(self, node);
    }

    fn visit_expr(&mut self, node: &syn::Expr) {
        self.verify_node(node);
        syn::visit::visit_expr(self, node);
    }

    fn visit_expr_array(&mut self, node: &syn::ExprArray) {
        self.verify_node(node);
        syn::visit::visit_expr_array(self, node);
    }

    fn visit_expr_assign(&mut self, node: &syn::ExprAssign) {
        self.verify_node(node);
        syn::visit::visit_expr_assign(self, node);
    }

    fn visit_expr_async(&mut self, node: &syn::ExprAsync) {
        self.verify_node(node);
        syn::visit::visit_expr_async(self, node);
    }

    fn visit_expr_await(&mut self, node: &syn::ExprAwait) {
        self.verify_node(node);
        syn::visit::visit_expr_await(self, node);
    }

    fn visit_expr_binary(&mut self, node: &syn::ExprBinary) {
        self.verify_node(node);
        syn::visit::visit_expr_binary(self, node);
    }

    fn visit_expr_block(&mut self, node: &syn::ExprBlock) {
        self.verify_node(node);
        syn::visit::visit_expr_block(self, node);
    }

    fn visit_expr_break(&mut self, node: &syn::ExprBreak) {
        self.verify_node(node);
        syn::visit::visit_expr_break(self, node);
    }

    fn visit_expr_call(&mut self, node: &syn::ExprCall) {
        self.verify_node(node);
        syn::visit::visit_expr_call(self, node);
    }

    fn visit_expr_cast(&mut self, node: &syn::ExprCast) {
        self.verify_node(node);
        syn::visit::visit_expr_cast(self, node);
    }

    fn visit_expr_closure(&mut self, node: &syn::ExprClosure) {
        self.verify_node(node);
        syn::visit::visit_expr_closure(self, node);
    }

    fn visit_expr_const(&mut self, node: &syn::ExprConst) {
        self.verify_node(node);
        syn::visit::visit_expr_const(self, node);
    }

    fn visit_expr_continue(&mut self, node: &syn::ExprContinue) {
        self.verify_node(node);
        syn::visit::visit_expr_continue(self, node);
    }

    fn visit_expr_field(&mut self, node: &syn::ExprField) {
        self.verify_node(node);
        syn::visit::visit_expr_field(self, node);
    }

    fn visit_expr_for_loop(&mut self, node: &syn::ExprForLoop) {
        self.verify_node(node);
        syn::visit::visit_expr_for_loop(self, node);
    }

    fn visit_expr_group(&mut self, node: &syn::ExprGroup) {
        self.verify_node(node);
        syn::visit::visit_expr_group(self, node);
    }

    fn visit_expr_if(&mut self, node: &syn::ExprIf) {
        self.verify_node(node);
        syn::visit::visit_expr_if(self, node);
    }

    fn visit_expr_index(&mut self, node: &syn::ExprIndex) {
        self.verify_node(node);
        syn::visit::visit_expr_index(self, node);
    }

    fn visit_expr_infer(&mut self, node: &syn::ExprInfer) {
        self.verify_node(node);
        syn::visit::visit_expr_infer(self, node);
    }

    fn visit_expr_let(&mut self, node: &syn::ExprLet) {
        self.verify_node(node);
        syn::visit::visit_expr_let(self, node);
    }

    fn visit_expr_lit(&mut self, node: &syn::ExprLit) {
        self.verify_node(node);
        syn::visit::visit_expr_lit(self, node);
    }

    fn visit_expr_loop(&mut self, node: &syn::ExprLoop) {
        self.verify_node(node);
        syn::visit::visit_expr_loop(self, node);
    }

    fn visit_expr_macro(&mut self, node: &syn::ExprMacro) {
        self.verify_node(node);
        syn::visit::visit_expr_macro(self, node);
    }

    fn visit_expr_match(&mut self, node: &syn::ExprMatch) {
        self.verify_node(node);
        syn::visit::visit_expr_match(self, node);
    }

    fn visit_expr_method_call(&mut self, node: &syn::ExprMethodCall) {
        self.verify_node(node);
        syn::visit::visit_expr_method_call(self, node);
    }

    fn visit_expr_paren(&mut self, node: &syn::ExprParen) {
        self.verify_node(node);
        syn::visit::visit_expr_paren(self, node);
    }

    fn visit_expr_path(&mut self, node: &syn::ExprPath) {
        self.verify_node(node);
        syn::visit::visit_expr_path(self, node);
    }

    fn visit_expr_range(&mut self, node: &syn::ExprRange) {
        self.verify_node(node);
        syn::visit::visit_expr_range(self, node);
    }

    fn visit_expr_raw_addr(&mut self, node: &syn::ExprRawAddr) {
        self.verify_node(node);
        syn::visit::visit_expr_raw_addr(self, node);
    }

    fn visit_expr_reference(&mut self, node: &syn::ExprReference) {
        self.verify_node(node);
        syn::visit::visit_expr_reference(self, node);
    }

    fn visit_expr_repeat(&mut self, node: &syn::ExprRepeat) {
        self.verify_node(node);
        syn::visit::visit_expr_repeat(self, node);
    }

    fn visit_expr_return(&mut self, node: &syn::ExprReturn) {
        self.verify_node(node);
        syn::visit::visit_expr_return(self, node);
    }

    fn visit_expr_struct(&mut self, node: &syn::ExprStruct) {
        self.verify_node(node);
        syn::visit::visit_expr_struct(self, node);
    }

    fn visit_expr_try(&mut self, node: &syn::ExprTry) {
        self.verify_node(node);
        syn::visit::visit_expr_try(self, node);
    }

    fn visit_expr_try_block(&mut self, node: &syn::ExprTryBlock) {
        self.verify_node(node);
        syn::visit::visit_expr_try_block(self, node);
    }

    fn visit_expr_tuple(&mut self, node: &syn::ExprTuple) {
        self.verify_node(node);
        syn::visit::visit_expr_tuple(self, node);
    }

    fn visit_expr_unary(&mut self, node: &syn::ExprUnary) {
        self.verify_node(node);
        syn::visit::visit_expr_unary(self, node);
    }

    fn visit_expr_unsafe(&mut self, node: &syn::ExprUnsafe) {
        self.verify_node(node);
        syn::visit::visit_expr_unsafe(self, node);
    }

    fn visit_expr_while(&mut self, node: &syn::ExprWhile) {
        self.verify_node(node);
        syn::visit::visit_expr_while(self, node);
    }

    fn visit_expr_yield(&mut self, node: &syn::ExprYield) {
        self.verify_node(node);
        syn::visit::visit_expr_yield(self, node);
    }

    fn visit_field(&mut self, node: &syn::Field) {
        self.verify_node(node);
        syn::visit::visit_field(self, node);
    }

    fn visit_field_pat(&mut self, node: &syn::FieldPat) {
        self.verify_node(node);
        syn::visit::visit_field_pat(self, node);
    }

    fn visit_field_value(&mut self, node: &syn::FieldValue) {
        self.verify_node(node);
        syn::visit::visit_field_value(self, node);
    }

    fn visit_fields(&mut self, node: &syn::Fields) {
        self.verify_node(node);
        syn::visit::visit_fields(self, node);
    }

    fn visit_fields_named(&mut self, node: &syn::FieldsNamed) {
        self.verify_node(node);
        syn::visit::visit_fields_named(self, node);
    }

    fn visit_fields_unnamed(&mut self, node: &syn::FieldsUnnamed) {
        self.verify_node(node);
        syn::visit::visit_fields_unnamed(self, node);
    }

    fn visit_file(&mut self, node: &syn::File) {
        self.verify_node(node);
        syn::visit::visit_file(self, node);
    }

    fn visit_fn_arg(&mut self, node: &syn::FnArg) {
        self.verify_node(node);
        syn::visit::visit_fn_arg(self, node);
    }

    fn visit_foreign_item(&mut self, node: &syn::ForeignItem) {
        self.verify_node(node);
        syn::visit::visit_foreign_item(self, node);
    }

    fn visit_foreign_item_fn(&mut self, node: &syn::ForeignItemFn) {
        self.verify_node(node);
        syn::visit::visit_foreign_item_fn(self, node);
    }

    fn visit_foreign_item_macro(&mut self, node: &syn::ForeignItemMacro) {
        self.verify_node(node);
        syn::visit::visit_foreign_item_macro(self, node);
    }

    fn visit_foreign_item_static(&mut self, node: &syn::ForeignItemStatic) {
        self.verify_node(node);
        syn::visit::visit_foreign_item_static(self, node);
    }

    fn visit_foreign_item_type(&mut self, node: &syn::ForeignItemType) {
        self.verify_node(node);
        syn::visit::visit_foreign_item_type(self, node);
    }

    fn visit_generic_argument(&mut self, node: &syn::GenericArgument) {
        self.verify_node(node);
        syn::visit::visit_generic_argument(self, node);
    }

    fn visit_generic_param(&mut self, node: &syn::GenericParam) {
        self.verify_node(node);
        syn::visit::visit_generic_param(self, node);
    }

    fn visit_generics(&mut self, node: &syn::Generics) {
        self.verify_node(node);
        syn::visit::visit_generics(self, node);
    }

    fn visit_ident(&mut self, node: &proc_macro2::Ident) {
        self.verify_node(node);
        syn::visit::visit_ident(self, node);
    }

    fn visit_impl_item(&mut self, node: &syn::ImplItem) {
        self.verify_node(node);
        syn::visit::visit_impl_item(self, node);
    }

    fn visit_impl_item_const(&mut self, node: &syn::ImplItemConst) {
        self.verify_node(node);
        syn::visit::visit_impl_item_const(self, node);
    }

    fn visit_impl_item_fn(&mut self, node: &syn::ImplItemFn) {
        self.verify_node(node);
        syn::visit::visit_impl_item_fn(self, node);
    }

    fn visit_impl_item_macro(&mut self, node: &syn::ImplItemMacro) {
        self.verify_node(node);
        syn::visit::visit_impl_item_macro(self, node);
    }

    fn visit_impl_item_type(&mut self, node: &syn::ImplItemType) {
        self.verify_node(node);
        syn::visit::visit_impl_item_type(self, node);
    }

    fn visit_index(&mut self, node: &syn::Index) {
        self.verify_node(node);
        syn::visit::visit_index(self, node);
    }

    fn visit_item(&mut self, node: &syn::Item) {
        self.verify_node(node);
        syn::visit::visit_item(self, node);
    }

    fn visit_item_const(&mut self, node: &syn::ItemConst) {
        self.verify_node(node);
        syn::visit::visit_item_const(self, node);
    }

    fn visit_item_enum(&mut self, node: &syn::ItemEnum) {
        self.verify_node(node);
        syn::visit::visit_item_enum(self, node);
    }

    fn visit_item_extern_crate(&mut self, node: &syn::ItemExternCrate) {
        self.verify_node(node);
        syn::visit::visit_item_extern_crate(self, node);
    }

    fn visit_item_fn(&mut self, node: &syn::ItemFn) {
        self.verify_node(node);
        syn::visit::visit_item_fn(self, node);
    }

    fn visit_item_foreign_mod(&mut self, node: &syn::ItemForeignMod) {
        self.verify_node(node);
        syn::visit::visit_item_foreign_mod(self, node);
    }

    fn visit_item_impl(&mut self, node: &syn::ItemImpl) {
        self.verify_node(node);
        syn::visit::visit_item_impl(self, node);
    }

    fn visit_item_macro(&mut self, node: &syn::ItemMacro) {
        self.verify_node(node);
        syn::visit::visit_item_macro(self, node);
    }

    fn visit_item_mod(&mut self, node: &syn::ItemMod) {
        self.verify_node(node);
        syn::visit::visit_item_mod(self, node);
    }

    fn visit_item_static(&mut self, node: &syn::ItemStatic) {
        self.verify_node(node);
        syn::visit::visit_item_static(self, node);
    }

    fn visit_item_struct(&mut self, node: &syn::ItemStruct) {
        self.verify_node(node);
        syn::visit::visit_item_struct(self, node);
    }

    fn visit_item_trait(&mut self, node: &syn::ItemTrait) {
        self.verify_node(node);
        syn::visit::visit_item_trait(self, node);
    }

    fn visit_item_trait_alias(&mut self, node: &syn::ItemTraitAlias) {
        self.verify_node(node);
        syn::visit::visit_item_trait_alias(self, node);
    }

    fn visit_item_type(&mut self, node: &syn::ItemType) {
        self.verify_node(node);
        syn::visit::visit_item_type(self, node);
    }

    fn visit_item_union(&mut self, node: &syn::ItemUnion) {
        self.verify_node(node);
        syn::visit::visit_item_union(self, node);
    }

    fn visit_item_use(&mut self, node: &syn::ItemUse) {
        self.verify_node(node);
        syn::visit::visit_item_use(self, node);
    }

    fn visit_label(&mut self, node: &syn::Label) {
        self.verify_node(node);
        syn::visit::visit_label(self, node);
    }

    fn visit_lifetime(&mut self, node: &syn::Lifetime) {
        self.verify_node(node);
        syn::visit::visit_lifetime(self, node);
    }

    fn visit_lifetime_param(&mut self, node: &syn::LifetimeParam) {
        self.verify_node(node);
        syn::visit::visit_lifetime_param(self, node);
    }

    fn visit_lit(&mut self, node: &syn::Lit) {
        self.verify_node(node);
        syn::visit::visit_lit(self, node);
    }

    fn visit_lit_bool(&mut self, node: &syn::LitBool) {
        self.verify_node(node);
        syn::visit::visit_lit_bool(self, node);
    }

    fn visit_lit_byte(&mut self, node: &syn::LitByte) {
        self.verify_node(node);
        syn::visit::visit_lit_byte(self, node);
    }

    fn visit_lit_byte_str(&mut self, node: &syn::LitByteStr) {
        self.verify_node(node);
        syn::visit::visit_lit_byte_str(self, node);
    }

    fn visit_lit_cstr(&mut self, node: &syn::LitCStr) {
        self.verify_node(node);
        syn::visit::visit_lit_cstr(self, node);
    }

    fn visit_lit_char(&mut self, node: &syn::LitChar) {
        self.verify_node(node);
        syn::visit::visit_lit_char(self, node);
    }

    fn visit_lit_float(&mut self, node: &syn::LitFloat) {
        self.verify_node(node);
        syn::visit::visit_lit_float(self, node);
    }

    fn visit_lit_int(&mut self, node: &syn::LitInt) {
        self.verify_node(node);
        syn::visit::visit_lit_int(self, node);
    }

    fn visit_lit_str(&mut self, node: &syn::LitStr) {
        self.verify_node(node);
        syn::visit::visit_lit_str(self, node);
    }

    fn visit_local(&mut self, node: &syn::Local) {
        self.verify_node(node);
        syn::visit::visit_local(self, node);
    }

    fn visit_macro(&mut self, node: &syn::Macro) {
        self.verify_node(node);
        syn::visit::visit_macro(self, node);
    }

    fn visit_member(&mut self, node: &syn::Member) {
        self.verify_node(node);
        syn::visit::visit_member(self, node);
    }

    fn visit_meta(&mut self, node: &syn::Meta) {
        self.verify_node(node);
        syn::visit::visit_meta(self, node);
    }

    fn visit_meta_list(&mut self, node: &syn::MetaList) {
        self.verify_node(node);
        syn::visit::visit_meta_list(self, node);
    }

    fn visit_meta_name_value(&mut self, node: &syn::MetaNameValue) {
        self.verify_node(node);
        syn::visit::visit_meta_name_value(self, node);
    }

    fn visit_parenthesized_generic_arguments(&mut self, node: &syn::ParenthesizedGenericArguments) {
        self.verify_node(node);
        syn::visit::visit_parenthesized_generic_arguments(self, node);
    }

    fn visit_pat(&mut self, node: &syn::Pat) {
        self.verify_node(node);
        syn::visit::visit_pat(self, node);
    }

    fn visit_pat_ident(&mut self, node: &syn::PatIdent) {
        self.verify_node(node);
        syn::visit::visit_pat_ident(self, node);
    }

    fn visit_pat_or(&mut self, node: &syn::PatOr) {
        self.verify_node(node);
        syn::visit::visit_pat_or(self, node);
    }

    fn visit_pat_paren(&mut self, node: &syn::PatParen) {
        self.verify_node(node);
        syn::visit::visit_pat_paren(self, node);
    }

    fn visit_pat_reference(&mut self, node: &syn::PatReference) {
        self.verify_node(node);
        syn::visit::visit_pat_reference(self, node);
    }

    fn visit_pat_rest(&mut self, node: &syn::PatRest) {
        self.verify_node(node);
        syn::visit::visit_pat_rest(self, node);
    }

    fn visit_pat_slice(&mut self, node: &syn::PatSlice) {
        self.verify_node(node);
        syn::visit::visit_pat_slice(self, node);
    }

    fn visit_pat_struct(&mut self, node: &syn::PatStruct) {
        self.verify_node(node);
        syn::visit::visit_pat_struct(self, node);
    }

    fn visit_pat_tuple(&mut self, node: &syn::PatTuple) {
        self.verify_node(node);
        syn::visit::visit_pat_tuple(self, node);
    }

    fn visit_pat_tuple_struct(&mut self, node: &syn::PatTupleStruct) {
        self.verify_node(node);
        syn::visit::visit_pat_tuple_struct(self, node);
    }

    fn visit_pat_type(&mut self, node: &syn::PatType) {
        self.verify_node(node);
        syn::visit::visit_pat_type(self, node);
    }

    fn visit_pat_wild(&mut self, node: &syn::PatWild) {
        self.verify_node(node);
        syn::visit::visit_pat_wild(self, node);
    }

    fn visit_path(&mut self, node: &syn::Path) {
        self.verify_node(node);
        syn::visit::visit_path(self, node);
    }

    fn visit_path_arguments(&mut self, node: &syn::PathArguments) {
        self.verify_node(node);
        syn::visit::visit_path_arguments(self, node);
    }

    fn visit_path_segment(&mut self, node: &syn::PathSegment) {
        self.verify_node(node);
        syn::visit::visit_path_segment(self, node);
    }

    fn visit_pointer_mutability(&mut self, node: &syn::PointerMutability) {
        self.verify_node(node);
        syn::visit::visit_pointer_mutability(self, node);
    }

    fn visit_precise_capture(&mut self, node: &syn::PreciseCapture) {
        self.verify_node(node);
        syn::visit::visit_precise_capture(self, node);
    }

    fn visit_predicate_lifetime(&mut self, node: &syn::PredicateLifetime) {
        self.verify_node(node);
        syn::visit::visit_predicate_lifetime(self, node);
    }

    fn visit_predicate_type(&mut self, node: &syn::PredicateType) {
        self.verify_node(node);
        syn::visit::visit_predicate_type(self, node);
    }

    fn visit_qself(&mut self, node: &syn::QSelf) {
        self.verify_node(node);
        syn::visit::visit_qself(self, node);
    }

    fn visit_range_limits(&mut self, node: &syn::RangeLimits) {
        self.verify_node(node);
        syn::visit::visit_range_limits(self, node);
    }

    fn visit_receiver(&mut self, node: &syn::Receiver) {
        self.verify_node(node);
        syn::visit::visit_receiver(self, node);
    }

    fn visit_return_type(&mut self, node: &syn::ReturnType) {
        self.verify_node(node);
        syn::visit::visit_return_type(self, node);
    }

    fn visit_signature(&mut self, node: &syn::Signature) {
        self.verify_node(node);
        syn::visit::visit_signature(self, node);
    }

    fn visit_static_mutability(&mut self, node: &syn::StaticMutability) {
        self.verify_node(node);
        syn::visit::visit_static_mutability(self, node);
    }

    fn visit_stmt(&mut self, node: &syn::Stmt) {
        self.verify_node(node);
        syn::visit::visit_stmt(self, node);
    }

    fn visit_stmt_macro(&mut self, node: &syn::StmtMacro) {
        self.verify_node(node);
        syn::visit::visit_stmt_macro(self, node);
    }

    fn visit_trait_bound(&mut self, node: &syn::TraitBound) {
        self.verify_node(node);
        syn::visit::visit_trait_bound(self, node);
    }

    fn visit_trait_bound_modifier(&mut self, node: &syn::TraitBoundModifier) {
        self.verify_node(node);
        syn::visit::visit_trait_bound_modifier(self, node);
    }

    fn visit_trait_item(&mut self, node: &syn::TraitItem) {
        self.verify_node(node);
        syn::visit::visit_trait_item(self, node);
    }

    fn visit_trait_item_const(&mut self, node: &syn::TraitItemConst) {
        self.verify_node(node);
        syn::visit::visit_trait_item_const(self, node);
    }

    fn visit_trait_item_fn(&mut self, node: &syn::TraitItemFn) {
        self.verify_node(node);
        syn::visit::visit_trait_item_fn(self, node);
    }

    fn visit_trait_item_macro(&mut self, node: &syn::TraitItemMacro) {
        self.verify_node(node);
        syn::visit::visit_trait_item_macro(self, node);
    }

    fn visit_trait_item_type(&mut self, node: &syn::TraitItemType) {
        self.verify_node(node);
        syn::visit::visit_trait_item_type(self, node);
    }

    fn visit_type(&mut self, node: &syn::Type) {
        self.verify_node(node);
        syn::visit::visit_type(self, node);
    }

    fn visit_type_array(&mut self, node: &syn::TypeArray) {
        self.verify_node(node);
        syn::visit::visit_type_array(self, node);
    }

    fn visit_type_bare_fn(&mut self, node: &syn::TypeBareFn) {
        self.verify_node(node);
        syn::visit::visit_type_bare_fn(self, node);
    }

    fn visit_type_group(&mut self, node: &syn::TypeGroup) {
        self.verify_node(node);
        syn::visit::visit_type_group(self, node);
    }

    fn visit_type_impl_trait(&mut self, node: &syn::TypeImplTrait) {
        self.verify_node(node);
        syn::visit::visit_type_impl_trait(self, node);
    }

    fn visit_type_infer(&mut self, node: &syn::TypeInfer) {
        self.verify_node(node);
        syn::visit::visit_type_infer(self, node);
    }

    fn visit_type_macro(&mut self, node: &syn::TypeMacro) {
        self.verify_node(node);
        syn::visit::visit_type_macro(self, node);
    }

    fn visit_type_never(&mut self, node: &syn::TypeNever) {
        self.verify_node(node);
        syn::visit::visit_type_never(self, node);
    }

    fn visit_type_param(&mut self, node: &syn::TypeParam) {
        self.verify_node(node);
        syn::visit::visit_type_param(self, node);
    }

    fn visit_type_param_bound(&mut self, node: &syn::TypeParamBound) {
        self.verify_node(node);
        syn::visit::visit_type_param_bound(self, node);
    }

    fn visit_type_paren(&mut self, node: &syn::TypeParen) {
        self.verify_node(node);
        syn::visit::visit_type_paren(self, node);
    }

    fn visit_type_path(&mut self, node: &syn::TypePath) {
        self.verify_node(node);
        syn::visit::visit_type_path(self, node);
    }

    fn visit_type_ptr(&mut self, node: &syn::TypePtr) {
        self.verify_node(node);
        syn::visit::visit_type_ptr(self, node);
    }

    fn visit_type_reference(&mut self, node: &syn::TypeReference) {
        self.verify_node(node);
        syn::visit::visit_type_reference(self, node);
    }

    fn visit_type_slice(&mut self, node: &syn::TypeSlice) {
        self.verify_node(node);
        syn::visit::visit_type_slice(self, node);
    }

    fn visit_type_trait_object(&mut self, node: &syn::TypeTraitObject) {
        self.verify_node(node);
        syn::visit::visit_type_trait_object(self, node);
    }

    fn visit_type_tuple(&mut self, node: &syn::TypeTuple) {
        self.verify_node(node);
        syn::visit::visit_type_tuple(self, node);
    }

    fn visit_un_op(&mut self, node: &syn::UnOp) {
        self.verify_node(node);
        syn::visit::visit_un_op(self, node);
    }

    fn visit_use_glob(&mut self, node: &syn::UseGlob) {
        self.verify_node(node);
        syn::visit::visit_use_glob(self, node);
    }

    fn visit_use_group(&mut self, node: &syn::UseGroup) {
        self.verify_node(node);
        syn::visit::visit_use_group(self, node);
    }

    fn visit_use_name(&mut self, node: &syn::UseName) {
        self.verify_node(node);
        syn::visit::visit_use_name(self, node);
    }

    fn visit_use_path(&mut self, node: &syn::UsePath) {
        self.verify_node(node);
        syn::visit::visit_use_path(self, node);
    }

    fn visit_use_rename(&mut self, node: &syn::UseRename) {
        self.verify_node(node);
        syn::visit::visit_use_rename(self, node);
    }

    fn visit_use_tree(&mut self, node: &syn::UseTree) {
        self.verify_node(node);
        syn::visit::visit_use_tree(self, node);
    }

    fn visit_variadic(&mut self, node: &syn::Variadic) {
        self.verify_node(node);
        syn::visit::visit_variadic(self, node);
    }

    fn visit_variant(&mut self, node: &syn::Variant) {
        self.verify_node(node);
        syn::visit::visit_variant(self, node);
    }

    fn visit_vis_restricted(&mut self, node: &syn::VisRestricted) {
        self.verify_node(node);
        syn::visit::visit_vis_restricted(self, node);
    }

    fn visit_visibility(&mut self, node: &syn::Visibility) {
        self.verify_node(node);
        syn::visit::visit_visibility(self, node);
    }

    fn visit_where_clause(&mut self, node: &syn::WhereClause) {
        self.verify_node(node);
        syn::visit::visit_where_clause(self, node);
    }

    fn visit_where_predicate(&mut self, node: &syn::WherePredicate) {
        self.verify_node(node);
        syn::visit::visit_where_predicate(self, node);
    }
}
