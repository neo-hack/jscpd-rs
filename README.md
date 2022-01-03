# jscpdrs
> detect copy paste in your `js/ts/jsx/tsx` files

## install

## brew
```console
brew tap neo-hack/jscpdrs
brew install jscpdrs
```

## usage

goto any `js/ts/jsx/tsx` projects.

```console
$ jscpdrs
```

will generate `result.json` files contain `copy/paste` code fragment

### options

- `cwd` - config detect project path, default `./`. e.g. `jscpdrs --cwd <path>`
- `ignore` - ignore detect files, support multiple values, default ignore `node_modules` and files defined in `.gitignore`. e.g. `jscpdrs --ignore mock __test__`
- `min_token` - dupe token more than `min_token` is duplicated, default `50`. e.g. `jscpdrs --min_token 50`
- `output` - define output results file path, default `./results.json`. e.g. `jscpdrs --output path/results.json`