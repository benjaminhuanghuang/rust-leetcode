## Trim
```
 for c in str.trim_start().chars() {
```   


## for chars in string
```
  for c in s.chars() {
    
  }
```
Use index
```
 for (i, c) in word.chars().enumerate() {
    if i == word.len() - 1 {
      //  TODO:
    }
            
 }
```

## count chars in a string
```
  let mut hm = HashMap::new();
  for ch in s.chars() {
    *hm.entry(ch).or_insert(0) += 1;
  }
```

## sting to vector and access char by index
```
  let s: Vec<char> = sv.chars().collect();
```


## substring
```
  s[start..=end].iter().collect()
```

## number to String
```
pub fn reverse_str(x: i32) -> i32 {
    x.signum()
      * x
        .abs()
        .to_string()
        .chars()
        .rev()
        .collect::<String>()
        .parse::<i32>()
        .unwrap_or(0)
  }
```