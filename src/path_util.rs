use quote::ToTokens;
use std::collections::BTreeSet;

pub fn last_segment_name<T>() -> &'static str {
    std::any::type_name::<T>()
        .split(':')
        .next_back()
        .unwrap_or("")
}

pub fn path_string(path: &syn::Path) -> String {
    path.segments
        .to_token_stream()
        .into_iter()
        .map(|token| token.to_string())
        .collect::<Vec<_>>()
        .join("")
}

pub fn collate(tree: &syn::UseTree) -> BTreeSet<String> {
    let mut paths = BTreeSet::new();
    collate_with_prefix(tree, &mut paths, String::new());
    paths
}

fn collate_with_prefix(tree: &syn::UseTree, paths: &mut BTreeSet<String>, prefix: String) {
    match tree {
        syn::UseTree::Path(syn::UsePath { ident, tree, .. }) => {
            let new_prefix = if prefix.is_empty() {
                ident.to_string()
            } else {
                format!("{}::{}", prefix, ident)
            };
            collate_with_prefix(tree, paths, new_prefix);
        }
        syn::UseTree::Group(syn::UseGroup { items, .. }) => {
            for item in items {
                collate_with_prefix(item, paths, prefix.clone());
            }
        }
        syn::UseTree::Name(syn::UseName { ident }) => {
            let full = if prefix.is_empty() {
                ident.to_string()
            } else {
                format!("{}::{}", prefix, ident)
            };
            paths.insert(full);
        }
        syn::UseTree::Rename(syn::UseRename { ident, .. }) => {
            let full = if prefix.is_empty() {
                ident.to_string()
            } else {
                format!("{}::{}", prefix, ident)
            };
            paths.insert(full);
        }
        syn::UseTree::Glob(syn::UseGlob { .. }) => {
            let full = if prefix.is_empty() {
                "*".to_string()
            } else {
                format!("{}::*", prefix)
            };
            paths.insert(full);
        }
    }
}
