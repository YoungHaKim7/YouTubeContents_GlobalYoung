use std::fmt;
use std::mem::replace;
use std::ops::{Index, IndexMut, Range};
use std::sync::atomic::Ordering;
use std::{ptr, slice};

use pi_null::Null;
use pi_share::{ShareMutex, SharePtr};

const BUCKETS: usize = (u32::BITS as usize) - SKIP_BUCKET;
const MAX_ENTRIES: usize = (u32::MAX as usize) - SKIP;

/// Creates a [`Arr`] containing the given elements.
///
/// `arr!` allows `Arr`s to be defined with the same syntax as array expressions.
/// There are two forms of this macro:
///
/// - Create a [`Arr`] containing a given list of elements:
///
/// ```
/// let arr = arr![1, 2, 3];
/// assert_eq!(arr[0], 1);
/// assert_eq!(arr[1], 2);
/// assert_eq!(arr[2], 3);
/// ```
///
/// - Create a [`Arr`] from a given element and size:
///
/// ```
/// let arr = arr![1; 3];
/// assert_eq!(arr[0], 1);
/// assert_eq!(arr[1], 1);
/// assert_eq!(arr[2], 1);
/// ```
#[macro_export]
macro_rules! arr {
    () => {
        $crate::Arr::new()
    };
    ($elem:expr; $n:expr) => {{
        let mut arr = $crate::Arr::with_capacity($n);
        arr.extend(::core::iter::repeat($elem).take($n));
        arr
    }};
    ($($x:expr),+ $(,)?) => (
        <$crate::Arr<_> as core::iter::FromIterator<_>>::from_iter([$($x),+])
    );
}

/// A lock-free, auto-expansion array.
///
/// See [the crate documentation](crate) for details.
///
/// # Notes
///
/// The bucket array is stored inline, meaning that the
/// `Arr<T>` is quite large. It is expected that you
/// store it behind an [`Arc`](std::sync::Arc) or similar.
#[derive(Default)]
pub struct Arr<T: Null> {
    // buckets of length 32, 64 .. 2^32
    buckets: [Bucket<T>; BUCKETS],
    lock: ShareMutex<()>,
}

unsafe impl<T: Send + Null> Send for Arr<T> {}
unsafe impl<T: Sync + Null> Sync for Arr<T> {}

impl<T: Null> Arr<T> {
    /// Constructs a new, empty `Arr<T>`.
    ///
    /// # Examples
    ///
    /// ```
    /// let arr: pi_arr::arr<i32> = pi_arr::arr::new();
    /// ```
    #[inline]
    pub fn new() -> Arr<T> {
        Arr::with_capacity(0)
    }

    /// Constructs a new, empty `Arr<T>` with the specified capacity.
    ///
    /// Capacity will be aligned to a power of 2 size.
    /// The array will be able to hold at least `capacity` elements
    /// without reallocating.
    ///
    /// # Examples
    ///
    /// ```
    /// let arr = pi_arr::arr::with_capacity(10);
    ///
    /// for i in 0..32 {
    ///     // will not allocate
    ///     arr.set(i, i);
    /// }
    ///
    /// // may allocate
    /// arr.set(33, 33);
    /// ```
    pub fn with_capacity(capacity: usize) -> Arr<T> {
        let mut buckets = [ptr::null_mut(); BUCKETS];
        if capacity == 0 {
            return Arr {
                buckets: buckets.map(Bucket::new),
                lock: ShareMutex::default(),
            };
        }
        let init = Location::of(capacity).bucket;
        for (i, bucket) in buckets[..=init].iter_mut().enumerate() {
            let len = Location::bucket_len(i);
            *bucket = Bucket::alloc(len);
        }

        Arr {
            buckets: buckets.map(Bucket::new),
            lock: ShareMutex::default(),
        }
    }
    /// Returns the number of elements in the array.
    ///
    /// Since there is a default value, a bucket is allocated,
    /// and the length is automatically increased by the length of the bucket.
    /// Bucket lengths start at 32 as powers of 2.
    /// # Examples
    ///
    /// ```
    /// let arr = pi_arr::arr::new();
    /// assert_eq!(arr.len(), 0);
    /// arr.set(1, 1);
    /// arr.set(2, 2);
    /// assert_eq!(arr.len(), 32);
    /// ```
    pub fn len(&self) -> usize {
        let mut len = 0;
        for i in 0..self.buckets.len() {
            let entries = self.entries(i);
            // bucket is uninitialized
            if entries.is_null() {
                continue;
            }
            len += Location::bucket_len(i);
        }
        len
    }

    /// Returns a reference to the element at the given index.
    ///
    /// # Examples
    ///
    /// ```
    /// let arr = pi_arr::arr![10, 40, 30];
    /// assert_eq!(Some(&40), arr.get(1));
    /// assert_eq!(None, arr.get(33));
    /// ```
    pub fn get(&self, index: usize) -> Option<&T> {
        let location = Location::of(index);

        // safety: `location.bucket` is always in bounds
        let entries = self.entries(location.bucket);

        // bucket is uninitialized
        if entries.is_null() {
            return None;
        }

        // safety: `location.entry` is always in bounds for it's bucket
        Some(unsafe { &*entries.add(location.entry) })
    }

    /// Returns a reference to an element, without doing bounds
    /// checking or verifying that the element is fully initialized.
    ///
    /// For a safe alternative see [`get`](Arr::get).
    ///
    /// # Safety
    ///
    /// Calling this method with an out-of-bounds index, or for an element that
    /// is being concurrently initialized is **undefined behavior**, even if
    /// the resulting reference is not used.
    ///
    /// # Examples
    ///
    /// ```
    /// let arr = pi_arr::arr![1, 2, 4];
    ///
    /// unsafe {
    ///     assert_eq!(arr.get_unchecked(1), &2);
    /// }
    /// ```
    pub unsafe fn get_unchecked(&self, index: usize) -> &T {
        let location = Location::of(index);

        // safety: caller guarantees the entry is initialized
        &*self.entries(location.bucket).add(location.entry)
    }

    /// Returns a mutable reference to the element at the given index.
    ///
    /// # Examples
    ///
    /// ```
    /// let mut arr = pi_arr::arr![10, 40, 30];
    /// assert_eq!(Some(&mut 40), arr.get_mut(1));
    /// assert_eq!(None, arr.get_mut(33));
    /// ```
    pub fn get_mut(&mut self, index: usize) -> Option<&mut T> {
        let location = Location::of(index);

        // safety: `location.bucket` is always in bounds
        let entries = unsafe {
            self.buckets
                .get_unchecked_mut(location.bucket)
                .entries
                .get_mut()
        };

        // bucket is uninitialized
        if entries.is_null() {
            return None;
        }

        // safety: `location.entry` is always in bounds for it's bucket
        Some(unsafe { &mut *entries.add(location.entry) })
    }

    /// Returns a mutable reference to an element, without doing bounds
    /// checking or verifying that the element is fully initialized.
    ///
    /// For a safe alternative see [`get`](Arr::get).
    ///
    /// # Safety
    ///
    /// Calling this method with an out-of-bounds index is **undefined
    /// behavior**, even if the resulting reference is not used.
    ///
    /// # Examples
    ///
    /// ```
    /// let mut arr = pi_arr::arr![1, 2, 4];
    ///
    /// unsafe {
    ///     assert_eq!(arr.get_unchecked_mut(1), &mut 2);
    /// }
    /// ```
    pub unsafe fn get_unchecked_mut(&mut self, index: usize) -> &mut T {
        let location = Location::of(index);

        // safety: caller guarantees the entry is initialized
        &mut *self
            .buckets
            .get_unchecked_mut(location.bucket)
            .entries
            .get_mut()
            .add(location.entry)
    }
    /// Returns a mutable reference to the element at the given index.
    /// If the bucket corresponding to the index is not allocated,
    /// it will be allocated automatically, and the returned T is null
    ///
    /// # Examples
    ///
    /// ```
    /// let mut arr = pi_arr::arr![10, 40, 30];
    /// let arr = pi_arr::arr![10, 40, 30];
    /// assert_eq!(40, *arr.get_alloc_mut(1));
    /// assert_eq!(true, arr.get_alloc_mut(3).is_null());
    /// ```
    pub fn get_alloc_mut(&mut self, index: usize) -> &mut T {
        let location = Location::of(index);
        let bucket = unsafe { self.buckets.get_unchecked_mut(location.bucket) };
        // safety: `location.bucket` is always in bounds
        let mut entries = *bucket.entries.get_mut();

        // bucket is uninitialized
        if entries.is_null() {
            entries = Bucket::alloc(location.bucket_len);
        }

        // safety: `location.entry` is always in bounds for it's bucket
        unsafe { &mut *entries.add(location.entry) }
    }
    /// set element at the given index.
    ///
    /// # Examples
    ///
    /// ```
    /// let mut arr = pi_arr::arr![10, 40, 30];
    /// assert_eq!(40, arr.set(1, 20));
    /// assert_eq!(true, arr.set(33, 5).is_null());
    /// ```
    pub fn set(&mut self, index: usize, value: T) -> T {
        replace(self.get_alloc_mut(index), value)
    }
    /// Returns a mutable reference to the element at the given index.
    /// If the bucket corresponding to the index is not allocated,
    /// it will not be allocated automatically, and the returned None.
    ///
    /// # Examples
    ///
    /// ```
    /// let arr = pi_arr::arr![10, 40, 30];
    /// assert_eq!(10, *arr.load(0).unwrap());
    /// assert_eq!(Some(&mut 40), arr.load_mut(1));
    /// assert_eq!(true, arr.load(3).unwrap().is_null());
    /// assert_eq!(None, arr.load(33));
    /// ```
    pub fn load(&self, index: usize) -> Option<&mut T> {
        let location = Location::of(index);

        // safety: `location.bucket` is always in bounds
        let entries = self.entries(location.bucket);

        // bucket is uninitialized
        if entries.is_null() {
            return None;
        }

        // safety: `location.entry` is always in bounds for it's bucket
        Some(unsafe { &mut *entries.add(location.entry) })
    }

    /// Returns a mutable reference to an element, without doing bounds
    /// checking or verifying that the element is fully initialized.
    ///
    /// For a safe alternative see [`get`](Arr::get).
    ///
    /// # Safety
    ///
    /// Calling this method with an out-of-bounds index is **undefined
    /// behavior**, even if the resulting reference is not used.
    ///
    /// # Examples
    ///
    /// ```
    /// let arr = pi_arr::arr![1, 2, 4];
    ///
    /// unsafe {
    ///     assert_eq!(arr.load_unchecked(1), &mut 2);
    /// }
    /// ```
    pub unsafe fn load_unchecked(&self, index: usize) -> &mut T {
        let location = Location::of(index);
        // safety: caller guarantees the entry is initialized
        &mut *self.entries(location.bucket).add(location.entry)
    }

    /// Returns a mutable reference to the element at the given index.
    /// If the bucket corresponding to the index is not allocated,
    /// it will be allocated automatically, and the returned T is null
    /// # Examples
    ///
    /// ```
    /// let arr = pi_arr::arr![10, 40, 30];
    /// assert_eq!(40, *arr.load_alloc(1));
    /// assert_eq!(true, arr.load_alloc(3).is_null());
    /// ```
    pub fn load_alloc(&self, index: usize) -> &mut T {
        let location = Location::of(index);
        let bucket = unsafe { self.buckets.get_unchecked(location.bucket) };
        // safety: `location.bucket` is always in bounds
        let mut entries = bucket.entries.load(Ordering::Acquire);
        // bucket is uninitialized
        if entries.is_null() {
            entries = bucket.init(location.bucket_len, &self.lock);
        }
        // safety: `location.entry` is always in bounds for it's bucket
        unsafe { &mut *entries.add(location.entry) }
    }

    /// insert an element at the given index.
    ///
    /// # Examples
    ///
    /// ```
    /// let arr = pi_arr::arr![1, 2];
    /// arr.insert(2, 3);
    /// assert_eq!(arr[0], 1);
    /// assert_eq!(arr[1], 2);
    /// assert_eq!(arr[2], 3);
    /// ```
    #[inline]
    pub fn insert(&self, index: usize, value: T) -> T {
        replace(&mut *self.load_alloc(index), value)
    }
    /// clear and free buckets.
    ///
    /// # Examples
    ///
    /// ```
    /// let arr = pi_arr::arr![1, 2];
    /// arr.clear();
    /// arr.set(2, 3);
    /// assert_eq!(arr[0], 0);
    /// assert_eq!(arr[1], 0);
    /// assert_eq!(arr[2], 3);
    /// ```
    pub fn clear(&self) {
        for (i, bucket) in self.buckets.iter().enumerate() {
            let entries = bucket.entries.load(Ordering::Acquire);
            if entries.is_null() {
                continue;
            }
            let len = Location::bucket_len(i);
            // safety: in drop
            unsafe { drop(Box::from_raw(slice::from_raw_parts_mut(entries, len))) }
        }
    }
    /// Returns an iterator over the array.
    ///
    /// Values are yielded in the form `(index, value)`. The array may
    /// have in-progress concurrent writes that create gaps, so `index`
    /// may not be strictly sequential.
    ///
    /// # Examples
    ///
    /// ```
    /// let arr = pi_arr::arr![1, 2, 4];
    /// let mut iterator = arr.iter();
    ///
    /// let r = iterator.next().unwrap();
    /// assert_eq!((r.0, *r.1), (0, 1));
    /// let r = iterator.next().unwrap();
    /// assert_eq!((r.0, *r.1), (1, 2));
    /// let r = iterator.next().unwrap();
    /// assert_eq!((r.0, *r.1), (2, 4));
    /// let r = iterator.next().unwrap();
    /// assert_eq!((r.0, r.1.is_null()), (3, true));
    /// assert_eq!(iterator.size_hint().0, 32-4);
    /// ```
    #[inline]
    pub fn iter(&self) -> Iter<'_, T> {
        self.slice(0..MAX_ENTRIES)
    }

    /// Returns an iterator over the array at the given range.
    ///
    /// Values are yielded in the form `(index, Entry)`.
    ///
    /// # Examples
    ///
    /// ```
    /// let arr = pi_arr::arr![1, 2, 4, 6];
    /// let mut iterator = arr.slice(1..3);
    ///
    /// let r = iterator.next().unwrap();
    /// assert_eq!((r.0, *r.1), (1, 2));
    /// let r = iterator.next().unwrap();
    /// assert_eq!((r.0, *r.1), (2, 4));
    /// assert_eq!(iterator.next(), None);
    /// ```
    pub fn slice(&self, range: Range<usize>) -> Iter<'_, T> {
        let mut start = Location::of(range.start);
        let mut end = Location::of(range.end);
        let mut bucket_ptr = ptr::null_mut();
        while start.bucket <= end.bucket {
            bucket_ptr = self.entries(start.bucket);
            if !bucket_ptr.is_null() {
                break;
            }
            start.up();
        }
        let bucket_index = start.bucket_index();
        while end.bucket > start.bucket {
            let entries = self.entries(end.bucket);
            if !entries.is_null() {
                break;
            }
            end.down();
        }
        let size = if start.bucket < end.bucket {
            let mut size = end.entry + start.bucket_len - start.entry;
            for bucket in (start.bucket + 1)..end.bucket {
                let entries = self.entries(bucket);
                if !entries.is_null() {
                    size += Location::bucket_len(bucket);
                }
            }
            size
        } else if end.bucket == end.bucket {
            end.entry.checked_sub(start.entry).unwrap_or(0)
        } else {
            0
        };
        Iter {
            arr: &self,
            start,
            end,
            bucket_ptr,
            bucket_index,
            size,
        }
    }
    #[inline]
    fn entries(&self, bucket: usize) -> *mut T {
        unsafe {
            self.buckets
                .get_unchecked(bucket)
                .entries
                .load(Ordering::Acquire)
        }
    }
}

impl<T: Null> Index<usize> for Arr<T> {
    type Output = T;

    fn index(&self, index: usize) -> &Self::Output {
        self.get(index).expect("no element found at index {index}")
    }
}
impl<T: Null> IndexMut<usize> for Arr<T> {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        self.get_mut(index)
            .expect("no element found at index_mut {index}")
    }
}

impl<T: Null> Drop for Arr<T> {
    fn drop(&mut self) {
        for (i, bucket) in self.buckets.iter_mut().enumerate() {
            let entries = *bucket.entries.get_mut();
            if entries.is_null() {
                continue;
            }
            let len = Location::bucket_len(i);
            // safety: in drop
            unsafe { drop(Box::from_raw(slice::from_raw_parts_mut(entries, len))) }
        }
    }
}

impl<T: Null> FromIterator<T> for Arr<T> {
    fn from_iter<I: IntoIterator<Item = T>>(iter: I) -> Self {
        let iter = iter.into_iter();

        let (lower, _) = iter.size_hint();
        let mut arr = Arr::with_capacity(lower);
        for (i, value) in iter.enumerate() {
            *arr.get_alloc_mut(i) = value;
        }
        arr
    }
}

impl<T: Null> Extend<T> for Arr<T> {
    fn extend<I: IntoIterator<Item = T>>(&mut self, iter: I) {
        let iter = iter.into_iter();
        for (i, value) in iter.enumerate() {
            *self.get_alloc_mut(i) = value;
        }
    }
}

impl<T: Clone + Null> Clone for Arr<T> {
    fn clone(&self) -> Arr<T> {
        let mut buckets = [ptr::null_mut(); BUCKETS];

        for (i, bucket) in buckets.iter_mut().enumerate() {
            let entries = self.entries(i);

            // bucket is uninitialized
            if entries.is_null() {
                continue;
            }
            let len = Location::bucket_len(i);
            *bucket = Bucket::alloc(len);
            unsafe {
                ptr::copy(entries, *bucket, len);
            }
        }

        Arr {
            buckets: buckets.map(Bucket::new),
            lock: ShareMutex::default(),
        }
    }
}

impl<T: fmt::Debug + Null> fmt::Debug for Arr<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_list().entries(self.iter()).finish()
    }
}

impl<T: PartialEq + Null> PartialEq for Arr<T> {
    fn eq(&self, other: &Self) -> bool {
        if self.len() != other.len() {
            return false;
        }

        // ensure indexes are checked along with values to handle gaps in the array
        for (index, value) in self.iter() {
            if other.get(index) != Some(value) {
                return false;
            }
        }

        true
    }
}

impl<A, T: Null> PartialEq<A> for Arr<T>
where
    A: AsRef<[T]>,
    T: PartialEq,
{
    fn eq(&self, other: &A) -> bool {
        let other = other.as_ref();

        if self.len() != other.len() {
            return false;
        }

        // ensure indexes are checked along with values to handle gaps in the array
        for (index, value) in self.iter() {
            if other.get(index) != Some(value) {
                return false;
            }
        }

        true
    }
}

impl<T: Eq + Null> Eq for Arr<T> {}

/// An iterator over the elements of a [`Arr<T>`].
///
/// See [`Arr::iter`] for details.
pub struct Iter<'a, T: Null> {
    arr: &'a Arr<T>,
    start: Location,
    end: Location,
    bucket_ptr: *mut T,
    bucket_index: usize,
    size: usize,
}
impl<'a, T: Null> Iterator for Iter<'a, T> {
    type Item = (usize, &'a mut T);
    fn next(&mut self) -> Option<Self::Item> {
        loop {
            if self.size == 0 {
                return None;
            }
            // 用size来保护self.start.entry不越过self.end.entry
            self.size -= 1;
            if self.start.entry < self.start.bucket_len {
                let r = unsafe { &mut *self.bucket_ptr.add(self.start.entry) };
                let index = self.start.entry;
                self.start.entry += 1;
                if r.is_null() {
                    continue;
                }
                return Some((self.bucket_index + index, r));
            }
            self.start.up();
            self.bucket_index = self.start.bucket_index();
            while self.start.bucket <= self.end.bucket {
                self.bucket_ptr = self.arr.entries(self.start.bucket);
                if !self.bucket_ptr.is_null() {
                    break;
                }
                self.start.up();
                self.bucket_index = self.start.bucket_index();
            }
            self.start.entry += 1;
            let r = unsafe { &mut *self.bucket_ptr };
            if !r.is_null() {
                return Some((self.bucket_index, r));
            }
        }
    }
    fn size_hint(&self) -> (usize, Option<usize>) {
        (0, Some(self.size))
    }
}

#[derive(Default)]
struct Bucket<T: Null> {
    entries: SharePtr<T>,
}

impl<T: Null> Bucket<T> {
    fn new(entries: *mut T) -> Bucket<T> {
        Bucket {
            entries: SharePtr::new(entries),
        }
    }
    fn alloc(len: usize) -> *mut T {
        let entries = (0..len).map(|_| T::null()).collect::<Box<[T]>>();
        Box::into_raw(entries) as _
    }
    fn init(&self, len: usize, lock: &ShareMutex<()>) -> *mut T {
        let _lock = lock.lock();
        let mut ptr = self.entries.load(Ordering::Acquire);
        if ptr.is_null() {
            ptr = Bucket::alloc(len);
            self.entries.store(ptr, Ordering::Release);
        }
        ptr
    }
}

#[derive(Debug)]
struct Location {
    // the index of the bucket
    bucket: usize,
    // the length of `bucket`
    bucket_len: usize,
    // the index of the entry in `bucket`
    entry: usize,
}

// skip the shorter buckets to avoid unnecessary allocations.
// this also reduces the maximum capacity of a arr.
const SKIP: usize = 32;
const SKIP_BUCKET: usize = ((usize::BITS - SKIP.leading_zeros()) as usize) - 1;

impl Location {
    fn of(index: usize) -> Location {
        let skipped = index.checked_add(SKIP).expect("exceeded maximum length");
        let bucket = usize::BITS - skipped.leading_zeros();
        let bucket = (bucket as usize) - (SKIP_BUCKET + 1);
        let bucket_len = Location::bucket_len(bucket);
        let entry = skipped ^ bucket_len;

        Location {
            bucket,
            bucket_len,
            entry,
        }
    }
    #[inline]
    fn bucket_len(bucket: usize) -> usize {
        1 << (bucket + SKIP_BUCKET)
    }
    #[inline]
    fn bucket_index(&self) -> usize {
        ((u32::MAX as u64) >> (u32::BITS - self.bucket as u32) << SKIP_BUCKET) as usize
    }
    fn up(&mut self) {
        self.bucket += 1;
        self.bucket_len = Location::bucket_len(self.bucket);
        self.entry = 0;
    }
    fn down(&mut self) {
        self.bucket -= 1;
        self.bucket_len = Location::bucket_len(self.bucket);
        self.entry = self.bucket_len;
    }
}

#[cfg(test)]
mod tests {
    use std::sync::{Arc, Mutex};

    use crate::*;

    #[test]
    fn test() {
        let arr = arr![1; 3];
        assert_eq!(arr[0], 1);
        assert_eq!(arr[1], 1);
        assert_eq!(arr[2], 1);

        let arr = arr![];
        assert_eq!(arr.len(), 0);
        arr.insert(1, 1);
        arr.insert(2, 2);
        assert_eq!(arr.len(), 32);

        let mut arr = arr![10, 40, 30];
        assert_eq!(40, *arr.get_alloc_mut(1));
        assert_eq!(true, arr.get_alloc_mut(3).is_null());
        assert_eq!(40, arr.set(1, 20));
        assert_eq!(true, arr.set(33, 5).is_null());

        let arr = arr![10, 40, 30];
        assert_eq!(Some(&40), arr.get(1));
        assert_eq!(None, arr.get(33));

        let arr = crate::arr![1, 2, 4];
        unsafe {
            assert_eq!(arr.get_unchecked(1), &2);
        }

        let mut arr = arr![10, 40, 30];
        assert_eq!(Some(&mut 40), arr.get_mut(1));
        assert_eq!(None, arr.get_mut(33));

        let mut arr = crate::arr![1, 2, 4];
        unsafe {
            assert_eq!(arr.get_unchecked_mut(1), &mut 2);
        }

        let arr = arr![10, 40, 30];
        assert_eq!(40, *arr.load_alloc(1));
        assert_eq!(true, arr.load_alloc(3).is_null());
        assert_eq!(true, arr.load_alloc(133).is_null());

        let arr = arr![10, 40, 30];
        assert_eq!(40, *arr.load(1).unwrap());
        assert_eq!(true, arr.load(3).unwrap().is_null());
        assert_eq!(None, arr.load(133));

        let arr = crate::arr![1, 2];
        arr.insert(2, 3);
        assert_eq!(arr[0], 1);
        assert_eq!(arr[1], 2);
        assert_eq!(arr[2], 3);

        let arr = crate::arr![1, 2, 4];
        arr.insert(1111, 1111);
        let mut iterator = arr.iter();
        let r = iterator.next().unwrap();
        assert_eq!((r.0, *r.1), (0, 1));
        let r = iterator.next().unwrap();
        assert_eq!((r.0, *r.1), (1, 2));
        let r = iterator.next().unwrap();
        assert_eq!((r.0, *r.1), (2, 4));
        let r = iterator.next().unwrap();
        assert_eq!((r.0, *r.1), (1111, 1111));
        assert_eq!(iterator.next(), None);
        assert_eq!(iterator.size_hint().0, 0);

        let mut iterator = arr.slice(1..3);
        let r = iterator.next().unwrap();
        assert_eq!((r.0, *r.1), (1, 2));
        let r = iterator.next().unwrap();
        assert_eq!((r.0, *r.1), (2, 4));
        assert_eq!(iterator.next(), None);
    }
    #[test]
    fn test_arc() {
        let arr = Arc::new(crate::Arr::new());

        // spawn 6 threads that append to the arr
        let threads = (0..6)
            .map(|i| {
                let arr = arr.clone();

                std::thread::spawn(move || {
                    arr.insert(i, i);
                })
            })
            .collect::<Vec<_>>();

        // wait for the threads to finish
        for thread in threads {
            thread.join().unwrap();
        }

        for i in 0..6 {
            assert!(arr.iter().any(|(_, x)| x == &i));
        }
    }
    #[test]
    fn test_mutex() {
        let arr = Arc::new(crate::Arr::new());

        // set an element
        arr.insert(0, Some(Mutex::new(1)));

        let thread = std::thread::spawn({
            let arr = arr.clone();
            move || {
                // mutate through the mutex
                *(arr[0].as_ref().unwrap().lock().unwrap()) += 1;
            }
        });

        thread.join().unwrap();

        let x = arr[0].as_ref().unwrap().lock().unwrap();
        assert_eq!(*x, 2);
    }
    #[test]
    fn location() {
        assert_eq!(Location::bucket_len(0), 32);
        for i in 0..32 {
            let loc = Location::of(i);
            assert_eq!(loc.bucket_len, 32);
            assert_eq!(loc.bucket, 0);
            assert_eq!(loc.entry, i);
            assert_eq!(loc.bucket_index(), 0)
        }

        assert_eq!(Location::bucket_len(1), 64);
        for i in 33..96 {
            let loc = Location::of(i);
            assert_eq!(loc.bucket_len, 64);
            assert_eq!(loc.bucket, 1);
            assert_eq!(loc.entry, i - 32);
            assert_eq!(loc.bucket_index(), 32)
        }

        assert_eq!(Location::bucket_len(2), 128);
        for i in 96..224 {
            let loc = Location::of(i);
            assert_eq!(loc.bucket_len, 128);
            assert_eq!(loc.bucket, 2);
            assert_eq!(loc.entry, i - 96);
            assert_eq!(loc.bucket_index(), 96)
        }

        let max = Location::of(MAX_ENTRIES);
        assert_eq!(max.bucket, BUCKETS - 1);
        assert_eq!(max.bucket_len, 1 << 31);
        assert_eq!(max.entry, (1 << 31) - 1);
    }
}
