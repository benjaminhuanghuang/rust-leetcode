## Create
```
  use std::collections::HashMap;

  let mut hm: HashMap<i32, i32> = HashMap::new();
```

## Insert
```
let mut hash_map:HashMap<String, String> = HashMap::new();
 
hash_map.insert("1".to_string(), "A".to_string());
hash_map.insert("2".to_string(), "B".to_string());
```


## Access

```
 // 接受一个引用并返回 Option<&V>
  match contacts.get(&"Daniel") {
      Some(&number) => println!("Calling Daniel: {}", call(number)),
      _ => println!("Don't have Daniel's number."),
  }
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
  // `HashMap::iter()` 返回一个迭代器，该迭代器以任意顺序举出
  // (&'a key, &'a value) 对。
  for (contact, &number) in contacts.iter() {
      println!("Calling {}: {}", contact, call(number)); 
  }
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