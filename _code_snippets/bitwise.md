## find different bits
using exclusive OR
```
    let mut xor = x ^ y;
```

## count the 1 
- 461. Hamming Distance
```
    // Faster!
    while xor != 0 {
        distance += 1;
        xor &= xor - 1;
    }

    while xor != 0 {
        distance += xor & 1;
        xor >>= 1;
    }
```