fn main() {
    let mut v1 = vec![10, 20, 30, 40];
    v1.pop();
    
    let mut v2 = Vec::new();
    v2.push(10);
    v2.push(20);
    v2.push(30);
    
    assert_eq!(v1, v2);
    
    v2.extend(0..2);
    
    assert_eq!(v2, &[10, 20, 30, 0, 1]);
    
    let mut v3 = vec![1, 10, 5, 1, 2, 11, 2, 40];
    v3.sort();
    v3.dedup();
    assert_eq!(v3, [1, 2, 5, 10, 11, 40]);
}