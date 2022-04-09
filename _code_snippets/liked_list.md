
## iteration  
```
  type List = Option<Box<ListNode>>

  let mut p = &l;
  while p.is_some() {
    println!("{}", p.as_ref().unwrap().val);
    p = p.as_ref().unwrap().next;
  }
```