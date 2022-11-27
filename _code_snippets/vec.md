

## Create N elements in vector
```
  let mut result = vec![String::new(); num_rows];

  let mut result = Vec::with_capacity(nums.len());
```

```
  let vec = vec![0; 5];    // [0, 0, 0, 0, 0]
```


## loop
Use index
```
for i in 0..nums.len() {
  
}
```

Use *
```
  for price in prices.iter() {
      if *price < lowest_price 
  }
```

```
  for (i, v) in nums.iter().enumerate() {
    
  }
```  

skip()
```
  for (j, num2) in nums.iter().skip(i + 1).enumerate() {
  
  }
```

## For each
```
  items.iter().enumerate().for_each(|(i, x)| {
      println!("Item {} = {}", i, x);
  })
```

## iterator to vec
```
  let mut result: HashSet<i32> = HashSet::new();

  for i in nums {
    let v = hm.entry(i).or_insert(0);
    *v += 1;
    if *v > 1 {
      result.insert(i);
    }
  }
  result.into_iter().collect()
```
