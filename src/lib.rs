//!
//! # Spielrs Diff
//! It is a library which compare two tree direcories asynchronously through [tokio](https://tokio.rs)
//! and return true in case that both are different. Useful to create watchers in the servers
//!
//! ## How install it
//! 1. add the dependency in the Cargo.toml file of the project:
//!
//! ```toml
//! spielrs_diff = "0.1"
//! ```
//!
//! ## Example
//! ```rust
//! use spielrs_diff::{dir_diff, diff::Diff};
//!
//! #[tokio::test]
//! async fn should_return_true_if_both_dir_tree_are_different() {
//!    let diff = dir_diff(Diff {
//!        dir: "./mocks/dir_one".to_string(),
//!        dir_comp: "./mocks/dir_five".to_string(),
//!        excluding: Some(vec!["purpose".to_string()]),
//!        recursive_excluding: true,
//!    })
//!    .await;
//!
//!    assert_eq!(diff, true);
//! }
//! ```
//!
//!
pub mod diff;
pub mod tree;

use diff::Diff;
use tree::{Tree, TreeBuilder};

/// Compare two tree directories and return true if both are different
/// You can exclude directories or files in the comparation only from the root path
/// of both or recursively
///
/// # Example
/// ```rust
/// use spielrs_diff::{dir_diff, diff::Diff};
///
/// #[tokio::test]
/// async fn should_return_true_if_both_dir_tree_are_different() {
///    let diff = dir_diff(Diff {
///        dir: "./mocks/dir_one".to_string(),
///        dir_comp: "./mocks/dir_five".to_string(),
///        excluding: Some(vec!["purpose".to_string()]),
///        recursive_excluding: true,
///    })
///    .await;
///
///    assert_eq!(diff, true);
/// }
/// ```
///
pub async fn dir_diff(diff_options: Diff) -> bool {
    let tree_one: Vec<Tree> = Tree::build_tree(
        diff_options.dir,
        diff_options.excluding.clone(),
        diff_options.recursive_excluding,
    )
    .await;
    let tree_two: Vec<Tree> = Tree::build_tree(
        diff_options.dir_comp,
        diff_options.excluding,
        diff_options.recursive_excluding,
    )
    .await;
    if Tree::tree_diff(tree_one.clone(), tree_two.clone()) {
        return true;
    }

    let content_one: Vec<String> = Tree::get_content_files(tree_one).await;
    let content_two: Vec<String> = Tree::get_content_files(tree_two).await;
    !Tree::compare_dir_content(content_one, content_two)
}

#[tokio::test]
async fn should_return_true_if_both_dir_tree_are_different() {
    let diff = dir_diff(Diff {
        dir: "./mocks/dir_one".to_string(),
        dir_comp: "./mocks/dir_three".to_string(),
        excluding: None,
        recursive_excluding: false,
    })
    .await;
    assert_eq!(diff, true);
}

#[tokio::test]
async fn should_return_false_if_both_dir_tree_are_equal() {
    let diff = dir_diff(Diff {
        dir: "./mocks/dir_one".to_string(),
        dir_comp: "./mocks/dir_two".to_string(),
        excluding: None,
        recursive_excluding: false,
    })
    .await;
    assert_eq!(diff, false);
}

#[tokio::test]
async fn should_return_true_if_both_dir_tree_have_different_content() {
    let diff = dir_diff(Diff {
        dir: "./mocks/dir_one".to_string(),
        dir_comp: "./mocks/dir_four".to_string(),
        excluding: None,
        recursive_excluding: false,
    })
    .await;
    assert_eq!(diff, true);
}

#[tokio::test]
async fn should_return_false_if_both_dir_have_different_subdir_excluded_recursively() {
    let diff = dir_diff(Diff {
        dir: "./mocks/dir_one".to_string(),
        dir_comp: "./mocks/dir_five".to_string(),
        excluding: Some(vec!["purpose".to_string()]),
        recursive_excluding: true,
    })
    .await;
    assert_eq!(diff, false);
}

#[tokio::test]
async fn should_return_true_if_both_dir_have_different_subdir_excluded_not_recursively() {
    let diff = dir_diff(Diff {
        dir: "./mocks/dir_one".to_string(),
        dir_comp: "./mocks/dir_five".to_string(),
        excluding: Some(vec!["purpose".to_string()]),
        recursive_excluding: false,
    })
    .await;
    assert_eq!(diff, true);
}
