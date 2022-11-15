# rittle

Create/save chronological notes.

## usage

New note:
```bash
rittle new                              # output: rittle-<ts>.md
rittle new --project foo                # output: foo-<ts>.md
rittle new --prefix me --project this   # output: me-this-<ts>.md
rittle new | xargs nvim                 # open file in editor
```

Save notes:
```bash
rittle save             # save notes recursively to ~/.rittle/rittle.md
rittle --project bar    # save notes recursively to ~/.rittle/bar.md
```
