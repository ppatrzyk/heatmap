# heatmap

## DEV

```
cargo build
target/debug/heatmap
```

## Example

```
curl -s http://api.nbp.pl/api/exchangerates/rates/a/eur/last/10/?format=json \
| jq -r '.rates | map([.no, .effectiveDate, .mid] | join(", ")) | join("\n")'
```
