# rittle

Create/save chronological notes.

## usage

New note:
```bash
rittle new                              # output: rittle-<ts>.md
rittle new | xargs nvim                 # open file in editor
rittle new --project foo                # output: foo-<ts>.md
rittle new --prefix bar --project baz   # output: bar-baz-<ts>.md
```

Save notes:
```bash
rittle save
```
