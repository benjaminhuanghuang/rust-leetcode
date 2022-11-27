## Create
```
  let mut hash: HashMap<i32, usize> = HashMap::new();
```


## Is exist 
Use HashMap.contains_key()
```
  if hash.contains_key(&val) {

  }
```

Use HashMap.get()
```
  if let Some(&complement_index) = hash.get(&complement) {
      
  }
```

Use HashMap.get()
```
  match hash.get(&val) {
    Some(&index) => { }
    None => {}
  }
```

## loop
```
  
```

## count chars in a string
```
  let mut hm = HashMap::new();
  for ch in s.chars() {
    *hm.entry(ch).or_insert(0) += 1;
  }
```


```
  for (i, num) in nums.iter().enumerate() {
    let complement = target - num;
    if hash.contains_key(&complement) {
      return vec![hash[&complement], i as i32];
    }
    hash.insert(num, i as i32);
  }

```

```
  let mut hash: HashMap<i32, usize> = HashMap::new();

  for (i, num) in nums.iter().enumerate() {
    let complement = target - num;
    if let Some(&complement_index) = hash.get(&complement) {
      return vec![complement_index as i32, i as i32];
    }
    hash.insert(*num, i);
  }
```