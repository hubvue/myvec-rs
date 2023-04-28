use myvec::MyVec;
fn main() {
    let mut vec: MyVec<usize> = MyVec::new();
    vec.push(1usize);
    vec.push(1usize);
    vec.push(1usize);
    vec.push(1usize);
    vec.push(1usize);

    assert_eq!(vec.capacity(), 8);
    assert_eq!(vec.len(), 5);
}
