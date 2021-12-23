# Database

{exchange}

```rs
struct BasePrice {
  pub ask_price: f32,
  pub bid_price: f32,
}
```

{symbol}:{exchange}

```rs
struct Price {
  pub ask_premium: f32,
  pub ask_price: f32,
  pub bid_premium: f32,
  pub bid_price: f32,
}
```

{symbol}

```rs
struct OraclePrice {
  pub ask_best: f32,
  pub ask_price: f32,
  pub bid_best: f32,
  pub bid_price: f32,
  pub prices: HashMap<String, BasePrice>,
}
```
