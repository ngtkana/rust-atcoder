pub fn windows<T>(items: &[T], bucket_count: usize, key_fn: impl Fn(&T) -> usize) -> Vec<&[T]> {
    let mut result = Vec::with_capacity(bucket_count);
    let mut end = 0;
    for key in 0..bucket_count {
        let start = end;
        while end < items.len() && key_fn(&items[end]) == key {
            end += 1;
        }
        result.push(&items[start..end]);
    }
    result
}

pub fn counting_sort<T>(
    items: Vec<T>,
    bucket_count: usize,
    key_fn: impl Fn(&T) -> usize,
) -> Vec<T> {
    if items.is_empty() {
        return Vec::new();
    }
    let len = items.len();

    let mut end = vec![0; bucket_count];
    for item in &items {
        end[key_fn(item)] += 1;
    }
    for i in 1..bucket_count {
        end[i] += end[i - 1];
    }

    let mut result: Vec<std::mem::MaybeUninit<T>> = Vec::with_capacity(len);
    unsafe { result.set_len(len) }

    let items_ptr = std::mem::ManuallyDrop::new(items).as_ptr();
    let result_ptr = result.as_mut_ptr() as *mut T;

    for i in (0..len).rev() {
        unsafe {
            let item_ref = &*items_ptr.add(i);
            let key = key_fn(item_ref);
            end[key] -= 1;
            std::ptr::write(result_ptr.add(end[key]), std::ptr::read(items_ptr.add(i)));
        }
    }

    let mut mem = std::mem::ManuallyDrop::new(result);
    unsafe { Vec::from_raw_parts(mem.as_mut_ptr() as *mut T, mem.len(), mem.capacity()) }
}
