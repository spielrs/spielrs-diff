use async_trait::async_trait;
use futures::StreamExt;
use std::iter::FromIterator;
use tokio::fs;
use tokio::stream;

/// Trait for `Tree` to create your own `TreeBuilder`
#[async_trait]
pub trait TreeBuilder {
    /// Build a vector of `Tree`
    async fn build_tree(
        dir_path: String,
        excluding: Option<Vec<String>>,
        recursive_excluding: bool,
    ) -> Vec<Tree>;
    /// Compare two tree directories and return true if are different
    fn tree_diff(dir_tree: Vec<Tree>, dir_tree_comp: Vec<Tree>) -> bool;
    /// Get the content by string of all the files in one tree directory
    async fn get_content_files(dir_tree: Vec<Tree>) -> Vec<String>;
    /// compare all the content from two tree directories and return true if both are equal
    fn compare_dir_content(dir_content: Vec<String>, dir_content_comp: Vec<String>) -> bool;
}

/// Represent a tree directory
#[derive(Debug, PartialEq, Clone)]
pub struct Tree {
    pub name: String,
    pub path: String,
    pub subdir: Option<Vec<Tree>>,
}

#[derive(Debug, PartialEq)]
struct TreeComp {
    pub name: String,
    pub subdir: Option<Vec<TreeComp>>,
}

struct ExtratedFile {
    pub name: String,
    pub path: String,
}

struct TreeFlatted(Vec<ExtratedFile>);

impl TreeFlatted {
    fn new() -> Self {
        TreeFlatted(Vec::new())
    }

    fn add(&mut self, elem: ExtratedFile) {
        self.0.push(elem);
    }
}

impl FromIterator<Tree> for TreeFlatted {
    fn from_iter<I: IntoIterator<Item = Tree>>(iter: I) -> Self {
        let mut tree_flatted = TreeFlatted::new();

        for i in iter {
            if let Some(entry) = i.subdir {
                for sub_iter in TreeFlatted::from_iter(entry).0 {
                    tree_flatted.add(sub_iter);
                }
            } else {
                let extrated_file = ExtratedFile {
                    name: i.name,
                    path: i.path,
                };

                tree_flatted.add(extrated_file);
            }
        }

        tree_flatted
    }
}

impl From<Tree> for TreeComp {
    fn from(tree: Tree) -> Self {
        TreeComp {
            name: tree.name,
            subdir: if let Some(entry) = tree.subdir {
                Some(entry.into_iter().map(TreeComp::from).collect())
            } else {
                None
            },
        }
    }
}

#[async_trait]
impl TreeBuilder for Tree {
    /// Build a vector of `Tree`
    /// You can exclude directories or files only from the root path
    /// of the directory or recursively in the building
    ///
    /// # Example
    ///
    /// ```rust
    /// use spielrs_diff::tree::{Tree, TreeBuilder};
    ///
    /// #[tokio::main]
    /// async fn main() {
    ///     let dir_one = Tree::build_tree(
    ///         "./mocks/dir_one".to_string(),
    ///         Some(vec!["purpose".to_string()]),
    ///         true
    ///     ).await;
    ///
    ///     println!("{:#?}", dir_one);
    /// }
    /// ```
    async fn build_tree(
        dir_path: String,
        excluding: Option<Vec<String>>,
        recursive_excluding: bool,
    ) -> Vec<Tree> {
        let mut entries = fs::read_dir(dir_path).await.unwrap();
        let mut tree: Vec<Tree> = vec![];
        let mut exclude: Vec<String> = vec![];
        if let Some(mut item) = excluding {
            exclude.append(&mut item);
        }

        while let Some(entry) = entries.next_entry().await.unwrap() {
            let file_name = entry.file_name().into_string().unwrap();

            if !exclude.clone().into_iter().any(|item| item == file_name) {
                let path: String = entry.path().into_os_string().into_string().unwrap();
                tree.push(Tree {
                    name: entry.file_name().into_string().unwrap(),
                    path: path.clone(),
                    subdir: if entry.path().is_dir() {
                        Some(
                            Tree::build_tree(
                                path,
                                if recursive_excluding {
                                    Some(exclude.clone())
                                } else {
                                    None
                                },
                                recursive_excluding,
                            )
                            .await,
                        )
                    } else {
                        None
                    },
                });
            }
        }

        tree
    }

    /// Compare two tree directories and return true if are different
    ///
    /// # Example
    ///
    /// ```rust
    /// use spielrs_diff::tree::{Tree, TreeBuilder};
    ///
    /// #[tokio::test]
    /// async fn should_return_false_equal_dir_tree() {
    ///     let dir_one = Tree::build_tree("./mocks/dir_one".to_string(), None, false).await;
    ///     let dir_two = Tree::build_tree("./mocks/dir_two".to_string(), None, false).await;
    ///
    ///     let diff = Tree::tree_diff(dir_one, dir_two);
    ///
    ///     assert_eq!(diff, false);
    /// }
    /// ```
    fn tree_diff(dir_tree: Vec<Tree>, dir_tree_comp: Vec<Tree>) -> bool {
        let dir_tree_from: Vec<TreeComp> = dir_tree.into_iter().map(TreeComp::from).collect();
        let dir_tree_comp_from: Vec<TreeComp> =
            dir_tree_comp.into_iter().map(TreeComp::from).collect();

        dir_tree_from != dir_tree_comp_from
    }

    /// Get the content by string of all the files in one tree directory
    ///
    /// # Example
    ///
    /// ```rust
    /// use spielrs_diff::tree::{Tree, TreeBuilder};
    ///
    /// #[tokio::test]
    /// async fn should_return_all_file_content() {
    ///     let dir_one = Tree::build_tree("./mocks/dir_one".to_string(), None, false).await;
    ///     let content = Tree::get_content_files(dir_one).await;
    ///
    ///     assert_eq!(
    ///         content,
    ///         vec!(
    ///             "Hello world",
    ///             "print(\"This line will be printed.\")",
    ///             "new language",
    ///             "fn main() {\n    println(\"hello world\")\n}\n",
    ///         )
    ///     )
    /// }
    /// ```
    async fn get_content_files(dir_tree: Vec<Tree>) -> Vec<String> {
        let file_list: TreeFlatted = TreeFlatted::from_iter(dir_tree);
        let files = stream::iter(file_list.0);

        let file_content: Vec<String> = files
            .then(|file| async { fs::read_to_string(file.path).await.unwrap() })
            .collect::<Vec<String>>()
            .await;

        file_content
    }

    /// compare all the content from two tree directories and return true if both are equal
    ///
    /// # Example
    ///
    /// ```rust
    /// use spielrs_diff::tree::{Tree, TreeBuilder};
    ///
    /// #[tokio::test]
    /// async fn should_return_true_if_both_dir_content_are_equal() {
    ///     let dir_one = Tree::build_tree("./mocks/dir_one".to_string(), None, false).await;
    ///     let content_one = Tree::get_content_files(dir_one).await;
    ///
    ///     let dir_two = Tree::build_tree("./mocks/dir_two".to_string(), None, false).await;
    ///     let content_two = Tree::get_content_files(dir_two).await;
    ///
    ///     assert_eq!(Tree::compare_dir_content(content_one, content_two), true);
    /// }
    /// ```
    fn compare_dir_content(dir_content: Vec<String>, dir_content_comp: Vec<String>) -> bool {
        dir_content.into_iter().all(move |content| {
            dir_content_comp
                .clone()
                .into_iter()
                .any(move |content_comp| content_comp == content)
        })
    }
}

#[tokio::test]
async fn should_return_false_equal_dir_tree() {
    let dir_one = Tree::build_tree("./mocks/dir_one".to_string(), None, false).await;
    let dir_two = Tree::build_tree("./mocks/dir_two".to_string(), None, false).await;

    let diff = Tree::tree_diff(dir_one, dir_two);

    assert_eq!(diff, false);
}

#[tokio::test]
async fn should_return_true_different_dir_tree() {
    let dir_one = Tree::build_tree("./mocks/dir_one".to_string(), None, false).await;
    let dir_three = Tree::build_tree("./mocks/dir_three".to_string(), None, false).await;

    let diff = Tree::tree_diff(dir_one, dir_three);

    assert_eq!(diff, true);
}

#[tokio::test]
async fn should_return_all_file_content() {
    let dir_one = Tree::build_tree("./mocks/dir_one".to_string(), None, false).await;
    let content = Tree::get_content_files(dir_one).await;

    assert_eq!(
        content,
        vec!(
            "Hello world",
            "print(\"This line will be printed.\")",
            "new language",
            "fn main() {\n    println(\"hello world\")\n}\n",
        )
    )
}

#[tokio::test]
async fn should_return_true_if_both_dir_content_are_equal() {
    let dir_one = Tree::build_tree("./mocks/dir_one".to_string(), None, false).await;
    let content_one = Tree::get_content_files(dir_one).await;

    let dir_two = Tree::build_tree("./mocks/dir_two".to_string(), None, false).await;
    let content_two = Tree::get_content_files(dir_two).await;

    assert_eq!(Tree::compare_dir_content(content_one, content_two), true);
}

#[tokio::test]
async fn should_return_false_if_both_dir_content_are_differents() {
    let dir_one = Tree::build_tree("./mocks/dir_one".to_string(), None, false).await;
    let content_one = Tree::get_content_files(dir_one).await;

    let dir_four = Tree::build_tree("./mocks/dir_four".to_string(), None, false).await;
    let content_four = Tree::get_content_files(dir_four).await;

    assert_eq!(Tree::compare_dir_content(content_one, content_four), false);
}

#[tokio::test]
async fn should_return_true_if_both_dir_tree_have_different_subdir_excluded_recursively() {
    let dir_one = Tree::build_tree(
        "./mocks/dir_one".to_string(),
        Some(vec!["purpose".to_string()]),
        true,
    )
    .await;
    let content_one = Tree::get_content_files(dir_one).await;

    let dir_five = Tree::build_tree(
        "./mocks/dir_five".to_string(),
        Some(vec!["purpose".to_string()]),
        true,
    )
    .await;
    let content_five = Tree::get_content_files(dir_five).await;

    assert_eq!(Tree::compare_dir_content(content_one, content_five), true);
}

#[tokio::test]
async fn should_return_false_if_both_dir_tree_have_different_subdir_excluded_not_recursively() {
    let dir_one = Tree::build_tree(
        "./mocks/dir_one".to_string(),
        Some(vec!["purpose".to_string()]),
        false,
    )
    .await;
    let content_one = Tree::get_content_files(dir_one).await;

    let dir_five = Tree::build_tree(
        "./mocks/dir_five".to_string(),
        Some(vec!["purpose".to_string()]),
        false,
    )
    .await;
    let content_five = Tree::get_content_files(dir_five).await;

    assert_eq!(Tree::compare_dir_content(content_one, content_five), false);
}
