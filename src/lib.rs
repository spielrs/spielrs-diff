//!
//! # Spielrs Diff
//! It is a library which compare two tree direcories or two files asynchronously through [tokio](https://tokio.rs)
//! and return true in case that both are different. Useful to create watchers in the servers
//!
//! ## How install it
//! 1. add the dependency in the Cargo.toml file of the project:
//!
//! ```toml
//! spielrs_diff = "0.2"
//! ```
//!
//! ## Example
//!
//! ### Dir comparation
//! ```rust
//! use spielrs_diff::{dir_diff, diff::DirDiff};
//!
//! #[tokio::test]
//! async fn should_return_true_if_both_dir_tree_are_different() {
//!    let diff = dir_diff(DirDiff {
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
//! ### File comparation
//! ```rust
//! use spielrs_diff::{file_diff, diff::FileDiff};
//!
//! #[tokio::test]
//! async fn should_return_true_if_both_files_are_not_equal() {
//!     let diff = file_diff(FileDiff {
//!         file: "./mocks/dir_one/vlang/purpose/purpose.txt".to_string(),
//!         file_comp: "./mocks/dir_five/vlang/purpose/purpose.txt".to_string(),
//!     })
//!     .await;
//!
//!     assert_eq!(diff, true);
//! }
//! ```
pub mod diff;
pub mod tree;

use diff::{DirDiff, FileDiff};
use tokio::fs;
use tree::{Tree, TreeBuilder};

/// Compare two tree directories and return true if both are different
/// You can exclude directories or files in the comparation only from the root path
/// of both or recursively
///
/// # Example
/// ```rust
/// use spielrs_diff::{dir_diff, diff::DirDiff};
///
/// #[tokio::test]
/// async fn should_return_true_if_both_dir_tree_are_different() {
///    let diff = dir_diff(DirDiff {
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
pub async fn dir_diff(dir_diff_options: DirDiff) -> bool {
    let tree_one: Vec<Tree> = Tree::build_tree(
        dir_diff_options.dir,
        dir_diff_options.excluding.clone(),
        dir_diff_options.recursive_excluding,
    )
    .await;
    let tree_two: Vec<Tree> = Tree::build_tree(
        dir_diff_options.dir_comp,
        dir_diff_options.excluding,
        dir_diff_options.recursive_excluding,
    )
    .await;
    if Tree::tree_diff(tree_one.clone(), tree_two.clone()) {
        return true;
    }

    let content_one: Vec<String> = Tree::get_content_files(tree_one).await;
    let content_two: Vec<String> = Tree::get_content_files(tree_two).await;
    !Tree::compare_dir_content(content_one, content_two)
}

/// Compare two files and return true if both are different
///
/// #Example
/// ```rust
/// use spielrs_diff::{file_diff, diff::FileDiff};
///
/// #[tokio::test]
/// async fn should_return_true_if_both_files_are_not_equal() {
///     let diff = file_diff(FileDiff {
///         file: "./mocks/dir_one/vlang/purpose/purpose.txt".to_string(),
///         file_comp: "./mocks/dir_five/vlang/purpose/purpose.txt".to_string(),
///     })
///     .await;
///
///     assert_eq!(diff, true);
/// }
/// ```
pub async fn file_diff(file_diff_options: FileDiff) -> bool {
    let file_one = fs::read_to_string(file_diff_options.file).await.unwrap();
    let file_two = fs::read_to_string(file_diff_options.file_comp)
        .await
        .unwrap();

    file_one != file_two
}

#[tokio::test]
async fn should_return_true_if_both_dir_tree_are_different() {
    let diff = dir_diff(DirDiff {
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
    let diff = dir_diff(DirDiff {
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
    let diff = dir_diff(DirDiff {
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
    let diff = dir_diff(DirDiff {
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
    let diff = dir_diff(DirDiff {
        dir: "./mocks/dir_one".to_string(),
        dir_comp: "./mocks/dir_five".to_string(),
        excluding: Some(vec!["purpose".to_string()]),
        recursive_excluding: false,
    })
    .await;
    assert_eq!(diff, true);
}

#[tokio::test]
async fn should_return_false_if_both_files_are_equal() {
    let diff = file_diff(FileDiff {
        file: "./mocks/dir_one/hello.txt".to_string(),
        file_comp: "./mocks/dir_two/hello.txt".to_string(),
    })
    .await;

    assert_eq!(diff, false);
}

#[tokio::test]
async fn should_return_true_if_both_files_are_not_equal() {
    let diff = file_diff(FileDiff {
        file: "./mocks/dir_one/vlang/purpose/purpose.txt".to_string(),
        file_comp: "./mocks/dir_five/vlang/purpose/purpose.txt".to_string(),
    })
    .await;

    assert_eq!(diff, true);
}
