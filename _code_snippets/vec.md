

## Create N elements in vector
```
  let mut result = vec![String::new(); num_rows];
```

```
  let vec = vec![0; 5];    // [0, 0, 0, 0, 0]
```


## loop
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