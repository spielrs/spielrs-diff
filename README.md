# Spielrs Diff
It is a library which compare two tree direcories or two files asynchronously through [tokio](https://tokio.rs)
and return true in case that both are different. Useful to create watchers in the servers

## How install it
1. add the dependency in the Cargo.toml file of the project:
```toml
spielrs_diff = "0.2"
```

## Example

### Dir comparation

```rust
use spielrs_diff::dir_diff;
#[tokio::test]
async fn should_return_true_if_both_dir_tree_are_different() {
   let diff = dir_diff(
       "./mocks/dir_one".to_string(),
       "./mocks/dir_three".to_string(),
   )
   .await;
   assert_eq!(diff, true);
}
```

### File comparation

```rust
use spielrs_diff::{file_diff, diff::FileDiff};
#[tokio::test]
async fn should_return_true_if_both_files_are_not_equal() {
    let diff = file_diff(FileDiff {
        file: "./mocks/dir_one/vlang/purpose/purpose.txt".to_string(),
        file_comp: "./mocks/dir_five/vlang/purpose/purpose.txt".to_string(),
    })
    .await;
    assert_eq!(diff, true);
}
```

## License

Spielrs Diff is MIT licensed. See [license](LICENSE)
