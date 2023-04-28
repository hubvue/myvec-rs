use std::alloc;
use std::ptr::NonNull;
pub struct MyVec<T> {
    ptr: NonNull<T>,
    len: usize,
    capacity: usize,
}

impl<T> MyVec<T> {
    pub fn new() -> Self {
        Self {
            ptr: NonNull::dangling(), // 初始化是创建一个悬空指针
            len: 0,
            capacity: 0,
        }
    }
    pub fn push(&mut self, item: T) {
        assert_ne!(std::mem::size_of::<T>(), 0, "No zero sized types");
        if self.capacity == 0 {
            // Safety: the layout is hardcoded to be 4 * sizeo_of<T> and
            // size_of<T> is > 0
            let layout = alloc::Layout::array::<T>(4).expect("Could not allocate");
            let ptr = unsafe { alloc::alloc(layout) } as *mut T;
            // 当内存耗尽或者布局不符合此分配器大小或对齐约束时，alloc会返回一个空指针，因此将指针包装成NonNull，
            // 当为空指针的时候panic
            let ptr = NonNull::new(ptr).expect("Could not allocation memory");
            // Safetyp: ptr是非空的，在之前分配了内存

            // The memory previsously at ptr is not read
            unsafe { ptr.as_ptr().write(item) };
            self.ptr = ptr;
            self.capacity = 4;
            self.len = 1;
        } else if self.len < self.capacity {
            // 指针计算偏移量操作安全的前提：
            //  计算出的偏移量以字节为单位不能超过isize:MAX
            //  使用checked_mul安全乘法确保不超过isize:MAX
            let offset = self
                .len
                .checked_mul(std::mem::size_of::<T>())
                .expect("Cannot reach memory location");

            assert!(offset < isize::MAX as usize, "Wrapped isize");
            // add 计算指针的偏移量，已size_of::<T>为单位，count为偏移的个数，总偏移量为 size_of::<T> * count
            // add 底层调offset来实现
            unsafe { self.ptr.as_ptr().add(self.len).write(item) }
            self.len += 1;
        } else {
            debug_assert!(self.len == self.capacity);
            let new_capacity = self.capacity.checked_mul(2).expect("Capacity wrapped");
            let align = std::mem::align_of::<T>();
            let size = std::mem::size_of::<T>()
                .checked_mul(self.capacity)
                .expect("Cannot reach memory location");

            size.checked_add(size % align).expect("Can not allocate");
            let ptr = unsafe {
                let layout = alloc::Layout::from_size_align_unchecked(size, align);
                let new_size = std::mem::size_of::<T>()
                    .checked_mul(new_capacity)
                    .expect("Size wrapped");

                let ptr = alloc::realloc(self.ptr.as_ptr() as *mut u8, layout, new_size);
                let ptr = NonNull::new(ptr as *mut T).expect("Could not reallocate");
                ptr.as_ptr().add(self.len).write(item);
                ptr
            };
            self.ptr = ptr;
            self.len += 1;
            self.capacity = new_capacity;
        }
    }
    pub fn get(&self, index: usize) -> Option<&T> {
        if index >= self.len {
            return None;
        }
        let item = unsafe { &*self.ptr.as_ptr().add(index) };
        Some(item)
    }
    pub fn len(&self) -> usize {
        self.len
    }
    pub fn capacity(&self) -> usize {
        self.capacity
    }
}

impl<T> Drop for MyVec<T> {
    fn drop(&mut self) {
        unsafe {
            std::ptr::drop_in_place(std::slice::from_raw_parts_mut(self.ptr.as_ptr(), self.len));
            let layout = alloc::Layout::from_size_align_unchecked(
                std::mem::size_of::<T>() * self.capacity,
                std::mem::align_of::<T>(),
            );
            alloc::dealloc(self.ptr.as_ptr() as *mut u8, layout)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let mut vec: MyVec<usize> = MyVec::new();
        vec.push(1usize);
        vec.push(2usize);
        vec.push(3usize);
        vec.push(4usize);
        vec.push(5usize);

        for n in 0..vec.len() {
            assert_eq!(vec.get(n), Some(&(n + 1)));
        }

        assert_eq!(vec.capacity(), 8);
        assert_eq!(vec.len(), 5);
    }
}
