pub struct PtrSlice<T> {
    ptr: *const T,
    len: usize,
}

impl<T> From<&[T]> for PtrSlice<T> {
    fn from(value: &[T]) -> Self {
        return PtrSlice { ptr: value.as_ptr(), len: value.len() }
    }
}

impl<T> PtrSlice<T> {
    pub fn iter(&self) -> PtrSliceIter<'_, T> {
        return PtrSliceIter { slice: self, i: 0 };
    }

    pub fn as_slice(&self) -> &[T] {
        return unsafe { std::slice::from_raw_parts(self.ptr, self.len) };
    }
}

impl<T> PtrSlice<&T> {
    pub unsafe fn pointerify(self) -> PtrSlice<*const T> {
        return PtrSlice { ptr: self.ptr as *const *const T, len: self.len };
    }
}


pub struct PtrSliceIter<'a, T> {
    slice: &'a PtrSlice<T>,
    i: usize,
}

impl<'a, T> Iterator for PtrSliceIter<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        if self.i >= self.slice.len {
            return None;
        }

        let item = unsafe { self.slice.ptr.add(self.i).as_ref().unwrap_unchecked() };
        self.i += 1;
        return Some(item);
    }
}