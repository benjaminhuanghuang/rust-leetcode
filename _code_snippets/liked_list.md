
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

## while loop
```
  while l1.is_some() && l2.is_some() {
    if l1.as_ref().unwrap().val <= l2.as_ref().unwrap().val {
      // better version: without data clone
      curr.next = l1.take();
      curr = curr.next.as_mut().unwrap();
      l1 = curr.next.take();
    } else {
      curr.next = l2.take();
      curr = curr.next.as_mut().unwrap();
      l2 = curr.next.take();
    }
  }
```

## next
```
*cur = Some(Box::new(ListNode::new(sum % 10)));
cur = &mut cur.as_mut().unwrap().next;

```