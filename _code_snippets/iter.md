## Sample
- 1. Two Sum
- 1018. Binary Prefix Divisible By 5


## Iter of vec
Index and value
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

Value
```
let v=vec![1,2,3,4,5];

for i in v.iter {
}
```
## For each
```
items.iter().enumerate().for_each(|(i, x)| {
    println!("Item {} = {}", i, x);
})
```

## filter
```
```