use std::f32;

fn main() {
    let nums1 = [1,2];
    let nums2 = [3,4];
    let mut q : f32 = nums1[0] as f32;
    let mut p : f32 = nums2[0] as f32;
    for i in 1..nums1.len() {
        q = q + (nums1[i] as f32)
    }
    q = q/(nums1.len() as f32);
    for i in 1..nums2.len() {
        p = p + (nums2[i] as f32)
    }
    p = p/(nums2.len() as f32);
    let output = (p+q)/2.0;
    println!("result: {}", output);
}
