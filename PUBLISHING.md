Currently, publishing is done manually. Simply,

1. Create a new branch
2. Bump the version of each package and commit. We want to keep sync between each oxytail package version. Also update documentation and every mention of version to be installed(including each packages `Cargo.toml`!).
3. Run the following commands

```
cargo publish -p oxytail-base
cargo publish -p oxytail-theme-dark
cargo publish -p oxytail-theme-defaults
```

4. Merge the release branch 