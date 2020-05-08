//!
//! # Spielrs Diff
//! It is a library which compare two tree direcories asynchronously through [tokio](https://tokio.rs))
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
//! use spielrs_diff::dir_diff;
//!
//! #[tokio::test]
//! async fn should_return_true_if_both_dir_tree_are_different() {
//!    let diff = dir_diff(
//!        "./mocks/dir_one".to_string(),
//!        "./mocks/dir_three".to_string(),
//!    )
//!    .await;
//!
//!    assert_eq!(diff, true);
//! }
//! ```
//!
//!
pub mod tree;

use tree::{Tree, TreeBuilder};

/// Compare two tree directories and return true if both are different
///
/// # Example
/// ```rust
/// use spielrs_diff::dir_diff;
///
/// #[tokio::test]
/// async fn should_return_true_if_both_dir_tree_are_different() {
///    let diff = dir_diff(
///        "./mocks/dir_one".to_string(),
///        "./mocks/dir_three".to_string(),
///    )
///    .await;
///
///    assert_eq!(diff, true);
/// }
/// ```
///
pub async fn dir_diff(dir_path: String, dir_path_comp: String) -> bool {
    let tree_one: Vec<Tree> = Tree::build_tree(dir_path).await;
    let tree_two: Vec<Tree> = Tree::build_tree(dir_path_comp).await;
    if Tree::tree_diff(tree_one.clone(), tree_two.clone()) {
        return true;
    }

    let content_one: Vec<String> = Tree::get_content_files(tree_one).await;
    let content_two: Vec<String> = Tree::get_content_files(tree_two).await;
    !Tree::compare_dir_content(content_one, content_two)
}

#[tokio::test]
async fn should_return_true_if_both_dir_tree_are_different() {
    let diff = dir_diff(
        "./mocks/dir_one".to_string(),
        "./mocks/dir_three".to_string(),
    )
    .await;
    assert_eq!(diff, true);
}

#[tokio::test]
async fn should_return_false_if_both_dir_tree_are_equal() {
    let diff = dir_diff("./mocks/dir_one".to_string(), "./mocks/dir_two".to_string()).await;
    assert_eq!(diff, false);
}

#[tokio::test]
async fn should_return_true_if_both_dir_tree_have_different_content() {
    let diff = dir_diff(
        "./mocks/dir_one".to_string(),
        "./mocks/dir_four".to_string(),
    )
    .await;
    assert_eq!(diff, true);
}
