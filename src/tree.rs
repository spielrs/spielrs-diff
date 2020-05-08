use async_trait::async_trait;
use futures::StreamExt;
use std::iter::FromIterator;
use tokio::fs;
use tokio::stream;

#[async_trait]
trait TreeBuilder {
    async fn build_tree(dir_path: String) -> Vec<Tree>;
    fn tree_diff(dir_tree: Vec<Tree>, dir_tree_comp: Vec<Tree>) -> bool;
    async fn get_content_files(dir_tree: Vec<Tree>) -> Vec<String>;
    fn compare_dir_contents(dir_contents: Vec<String>, dir_contents_comp: Vec<String>) -> bool;
}

#[derive(Debug, PartialEq)]
pub struct Tree {
    pub name: String,
    pub path: String,
    pub subdir: Option<Vec<Tree>>,
}

#[derive(Debug, PartialEq)]
pub struct TreeComp {
    pub name: String,
    pub subdir: Option<Vec<TreeComp>>,
}

pub struct ExtratedFile {
    pub name: String,
    pub path: String,
}

pub struct TreeFlatted(Vec<ExtratedFile>);

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
    async fn build_tree(dir_path: String) -> Vec<Tree> {
        let mut entries = fs::read_dir(dir_path).await.unwrap();
        let mut tree: Vec<Tree> = vec![];
        while let Some(entry) = entries.next_entry().await.unwrap() {
            let path: String = entry.path().into_os_string().into_string().unwrap();
            tree.push(Tree {
                name: entry.file_name().into_string().unwrap(),
                path: path.clone(),
                subdir: if entry.path().is_dir() {
                    Some(Tree::build_tree(path).await)
                } else {
                    None
                },
            });
        }

        tree
    }

    fn tree_diff(dir_tree: Vec<Tree>, dir_tree_comp: Vec<Tree>) -> bool {
        let dir_tree_from: Vec<TreeComp> = dir_tree.into_iter().map(TreeComp::from).collect();
        let dir_tree_comp_from: Vec<TreeComp> =
            dir_tree_comp.into_iter().map(TreeComp::from).collect();

        dir_tree_from == dir_tree_comp_from
    }

    async fn get_content_files(dir_tree: Vec<Tree>) -> Vec<String> {
        let file_list: TreeFlatted = TreeFlatted::from_iter(dir_tree);
        let files = stream::iter(file_list.0);

        let file_contents: Vec<String> = files
            .then(|file| async { fs::read_to_string(file.path).await.unwrap() })
            .collect::<Vec<String>>()
            .await;

        file_contents
    }

    fn compare_dir_contents(dir_contents: Vec<String>, dir_contents_comp: Vec<String>) -> bool {
        dir_contents.into_iter().all(move |content| {
            dir_contents_comp
                .clone()
                .into_iter()
                .any(move |content_comp| content_comp == content)
        })
    }
}

#[tokio::test]
async fn should_return_true_equal_dir_tree() {
    let dir_one = Tree::build_tree("./mocks/dir_one".to_string()).await;
    let dir_two = Tree::build_tree("./mocks/dir_two".to_string()).await;

    let are_equal = Tree::tree_diff(dir_one, dir_two);

    assert_eq!(are_equal, true);
}

#[tokio::test]
async fn should_return_false_different_dir_tree() {
    let dir_one = Tree::build_tree("./mocks/dir_one".to_string()).await;
    let dir_three = Tree::build_tree("./mocks/dir_three".to_string()).await;

    let are_equal = Tree::tree_diff(dir_one, dir_three);

    assert_eq!(are_equal, false);
}

#[tokio::test]
async fn should_return_all_file_contents() {
    let dir_one = Tree::build_tree("./mocks/dir_one".to_string()).await;
    let contents = Tree::get_content_files(dir_one).await;

    assert_eq!(
        contents,
        vec!(
            "Hello world",
            "print(\"This line will be printed.\")",
            "new language",
            "fn main() {\n    println(\"hello world\")\n}\n",
        )
    )
}

#[tokio::test]
async fn should_return_true_if_both_dir_contents_are_equal() {
    let dir_one = Tree::build_tree("./mocks/dir_one".to_string()).await;
    let contents_one = Tree::get_content_files(dir_one).await;

    let dir_two = Tree::build_tree("./mocks/dir_two".to_string()).await;
    let contents_two = Tree::get_content_files(dir_two).await;

    assert_eq!(Tree::compare_dir_contents(contents_one, contents_two), true);
}

#[tokio::test]
async fn should_return_false_if_both_dir_contents_are_differents() {
    let dir_one = Tree::build_tree("./mocks/dir_one".to_string()).await;
    let contents_one = Tree::get_content_files(dir_one).await;

    let dir_four = Tree::build_tree("./mocks/dir_four".to_string()).await;
    let contents_four = Tree::get_content_files(dir_four).await;

    assert_eq!(
        Tree::compare_dir_contents(contents_one, contents_four),
        false
    );
}
