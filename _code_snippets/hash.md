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