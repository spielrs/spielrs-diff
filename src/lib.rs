pub mod tree;

use tree::{Tree, TreeBuilder};

pub async fn dir_diff(dir_path: String, dir_path_comp: String) -> bool {
    let tree_one: Vec<Tree> = Tree::build_tree(dir_path).await;
    let tree_two: Vec<Tree> = Tree::build_tree(dir_path_comp).await;
    if !Tree::tree_diff(tree_one.clone(), tree_two.clone()) {
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
