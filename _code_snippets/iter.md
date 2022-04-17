

## index value
```
  for (i, num1) in nums.iter().enumerate() {
    //enumerate() starts from 0
    for (j, num2) in nums.iter().skip(i + 1).enumerate() {
      if num1 + num2 == target {
        return vec![i as i32, (j + 1 + i) as i32];
      }
    }
  }
  vec![]
```

## value
```
for price in prices.iter {
}
```