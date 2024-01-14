Currently, publishing is done manually. Simply,

1. Create a new branch
2. Bump the version and commit
3. Run the following commands

```
cargo publish -p oxytail-base
cargo publish -p oxytail-theme-dark
cargo publish -p oxytail-theme-defaults
```

4. Merge the release branch 