
## iteration  
```
  type List = Option<Box<ListNode>>

  let mut p = &l;
  while p.is_some() {
    println!("{}", p.as_ref().unwrap().val);
    p = p.as_ref().unwrap().next;
  }
```


## while loop
```
  let mut head = head;
  let mut reversed = None;

  while let Some(mut node) = head {
    head = node.next;
    node.next = reversed;
    reversed = Some(node);
  }

  reversed
```

## next
```
*cur = Some(Box::new(ListNode::new(sum % 10)));
cur = &mut cur.as_mut().unwrap().next;

```