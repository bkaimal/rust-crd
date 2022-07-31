# Add CRD

`$ kubectl apply -f docs/crd.yaml`

`$ kubectl apply -f docs/book.yaml`

# Building

```
$ cargo run
    Finished dev [unoptimized + debuginfo] target(s) in 0.76s
     Running `target/debug/rust-crd`
saw apply to Moby Dick
```