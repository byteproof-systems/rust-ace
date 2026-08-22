#![allow(dead_code)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(unused_assignments)]
#![allow(unused_braces)]
#![allow(unused_mut)]
#![allow(unused_parens)]
#![allow(unused_unsafe)]
#![allow(unused_variables)]
#![allow(clippy::all)]
#![feature(c_variadic)]
#[allow(dead_code, non_snake_case, unused_unsafe, clippy::all)]
pub mod __cxx_std {
    use core::ptr;
    /// `std::allocator<T>`: a stateless, zero-sized allocator. The
    /// `std::allocator_traits` static interface that operates on it is lowered
    /// directly to allocation / placement construction at the call site, so the
    /// marker carries no state.
    #[derive(Copy, Clone)]
    pub struct Allocator;
    /// `std::equal_to<T>` — a stateless comparison functor marker; the
    /// `operator()` call is intercepted at the call site as `a == b`.
    #[derive(Copy, Clone)]
    pub struct EqualTo;
    /// `std::char_traits<char>` — a STATELESS traits class: every member is a
    /// static function over `char`/`int` (`eof`, `to_int_type`, …). Mapped to a
    /// zero-sized marker; the static interface is lowered directly at the call
    /// site (see `lower_char_traits_call`), so the marker carries no state. The
    /// `unsigned char` / `signed char` specialisations collapse onto it too.
    #[derive(Copy, Clone)]
    pub struct CharTraits;
    /// `std::integral_constant<T, v>` (`std::true_type` / `std::false_type`) —
    /// a STATELESS compile-time tag whose only run-time use is overload
    /// selection by tag dispatch (`dump_float(x, std::true_type{})`); the
    /// overload was already resolved by clang, and the `::value` constant is
    /// folded at each use site, so the marker carries no state.
    #[derive(Copy, Clone)]
    pub struct IntegralConstant;
    /// The <iterator> category tags (`std::input_iterator_tag` …
    /// `std::random_access_iterator_tag`). They are empty, stateless markers
    /// used only as the compile-time `iterator_category` member typedef and as
    /// tag-dispatch arguments (resolved by clang before lowering), so the whole
    /// family collapses onto this one zero-sized marker — exactly as the
    /// <stdexcept> hierarchy collapses onto `Exception`.
    #[derive(Copy, Clone)]
    pub struct IteratorTag;
    /// `std::vector<T>`: a contiguous, growable, owning sequence. Backed by raw
    /// `libc` allocation; capacity grows geometrically (doubling), matching the
    /// amortised-O(1) `push_back` of libstdc++. Rust values are trivially
    /// relocatable, so `realloc`'s bitwise move is sound for any `T`.
    pub struct Vector<T> {
        ptr: *mut T,
        len: usize,
        cap: usize,
    }
    impl<T> Vector<T> {
        #[inline]
        pub fn new() -> Self {
            Vector {
                ptr: ptr::null_mut(),
                len: 0,
                cap: 0,
            }
        }
        #[inline]
        unsafe fn grow_to(&mut self, want: usize) {
            if want <= self.cap {
                return;
            }
            let mut newcap = if self.cap == 0 { 1usize } else { self.cap * 2 };
            if newcap < want {
                newcap = want;
            }
            let elem = core::mem::size_of::<T>();
            if elem == 0 {
                self.cap = newcap;
                return;
            }
            let nbytes = newcap
                .checked_mul(elem)
                .expect("std::vector capacity overflow");
            let np = libc::realloc(self.ptr as *mut libc::c_void, nbytes) as *mut T;
            assert!(! np.is_null(), "std::vector allocation failed");
            self.ptr = np;
            self.cap = newcap;
        }
        #[inline]
        pub unsafe fn push_back(&mut self, value: T) {
            if self.len == self.cap {
                self.grow_to(self.len + 1);
            }
            if core::mem::size_of::<T>() != 0 {
                ptr::write(self.ptr.add(self.len), value);
            } else {
                core::mem::forget(value);
            }
            self.len += 1;
        }
        /// Grow to at least `want` capacity, RELOCATING the existing elements
        /// with `relocate(dst, src)` — move-construct into the new buffer,
        /// then destroy the moved-from source: the exact C++ vector-growth
        /// recipe for an element type whose move-constructor/destructor pair
        /// is not equivalent to a bitwise move. `None` relocates bitwise via
        /// [`Self::grow_to`]'s `realloc`, which IS that recipe's observable
        /// effect for every element the lowering proved trivially
        /// relocatable.
        #[inline]
        unsafe fn grow_to_relocating(
            &mut self,
            want: usize,
            relocate: Option<unsafe fn(*mut T, *mut T)>,
        ) {
            let Some(f) = relocate else {
                self.grow_to(want);
                return;
            };
            if want <= self.cap {
                return;
            }
            let mut newcap = if self.cap == 0 { 1usize } else { self.cap * 2 };
            if newcap < want {
                newcap = want;
            }
            let elem = core::mem::size_of::<T>();
            if elem == 0 {
                self.cap = newcap;
                return;
            }
            let nbytes = newcap
                .checked_mul(elem)
                .expect("std::vector capacity overflow");
            let np = libc::malloc(nbytes) as *mut T;
            assert!(! np.is_null(), "std::vector allocation failed");
            let mut i = 0usize;
            while i < self.len {
                f(np.add(i), self.ptr.add(i));
                i += 1;
            }
            if !self.ptr.is_null() {
                libc::free(self.ptr as *mut libc::c_void);
            }
            self.ptr = np;
            self.cap = newcap;
        }
        /// `push_back(x)` for an element type with a non-bitwise
        /// move/copy constructor: C++ move-constructs the new element into
        /// its slot from the argument (then destroys the moved-from
        /// temporary), and growth relocates the existing elements with the
        /// same recipe — `relocate` runs exactly where C++ runs the
        /// constructor + destructor pair. `None` is exactly
        /// [`Self::push_back`].
        #[inline]
        pub unsafe fn push_back_relocating(
            &mut self,
            value: T,
            relocate: Option<unsafe fn(*mut T, *mut T)>,
        ) {
            let Some(f) = relocate else {
                self.push_back(value);
                return;
            };
            if self.len == self.cap {
                self.grow_to_relocating(self.len + 1, relocate);
            }
            if core::mem::size_of::<T>() != 0 {
                let mut value = core::mem::ManuallyDrop::new(value);
                f(self.ptr.add(self.len), &mut *value as *mut T);
            } else {
                core::mem::forget(value);
            }
            self.len += 1;
        }
        /// `emplace_back(args…)` for an element type with a non-bitwise move
        /// constructor: C++ constructs the new element IN PLACE — `value` is
        /// that construction's result, written directly to its slot with NO
        /// extra move (unlike `push_back`) — while buffer growth still
        /// relocates the existing elements with `relocate`.
        #[inline]
        pub unsafe fn emplace_back_relocating(
            &mut self,
            value: T,
            relocate: Option<unsafe fn(*mut T, *mut T)>,
        ) {
            if self.len == self.cap {
                self.grow_to_relocating(self.len + 1, relocate);
            }
            if core::mem::size_of::<T>() != 0 {
                ptr::write(self.ptr.add(self.len), value);
            } else {
                core::mem::forget(value);
            }
            self.len += 1;
        }
        /// `reserve(n)` with growth relocation (see
        /// [`Self::grow_to_relocating`]).
        #[inline]
        pub unsafe fn reserve_relocating(
            &mut self,
            want: usize,
            relocate: Option<unsafe fn(*mut T, *mut T)>,
        ) {
            self.grow_to_relocating(want, relocate);
        }
        /// `resize(n)` with growth relocation; the truncation/extension
        /// behaviour is exactly [`Self::resize_default`] (whose own
        /// `grow_to` is a no-op after the relocating growth here).
        #[inline]
        pub unsafe fn resize_default_relocating(
            &mut self,
            n: usize,
            relocate: Option<unsafe fn(*mut T, *mut T)>,
        ) {
            if n > self.len {
                self.grow_to_relocating(n, relocate);
            }
            self.resize_default(n);
        }
        /// `resize(n)` whose extension tail is VALUE-INITIALISED by running
        /// `value_init` (a class type's default constructor) for each new
        /// element, rather than zero-filled — used when the element's value-init
        /// runs code that is not provably zero. Truncation (`n <= len`) drops the
        /// tail exactly as [`Self::resize_default`].
        #[inline]
        pub unsafe fn resize_with_relocating(
            &mut self,
            n: usize,
            value_init: unsafe fn() -> T,
            relocate: Option<unsafe fn(*mut T, *mut T)>,
        ) {
            if n <= self.len {
                self.resize_default(n);
                return;
            }
            self.grow_to_relocating(n, relocate);
            if core::mem::size_of::<T>() != 0 {
                while self.len < n {
                    ptr::write(self.ptr.add(self.len), value_init());
                    self.len += 1;
                }
            } else {
                self.len = n;
            }
        }
        /// `insert(pos, first, last)` — the iterator-range insert. The tail
        /// `[idx, len)` SHIFTS up by `n`: for a non-bitwise element the shift
        /// RELOCATES each element with the move thunk (high-to-low, so every
        /// overwritten destination is either uninitialised tail or an
        /// already-relocated husk), exactly the move C++'s insert performs; a
        /// trivially-relocatable element shifts bitwise. The INSERTED range is
        /// COPY-constructed: `std::vector::insert(first, last)` copies each
        /// `[first, last)` element, so `copy_elem` (when present)
        /// clone-constructs each slot from the source — a bitwise copy of an
        /// OWNING element (`basic_json`, `String`) would alias the source
        /// payload and double-free it on destruction. `None` keeps the bitwise
        /// copy for a trivially-copyable element.
        #[inline]
        pub unsafe fn insert_range_relocating(
            &mut self,
            pos: *mut T,
            first: *const T,
            last: *const T,
            relocate: Option<unsafe fn(*mut T, *mut T)>,
            copy_elem: Option<unsafe fn(*mut T, *const T)>,
        ) -> *mut T {
            let idx = if self.ptr.is_null() {
                0usize
            } else {
                pos.offset_from(self.ptr) as usize
            };
            let n = if (last as usize) > (first as usize) {
                (last as usize - first as usize) / core::mem::size_of::<T>().max(1)
            } else {
                0
            };
            if n == 0 {
                return self.ptr.add(idx);
            }
            self.grow_to_relocating(self.len + n, relocate);
            if core::mem::size_of::<T>() != 0 {
                match relocate {
                    Some(reloc) => {
                        let mut j = self.len;
                        while j > idx {
                            j -= 1;
                            reloc(self.ptr.add(j + n), self.ptr.add(j));
                        }
                    }
                    None => {
                        ptr::copy(
                            self.ptr.add(idx),
                            self.ptr.add(idx + n),
                            self.len - idx,
                        );
                    }
                }
                match copy_elem {
                    Some(copy) => {
                        let mut k = 0usize;
                        while k < n {
                            copy(self.ptr.add(idx + k), first.add(k));
                            k += 1;
                        }
                    }
                    None => {
                        ptr::copy_nonoverlapping(first, self.ptr.add(idx), n);
                    }
                }
            }
            self.len += n;
            self.ptr.add(idx)
        }
        /// `insert(pos, value)` — the single-element insert. The tail `[idx,
        /// len)` shifts up by one (relocating with the move thunk high-to-low
        /// for a non-bitwise element, bitwise otherwise — see
        /// [`Self::insert_range_relocating`]), then the moved/copied `value` is
        /// written into the vacated slot. Returns the iterator to the inserted
        /// element.
        #[inline]
        pub unsafe fn insert_one_relocating(
            &mut self,
            pos: *mut T,
            value: T,
            relocate: Option<unsafe fn(*mut T, *mut T)>,
        ) -> *mut T {
            let idx = if self.ptr.is_null() {
                0usize
            } else {
                pos.offset_from(self.ptr) as usize
            };
            self.grow_to_relocating(self.len + 1, relocate);
            if core::mem::size_of::<T>() != 0 {
                match relocate {
                    Some(reloc) => {
                        let mut j = self.len;
                        while j > idx {
                            j -= 1;
                            reloc(self.ptr.add(j + 1), self.ptr.add(j));
                        }
                    }
                    None => {
                        ptr::copy(
                            self.ptr.add(idx),
                            self.ptr.add(idx + 1),
                            self.len - idx,
                        );
                    }
                }
                ptr::write(self.ptr.add(idx), value);
            } else {
                core::mem::forget(value);
            }
            self.len += 1;
            self.ptr.add(idx)
        }
        #[inline]
        pub fn size(&self) -> libc::c_ulong {
            self.len as libc::c_ulong
        }
        /// `std::vector::capacity()` — the allocated element capacity. nlohmann
        /// reads it before/after `push_back` to detect a reallocation (then
        /// re-seats parent pointers); this reflects the support vector's
        /// doubling growth, so the comparison is observably the libstdc++ one.
        #[inline]
        pub fn capacity(&self) -> libc::c_ulong {
            self.cap as libc::c_ulong
        }
        /// `std::move(src.begin(), src.end(), std::back_inserter(*dest))` over a
        /// WHOLE container: append every element of `src` to `dest` and leave
        /// `src` empty. libstdc++ move-assigns each element, leaving the source
        /// elements moved-from; the source is always cleared / destroyed right
        /// after, so emptying `src` here is observably identical — and, unlike a
        /// bitwise copy of owning elements, never double-drops (the moved-out
        /// elements are no longer owned by `src`, so its `len` is reset to 0
        /// without dropping them).
        #[inline]
        pub unsafe fn move_all_into(src: *mut Vector<T>, dest: *mut Vector<T>) {
            let n = (*src).len;
            let mut i = 0usize;
            while i < n {
                let v = ptr::read((*src).ptr.add(i));
                (*dest).push_back(v);
                i += 1;
            }
            (*src).len = 0;
        }
        #[inline]
        pub fn empty(&self) -> bool {
            self.len == 0
        }
        /// Address of element `i` — the UNCHECKED `operator[]` storage
        /// location (out-of-range is UB in C++ too, so no check).
        #[inline]
        pub unsafe fn at(&self, i: usize) -> *mut T {
            self.ptr.add(i)
        }
        /// `at(i)` — the BOUNDS-CHECKED element accessor. `std::vector::at`
        /// throws `std::out_of_range` when `i >= size()`; under `panic =
        /// abort` that throw is observably a process abort, so a faithful
        /// translation aborts here too rather than forming an out-of-bounds
        /// pointer (the silent UB the unchecked `at` would produce).
        #[inline]
        pub unsafe fn at_checked(&self, i: usize) -> *mut T {
            if i >= self.len {
                panic!("std::vector::at: index out of range");
            }
            self.ptr.add(i)
        }
        /// Pointer past the last element (`end()`), for iteration.
        #[inline]
        pub unsafe fn end_ptr(&self) -> *mut T {
            self.ptr.add(self.len)
        }
        /// Pointer to the first element (`begin()`).
        #[inline]
        pub unsafe fn begin_ptr(&self) -> *mut T {
            self.ptr
        }
        /// Storage location of the last element (`back()`).
        #[inline]
        pub unsafe fn back(&self) -> *mut T {
            self.ptr.add(self.len - 1)
        }
        /// Storage location of the first element (`front()`).
        #[inline]
        pub unsafe fn front(&self) -> *mut T {
            self.ptr
        }
        /// `pop_back()`: drop the last element and shrink.
        #[inline]
        pub unsafe fn pop_back(&mut self) {
            if self.len != 0 {
                self.len -= 1;
                if core::mem::size_of::<T>() != 0 {
                    ptr::drop_in_place(self.ptr.add(self.len));
                }
            }
        }
        /// `reserve(n)`: grow-only capacity request (`std::vector::reserve`).
        #[inline]
        pub unsafe fn reserve(&mut self, want: usize) {
            self.grow_to(want);
        }
        /// `resize(n)`: truncate (dropping the removed tail elements) or extend
        /// with VALUE-INITIALISED elements (`std::vector::resize`; the
        /// translated element types are C-layout records/scalars, for which
        /// value-initialisation is all-zero).
        #[inline]
        pub unsafe fn resize_default(&mut self, n: usize) {
            if n < self.len {
                if core::mem::size_of::<T>() != 0 {
                    let mut i = n;
                    while i < self.len {
                        ptr::drop_in_place(self.ptr.add(i));
                        i += 1;
                    }
                }
                self.len = n;
                return;
            }
            self.grow_to(n);
            if core::mem::size_of::<T>() != 0 {
                let mut i = self.len;
                while i < n {
                    ptr::write(self.ptr.add(i), core::mem::zeroed());
                    i += 1;
                }
            }
            self.len = n;
        }
        /// `insert(pos, first, last)` — the iterator-range insert: shift the
        /// tail up by the range's length and COPY `[first, last)` in (the
        /// corpus inserts byte/scalar ranges — nlohmann's
        /// `output_vector_adapter::write_characters` appending `[s, s + n)` —
        /// so a bitwise copy IS the element copy). Returns the iterator to the
        /// first inserted element.
        #[inline]
        pub unsafe fn insert_range(
            &mut self,
            pos: *mut T,
            first: *const T,
            last: *const T,
        ) -> *mut T {
            let idx = if self.ptr.is_null() {
                0usize
            } else {
                pos.offset_from(self.ptr) as usize
            };
            let n = if (last as usize) > (first as usize) {
                (last as usize - first as usize) / core::mem::size_of::<T>().max(1)
            } else {
                0
            };
            if n == 0 {
                return self.ptr.add(idx);
            }
            self.grow_to(self.len + n);
            if core::mem::size_of::<T>() != 0 {
                ptr::copy(self.ptr.add(idx), self.ptr.add(idx + n), self.len - idx);
                ptr::copy_nonoverlapping(first, self.ptr.add(idx), n);
            }
            self.len += n;
            self.ptr.add(idx)
        }
        /// `clear()`: drop every element; capacity is retained.
        #[inline]
        pub unsafe fn clear(&mut self) {
            if core::mem::size_of::<T>() != 0 {
                let mut i = 0usize;
                while i < self.len {
                    ptr::drop_in_place(self.ptr.add(i));
                    i += 1;
                }
            }
            self.len = 0;
        }
        /// `erase(pos)`: remove the element at iterator `pos` (a pointer into
        /// the storage), drop it, shift the tail down one slot, and return the
        /// iterator to the element that now occupies `pos` — exactly
        /// `std::vector::erase(const_iterator)`. `pos == end()` returns `end()`.
        #[inline]
        pub unsafe fn erase(&mut self, pos: *mut T) -> *mut T {
            if core::mem::size_of::<T>() == 0 {
                if self.len != 0 {
                    self.len -= 1;
                }
                return self.ptr;
            }
            let idx = pos.offset_from(self.ptr) as usize;
            if idx >= self.len {
                return self.end_ptr();
            }
            ptr::drop_in_place(self.ptr.add(idx));
            let tail = self.len - idx - 1;
            if tail != 0 {
                ptr::copy(self.ptr.add(idx + 1), self.ptr.add(idx), tail);
            }
            self.len -= 1;
            self.ptr.add(idx)
        }
        /// `max_size()`: the theoretical element-count ceiling.
        #[inline]
        pub fn max_size(&self) -> libc::c_ulong {
            let e = core::mem::size_of::<T>().max(1);
            (usize::MAX / e) as libc::c_ulong
        }
    }
    impl<T> Drop for Vector<T> {
        fn drop(&mut self) {
            unsafe {
                if core::mem::size_of::<T>() != 0 {
                    let mut i = 0usize;
                    while i < self.len {
                        ptr::drop_in_place(self.ptr.add(i));
                        i += 1;
                    }
                    if !self.ptr.is_null() {
                        libc::free(self.ptr as *mut libc::c_void);
                    }
                }
            }
        }
    }
    /// `std::vector<T>(const std::vector<T>&)` — the deep copy constructor:
    /// allocate fresh storage and CLONE each element, so the copy owns
    /// independent elements (requires `T: Clone`) and never aliases or
    /// double-frees the source's. A `T` whose own copy is a deep one (an
    /// owning support type, a record with a copy constructor) clones deeply;
    /// a trivially-`Copy` `T` clones bitwise.
    impl<T: Clone> Clone for Vector<T> {
        fn clone(&self) -> Self {
            let mut v = Vector::new();
            if core::mem::size_of::<T>() == 0 {
                v.len = self.len;
                return v;
            }
            unsafe {
                v.grow_to(self.len);
                let mut i = 0usize;
                while i < self.len {
                    ptr::write(v.ptr.add(i), (*self.ptr.add(i)).clone());
                    i += 1;
                }
                v.len = self.len;
            }
            v
        }
    }
    /// `std::vector<T>::operator==` — equal length and element-wise equal
    /// elements (libstdc++'s `std::equal` over the two ranges). Element
    /// comparison uses `T`'s own equality.
    impl<T: PartialEq> PartialEq for Vector<T> {
        fn eq(&self, other: &Self) -> bool {
            if self.len != other.len {
                return false;
            }
            unsafe {
                let mut i = 0usize;
                while i < self.len {
                    if *self.ptr.add(i) != *other.ptr.add(i) {
                        return false;
                    }
                    i += 1;
                }
            }
            true
        }
    }
    impl<T: PartialOrd> Vector<T> {
        /// `std::vector::operator<` — lexicographical comparison using the
        /// ELEMENT's `<` (`std::lexicographical_compare(a.begin(), a.end(),
        /// b.begin(), b.end())`). NOT Rust's derived `Vec` ordering: that stops
        /// at the first `partial_cmp == None` element, whereas C++ treats an
        /// unordered-OR-equal element pair (`!(x<y) && !(y<x)`) as "continue".
        /// This loop advances on that case, so a `vector<basic_json>` ordering
        /// (where some element pairs are unordered) stays byte-identical.
        #[inline]
        pub fn lex_less(&self, other: &Self) -> bool {
            unsafe {
                let n = if self.len < other.len { self.len } else { other.len };
                let mut i = 0usize;
                while i < n {
                    if *self.ptr.add(i) < *other.ptr.add(i) {
                        return true;
                    }
                    if *other.ptr.add(i) < *self.ptr.add(i) {
                        return false;
                    }
                    i += 1;
                }
                self.len < other.len
            }
        }
    }
    /// `std::string` (the `char` specialisation of `std::basic_string`): an
    /// owning, contiguous byte sequence over raw `libc` allocation, with
    /// geometric growth. Stores the logical length (excluding any terminator);
    /// element type is the platform `char`.
    pub struct String {
        ptr: *mut u8,
        len: usize,
        cap: usize,
    }
    impl String {
        #[inline]
        pub fn new() -> Self {
            String {
                ptr: ptr::null_mut(),
                len: 0,
                cap: 0,
            }
        }
        /// Grow-only capacity request — also `std::string::reserve(n)` (since
        /// C++20 a request below the capacity is a no-op, which libstdc++'s
        /// observable size/content behaviour always matched). Always keeps room
        /// for ONE extra byte beyond `want`: libstdc++ stores a NUL terminator
        /// just past the logical length, which lets the CONST `c_str()`/`data()`
        /// materialise the terminator (a heap-capacity write, not a struct
        /// mutation) without needing `&mut self` — so it is callable on a
        /// `const std::string&` (a `*const String`), exactly as in C++.
        #[inline]
        pub unsafe fn reserve(&mut self, want: usize) {
            let need = want + 1;
            if need <= self.cap {
                return;
            }
            let mut newcap = if self.cap == 0 { 1usize } else { self.cap * 2 };
            if newcap < need {
                newcap = need;
            }
            let np = libc::realloc(self.ptr as *mut libc::c_void, newcap) as *mut u8;
            assert!(! np.is_null(), "std::string allocation failed");
            self.ptr = np;
            self.cap = newcap;
        }
        /// `resize(n, ch)`: truncate to `n` bytes, or extend by appending
        /// copies of `ch` (`std::basic_string::resize`; the one-argument form
        /// fills with NUL bytes).
        #[inline]
        pub unsafe fn resize(&mut self, n: usize, fill: libc::c_char) {
            if n > self.len {
                self.reserve(n);
                libc::memset(
                    self.ptr.add(self.len) as *mut libc::c_void,
                    fill as libc::c_int,
                    n - self.len,
                );
            }
            self.len = n;
        }
        /// `std::string(count, ch)` — the fill constructor.
        #[inline]
        pub unsafe fn from_fill(n: usize, c: libc::c_char) -> Self {
            let mut r = String::new();
            if n != 0 {
                r.reserve(n);
                libc::memset(r.ptr as *mut libc::c_void, c as libc::c_int, n);
                r.len = n;
            }
            r
        }
        /// `find(ch, pos)` — the index of the first occurrence of byte `ch` at
        /// or after `pos`, or `npos` when absent (the `char` overload of
        /// `std::basic_string::find`; byte-wise over the logical bytes, so an
        /// interior NUL is findable — nlohmann's BSON key validation).
        #[inline]
        pub fn find_char(&self, c: libc::c_char, pos: libc::c_ulong) -> libc::c_ulong {
            let hay = self.as_byte_slice();
            let mut i = pos as usize;
            while i < hay.len() {
                if hay[i] == c as u8 {
                    return i as libc::c_ulong;
                }
                i += 1;
            }
            libc::c_ulong::MAX
        }
        /// Storage location of the last byte (`back()`).
        #[inline]
        pub unsafe fn back(&self) -> *mut libc::c_char {
            self.ptr.add(self.len - 1) as *mut libc::c_char
        }
        /// `std::string(const char*)`: copy the NUL-terminated C string.
        #[inline]
        pub unsafe fn from_cstr(s: *const libc::c_char) -> Self {
            let mut r = String::new();
            if !s.is_null() {
                let n = libc::strlen(s);
                r.reserve(n);
                ptr::copy_nonoverlapping(s as *const u8, r.ptr, n);
                r.len = n;
            }
            r
        }
        /// `std::to_string(integral)` — the signed overloads. libstdc++ formats
        /// via `snprintf`; a 64-bit integer needs at most 20 digits + sign +
        /// NUL, so a fixed 24-byte buffer never truncates.
        #[inline]
        pub unsafe fn from_i64(v: i64) -> Self {
            let mut buf = [0 as libc::c_char; 24];
            libc::snprintf(
                buf.as_mut_ptr(),
                buf.len(),
                b"%lld\0".as_ptr() as *const libc::c_char,
                v as libc::c_longlong,
            );
            String::from_cstr(buf.as_ptr())
        }
        /// `std::to_string(integral)` — the unsigned overloads.
        #[inline]
        pub unsafe fn from_u64(v: u64) -> Self {
            let mut buf = [0 as libc::c_char; 24];
            libc::snprintf(
                buf.as_mut_ptr(),
                buf.len(),
                b"%llu\0".as_ptr() as *const libc::c_char,
                v as libc::c_ulonglong,
            );
            String::from_cstr(buf.as_ptr())
        }
        /// `std::to_string(floating)` — libstdc++ uses `snprintf("%f", …)` (six
        /// fractional digits). The width is unbounded for large magnitudes, so
        /// size the buffer from `snprintf`'s reported length (two-pass).
        #[inline]
        pub unsafe fn from_f64(v: f64) -> Self {
            let fmt = b"%f\0".as_ptr() as *const libc::c_char;
            let n = libc::snprintf(ptr::null_mut(), 0, fmt, v);
            if n < 0 {
                return String::new();
            }
            let mut r = String::new();
            r.reserve(n as usize + 1);
            libc::snprintf(r.ptr as *mut libc::c_char, n as usize + 1, fmt, v);
            r.len = n as usize;
            r
        }
        #[inline]
        pub unsafe fn push_back(&mut self, c: libc::c_char) {
            self.reserve(self.len + 1);
            *self.ptr.add(self.len) = c as u8;
            self.len += 1;
        }
        /// `operator+=(const char*)` / `append(const char*)`.
        #[inline]
        pub unsafe fn append_cstr(&mut self, s: *const libc::c_char) {
            if s.is_null() {
                return;
            }
            let n = libc::strlen(s);
            if n != 0 {
                self.reserve(self.len + n);
                ptr::copy_nonoverlapping(s as *const u8, self.ptr.add(self.len), n);
                self.len += n;
            }
        }
        /// `append(const char* s, size_type count)` — append EXACTLY `count`
        /// bytes from `s`. The count is authoritative (NOT NUL-terminated): the
        /// JSON serializer's `write_characters` writes fixed-length spans of a
        /// REUSED scratch buffer, so a `strlen`-based copy would read stale tail
        /// bytes left by a previous, longer write.
        #[inline]
        pub unsafe fn append_bytes(&mut self, s: *const libc::c_char, n: usize) {
            if s.is_null() || n == 0 {
                return;
            }
            self.reserve(self.len + n);
            ptr::copy_nonoverlapping(s as *const u8, self.ptr.add(self.len), n);
            self.len += n;
        }
        /// `operator+=(const std::string&)` / `append(const std::string&)`:
        /// append the WHOLE byte sequence (NOT NUL-terminated; an interior NUL
        /// is part of the value and must be copied).
        #[inline]
        pub unsafe fn append_str(&mut self, other: &String) {
            let n = other.len;
            if n != 0 {
                self.reserve(self.len + n);
                ptr::copy_nonoverlapping(other.ptr, self.ptr.add(self.len), n);
                self.len += n;
            }
        }
        #[inline]
        pub fn size(&self) -> libc::c_ulong {
            self.len as libc::c_ulong
        }
        #[inline]
        pub fn empty(&self) -> bool {
            self.len == 0
        }
        /// Address of byte `i` — the UNCHECKED `operator[]` storage location.
        /// In C++ `operator[](pos)` is unchecked for `pos < size()` and
        /// `operator[](size())` legally addresses the NUL terminator (a
        /// spare capacity byte `reserve` always keeps), so no bounds check.
        #[inline]
        pub unsafe fn at(&self, i: usize) -> *mut libc::c_char {
            self.ptr.add(i) as *mut libc::c_char
        }
        /// `at(i)` — the BOUNDS-CHECKED byte accessor. `std::string::at`
        /// throws `std::out_of_range` when `i >= size()`; under `panic =
        /// abort` that is observably a process abort, so a faithful
        /// translation aborts here rather than forming an out-of-bounds
        /// pointer.
        #[inline]
        pub unsafe fn at_checked(&self, i: usize) -> *mut libc::c_char {
            if i >= self.len {
                panic!("std::string::at: index out of range");
            }
            self.ptr.add(i) as *mut libc::c_char
        }
        /// `c_str()` / `data()`: a NUL-terminated view of the bytes. CONST in
        /// C++ (`const char* c_str() const`), so it must be callable on a
        /// `const std::string&` — i.e. take `&self`. `reserve` always keeps a
        /// spare capacity byte past the length, so the terminator is written
        /// through the raw heap pointer (one capacity byte, NOT a field of
        /// `self`): the struct is unmodified, so the shared borrow is not
        /// violated. An empty (never-allocated) string returns a static `""`.
        #[inline]
        pub unsafe fn c_str(&self) -> *const libc::c_char {
            if self.ptr.is_null() {
                return b"\0".as_ptr() as *const libc::c_char;
            }
            *self.ptr.add(self.len) = 0;
            self.ptr as *const libc::c_char
        }
        /// `clear()`: logical length to zero (capacity retained).
        #[inline]
        pub fn clear(&mut self) {
            self.len = 0;
        }
        /// `std::string::compare` — lexicographic byte comparison treating each
        /// byte as `unsigned char` (the standard's `char_traits<char>::compare`
        /// /`lt` use `unsigned char`), returning a negative / zero / positive
        /// `int`. The free relational operators (`a < b`, `a == b`, …) are this
        /// `compare(...) <relop> 0`. Interior NUL bytes are part of the value.
        #[inline]
        pub unsafe fn compare(&self, other: &String) -> libc::c_int {
            let a = if self.ptr.is_null() {
                &[][..]
            } else {
                core::slice::from_raw_parts(self.ptr, self.len)
            };
            let b = if other.ptr.is_null() {
                &[][..]
            } else {
                core::slice::from_raw_parts(other.ptr, other.len)
            };
            match a.cmp(b) {
                core::cmp::Ordering::Less => -1,
                core::cmp::Ordering::Equal => 0,
                core::cmp::Ordering::Greater => 1,
            }
        }
        /// `std::string::compare` against a NUL-terminated C string (the
        /// `const char*` relational overloads): same lexicographic byte order.
        #[inline]
        pub unsafe fn compare_cstr(&self, other: *const libc::c_char) -> libc::c_int {
            let a = if self.ptr.is_null() {
                &[][..]
            } else {
                core::slice::from_raw_parts(self.ptr, self.len)
            };
            let n = if other.is_null() { 0 } else { libc::strlen(other) };
            let b = if other.is_null() {
                &[][..]
            } else {
                core::slice::from_raw_parts(other as *const u8, n)
            };
            match a.cmp(b) {
                core::cmp::Ordering::Less => -1,
                core::cmp::Ordering::Equal => 0,
                core::cmp::Ordering::Greater => 1,
            }
        }
        /// `operator+`: a fresh `std::string` that is the concatenation of two
        /// byte sequences. The left operand seeds a clone the right is appended
        /// to (matching the standard's `basic_string operator+`). The variants
        /// cover the `std::string` / `const char*` / `char` operand forms.
        #[inline]
        pub unsafe fn concat(a: &String, b: &String) -> String {
            let mut r = a.clone();
            r.append_str(b);
            r
        }
        #[inline]
        pub unsafe fn concat_cstr_rhs(a: &String, b: *const libc::c_char) -> String {
            let mut r = a.clone();
            r.append_cstr(b);
            r
        }
        #[inline]
        pub unsafe fn concat_cstr_lhs(a: *const libc::c_char, b: &String) -> String {
            let mut r = String::from_cstr(a);
            r.append_str(b);
            r
        }
        #[inline]
        pub unsafe fn concat_char_rhs(a: &String, c: libc::c_char) -> String {
            let mut r = a.clone();
            r.push_back(c);
            r
        }
        #[inline]
        pub unsafe fn concat_char_lhs(c: libc::c_char, b: &String) -> String {
            let mut r = String::new();
            r.push_back(c);
            r.append_str(b);
            r
        }
        /// Pointer to the first byte (`begin()`), for iteration.
        #[inline]
        pub unsafe fn begin_ptr(&self) -> *mut libc::c_char {
            self.ptr as *mut libc::c_char
        }
        /// Pointer past the last byte (`end()`).
        #[inline]
        pub unsafe fn end_ptr(&self) -> *mut libc::c_char {
            self.ptr.add(self.len) as *mut libc::c_char
        }
        /// The logical bytes as a slice (empty when the buffer is unallocated;
        /// `from_raw_parts` requires a non-null pointer even for length 0).
        #[inline]
        fn as_byte_slice(&self) -> &[u8] {
            if self.len == 0 {
                &[]
            } else {
                unsafe { core::slice::from_raw_parts(self.ptr, self.len) }
            }
        }
        /// `find(str, pos)` — the index of the first occurrence of `str` at or
        /// after byte `pos`, or `npos` (`size_type::MAX`) if absent. Byte-wise
        /// over the logical bytes (an interior NUL is part of the value),
        /// matching `std::basic_string::find`: an empty needle matches at `pos`
        /// (when `pos <= size`), and `pos > size` never matches. The result type
        /// is `size_type` (= `std::size_t` = `unsigned long` on the target).
        #[inline]
        pub fn find_str(&self, needle: &String, pos: libc::c_ulong) -> libc::c_ulong {
            let hay = self.as_byte_slice();
            let need = needle.as_byte_slice();
            let pos = pos as usize;
            if pos > hay.len() {
                return libc::c_ulong::MAX;
            }
            if need.is_empty() {
                return pos as libc::c_ulong;
            }
            if need.len() > hay.len() {
                return libc::c_ulong::MAX;
            }
            let last = hay.len() - need.len();
            let mut i = pos;
            while i <= last {
                if &hay[i..i + need.len()] == need {
                    return i as libc::c_ulong;
                }
                i += 1;
            }
            libc::c_ulong::MAX
        }
        /// `replace(pos, count, str)` — replace the `count` bytes starting at
        /// `pos` with the bytes of `str`, building the new value
        /// prefix + `str` + suffix (`std::basic_string::replace`). `count` is
        /// clamped to the bytes available after `pos`.
        #[inline]
        pub unsafe fn replace_str(
            &mut self,
            pos: libc::size_t,
            count: libc::size_t,
            other: &String,
        ) {
            let len = self.len;
            let pos = (pos as usize).min(len);
            let count = (count as usize).min(len - pos);
            let suffix_len = len - pos - count;
            let other_len = other.len;
            let mut result = String::new();
            result.reserve(pos + other_len + suffix_len);
            if pos != 0 {
                ptr::copy_nonoverlapping(self.ptr, result.ptr, pos);
            }
            if other_len != 0 {
                ptr::copy_nonoverlapping(other.ptr, result.ptr.add(pos), other_len);
            }
            if suffix_len != 0 {
                ptr::copy_nonoverlapping(
                    self.ptr.add(pos + count),
                    result.ptr.add(pos + other_len),
                    suffix_len,
                );
            }
            result.len = pos + other_len + suffix_len;
            *self = result;
        }
        /// `substr(pos, count)` — a fresh owning string holding the `count`
        /// bytes starting at byte `pos`, with `count` clamped to the bytes
        /// available after `pos` (`std::basic_string::substr`; the default
        /// `count` of `npos` therefore means "to the end"). Memory-safe in the
        /// `pos > size()` corner (which `std::substr` reports as
        /// `std::out_of_range`) by clamping `pos` to the length — the same
        /// clamping convention `replace_str` uses — yielding an empty result.
        #[inline]
        pub unsafe fn substr(&self, pos: libc::size_t, count: libc::size_t) -> String {
            let len = self.len;
            let pos = (pos as usize).min(len);
            let count = (count as usize).min(len - pos);
            let mut r = String::new();
            if count != 0 {
                r.reserve(count);
                ptr::copy_nonoverlapping(self.ptr.add(pos), r.ptr, count);
                r.len = count;
            }
            r
        }
        /// `find_first_of(c, pos)` for the single-CHARACTER overload — the index
        /// of the first byte equal to `c` at or after byte `pos`, or `npos`
        /// (`size_type::MAX`) if none. For a single character `find_first_of`
        /// coincides with `find`; `pos >= size` never matches. The result type
        /// is `size_type` (= `std::size_t` = `unsigned long` on the target).
        #[inline]
        pub fn find_first_of_char(
            &self,
            c: libc::c_char,
            pos: libc::c_ulong,
        ) -> libc::c_ulong {
            let hay = self.as_byte_slice();
            let needle = c as u8;
            let mut i = pos as usize;
            while i < hay.len() {
                if hay[i] == needle {
                    return i as libc::c_ulong;
                }
                i += 1;
            }
            libc::c_ulong::MAX
        }
    }
    impl Drop for String {
        fn drop(&mut self) {
            unsafe {
                if !self.ptr.is_null() {
                    libc::free(self.ptr as *mut libc::c_void);
                }
            }
        }
    }
    /// `std::wstring` (the `wchar_t` specialisation of `std::basic_string`):
    /// the wide sibling of [`String`] — an owning, contiguous sequence of
    /// 32-bit `wchar_t` codepoints (this target's `wchar_t` is 4 bytes) over
    /// raw `libc` allocation with geometric growth. Same memory discipline as
    /// `String`: the logical length excludes the terminator, and `reserve`
    /// always keeps one spare ELEMENT past the length so the const
    /// `c_str()`/`data()` can materialise the L'\0' terminator through the
    /// heap pointer without `&mut self`.
    pub struct WString {
        ptr: *mut u32,
        len: usize,
        cap: usize,
    }
    impl WString {
        #[inline]
        pub fn new() -> Self {
            WString {
                ptr: ptr::null_mut(),
                len: 0,
                cap: 0,
            }
        }
        /// Grow-only capacity request, in ELEMENTS (see `String::reserve`).
        #[inline]
        pub unsafe fn reserve(&mut self, want: usize) {
            let need = want + 1;
            if need <= self.cap {
                return;
            }
            let mut newcap = if self.cap == 0 { 1usize } else { self.cap * 2 };
            if newcap < need {
                newcap = need;
            }
            let np = libc::realloc(self.ptr as *mut libc::c_void, newcap * 4)
                as *mut u32;
            assert!(! np.is_null(), "std::wstring allocation failed");
            self.ptr = np;
            self.cap = newcap;
        }
        /// `resize(n, ch)`: truncate to `n` elements, or extend by appending
        /// copies of `ch` (the one-argument form fills with L'\0').
        #[inline]
        pub unsafe fn resize(&mut self, n: usize, fill: libc::wchar_t) {
            if n > self.len {
                self.reserve(n);
                let mut i = self.len;
                while i < n {
                    *self.ptr.add(i) = fill as u32;
                    i += 1;
                }
            }
            self.len = n;
        }
        #[inline]
        pub fn size(&self) -> libc::c_ulong {
            self.len as libc::c_ulong
        }
        #[inline]
        pub fn empty(&self) -> bool {
            self.len == 0
        }
        /// `clear()`: logical length to zero (capacity retained).
        #[inline]
        pub fn clear(&mut self) {
            self.len = 0;
        }
        /// `operator[]` / unchecked `at`: pointer to the `i`-th element —
        /// C++ additionally allows `&s[size()]` (the terminator's address),
        /// which stays in the reserved spare element.
        #[inline]
        pub unsafe fn at(&self, i: libc::c_ulong) -> *mut libc::wchar_t {
            self.ptr.add(i as usize) as *mut libc::wchar_t
        }
        /// `push_back(ch)`.
        #[inline]
        pub unsafe fn push_back(&mut self, ch: libc::wchar_t) {
            self.reserve(self.len + 1);
            *self.ptr.add(self.len) = ch as u32;
            self.len += 1;
        }
        /// `c_str()` / `data()`: NUL-terminated wide pointer; the terminator
        /// write goes through the reserved spare element (see `String::c_str`
        /// for why this is callable on a `const` receiver). An empty
        /// (never-allocated) string returns a static L"".
        #[inline]
        pub unsafe fn c_str(&self) -> *const libc::wchar_t {
            if self.ptr.is_null() {
                static EMPTY: u32 = 0;
                return &EMPTY as *const u32 as *const libc::wchar_t;
            }
            *self.ptr.add(self.len) = 0;
            self.ptr as *const libc::wchar_t
        }
        /// The wide analogue of `String::from_cstr`: copy a NUL-terminated
        /// `wchar_t` buffer (a `std::wstring(const wchar_t*)` construction).
        pub unsafe fn from_wcstr(s: *const libc::wchar_t) -> Self {
            let mut r = WString::new();
            if s.is_null() {
                return r;
            }
            let mut n = 0usize;
            while *s.add(n) != 0 {
                n += 1;
            }
            if n != 0 {
                r.reserve(n);
                ::core::ptr::copy_nonoverlapping(s as *const u32, r.ptr, n);
                r.len = n;
            }
            r
        }
    }
    impl Drop for WString {
        fn drop(&mut self) {
            unsafe {
                if !self.ptr.is_null() {
                    libc::free(self.ptr as *mut libc::c_void);
                }
            }
        }
    }
    /// `std::wstring` value-copy (the copy constructor): a fresh owning
    /// buffer with the same codepoints.
    impl Clone for WString {
        fn clone(&self) -> Self {
            let mut r = WString::new();
            let n = self.len;
            if n != 0 {
                unsafe {
                    r.reserve(n);
                    ::core::ptr::copy_nonoverlapping(self.ptr, r.ptr, n);
                }
                r.len = n;
            }
            r
        }
    }
    /// `std::string` value-copy (the copy constructor): a fresh owning buffer
    /// with the same bytes. Needed to insert a key into the ordered `Map`.
    impl Clone for String {
        fn clone(&self) -> Self {
            let mut r = String::new();
            let n = self.len;
            if n != 0 {
                unsafe {
                    r.reserve(n);
                    ptr::copy_nonoverlapping(self.ptr, r.ptr, n);
                }
                r.len = n;
            }
            r
        }
    }
    /// `std::string`'s comparisons: `char_traits<char>::compare` is a byte-wise
    /// (unsigned) lexicographic comparison — exactly slice `Ord`/`Eq` over the
    /// bytes — which is what `std::map<std::string, …>` orders its keys by.
    impl PartialEq for String {
        fn eq(&self, other: &Self) -> bool {
            self.as_byte_slice() == other.as_byte_slice()
        }
    }
    impl Eq for String {}
    impl PartialOrd for String {
        fn partial_cmp(&self, other: &Self) -> Option<core::cmp::Ordering> {
            Some(self.cmp(other))
        }
    }
    impl Ord for String {
        fn cmp(&self, other: &Self) -> core::cmp::Ordering {
            self.as_byte_slice().cmp(other.as_byte_slice())
        }
    }
    /// The append operation of a container that `std::back_insert_iterator`
    /// drives — `*it = v` calls `container->push_back(v)`. `Elem` is the
    /// container's `value_type`. Implemented for the support `String`
    /// (`char`-element) and `Vector<T>` (`T`-element).
    pub trait BackInsertable {
        type Elem;
        unsafe fn back_insert(&mut self, value: Self::Elem);
    }
    impl BackInsertable for String {
        type Elem = libc::c_char;
        #[inline]
        unsafe fn back_insert(&mut self, value: libc::c_char) {
            self.push_back(value);
        }
    }
    impl<T> BackInsertable for Vector<T> {
        type Elem = T;
        #[inline]
        unsafe fn back_insert(&mut self, value: T) {
            self.push_back(value);
        }
    }
    /// `std::back_insert_iterator<Container>` — an output iterator that appends
    /// each assigned value to the back of the bound container via its
    /// `push_back`. `operator*` and `operator++` are identity (the iterator is
    /// its own assignable proxy); `*it = v` / `*it++ = v` append `v`. Holds the
    /// container by raw pointer (the iterator is non-owning, like the C++
    /// `container_type*` member), so it is trivially copyable regardless of the
    /// container type (the manual `Copy`/`Clone` avoid the derive's `C: Copy`
    /// bound).
    pub struct BackInsertIterator<C> {
        container: *mut C,
    }
    impl<C> Copy for BackInsertIterator<C> {}
    impl<C> Clone for BackInsertIterator<C> {
        #[inline]
        fn clone(&self) -> Self {
            *self
        }
    }
    impl<C: BackInsertable> BackInsertIterator<C> {
        /// `std::back_inserter(c)` — bind the iterator to container `c`.
        #[inline]
        pub unsafe fn new(container: *mut C) -> Self {
            BackInsertIterator { container }
        }
        /// `*it = value` — append `value` to the bound container. Takes `&self`
        /// (the append mutates the CONTAINER through the raw pointer, not the
        /// iterator handle), so a `Copy` iterator threaded through the
        /// `*it++ = v` operator chain is not also mutably borrowed.
        #[inline]
        pub unsafe fn push(&self, value: <C as BackInsertable>::Elem) {
            (*self.container).back_insert(value);
        }
    }
    /// `std::reverse_iterator<Iter>` (and nlohmann's behaviour-identical subclass
    /// `json_reverse_iterator<Iter>`): a bidirectional iterator adaptor that
    /// traverses a range backwards. It holds the underlying `current` iterator
    /// and dereferences the element BEFORE it (`*--tmp`), so advancing the
    /// adaptor (`++`) DECREMENTS `current`. This is a thin holder: every element
    /// operation is lowered IN LINE against the concrete `current` iterator's own
    /// `operator--`/`operator*`/`operator->`/`operator!=` (see
    /// `lower_stdlib_method_call` / `lower_stdlib_free_call` — those are the only
    /// place the adaptor's semantics live, since the base iterator is a translated
    /// type with per-instantiation method names). The `current` field is `pub` for
    /// that in-line access from the call site's module.
    #[derive(Copy, Clone)]
    pub struct ReverseIterator<I> {
        pub current: I,
    }
    impl<I> ReverseIterator<I> {
        /// `reverse_iterator(it)` — adapt the base iterator `it`.
        #[inline]
        pub unsafe fn new(current: I) -> Self {
            ReverseIterator { current }
        }
    }
    /// `std::unique_ptr<T>`: a sole-ownership owning pointer. Holds a raw
    /// pointer to a single heap-allocated `T`; destruction runs `T`'s
    /// destructor and frees the storage. Moves transfer ownership (Rust moves
    /// it bitwise; the moved-from value is not dropped).
    pub struct UniquePtr<T> {
        ptr: *mut T,
    }
    impl<T> UniquePtr<T> {
        #[inline]
        pub fn null() -> Self {
            UniquePtr { ptr: ptr::null_mut() }
        }
        /// `std::make_unique<T>(args…)`: allocate one `T` and construct it.
        #[inline]
        pub unsafe fn make(value: T) -> Self {
            let bytes = core::mem::size_of::<T>().max(1);
            let p = libc::malloc(bytes) as *mut T;
            assert!(! p.is_null(), "std::make_unique allocation failed");
            ptr::write(p, value);
            UniquePtr { ptr: p }
        }
        /// `std::make_unique<T[]>(n)` — the ARRAY overload: allocate `n`
        /// value-initialised (zeroed, the value-init of a trivial `T`) elements
        /// and own them. The array form `std::unique_ptr<T[]>` has pointer type
        /// `T*` (the element pointer), so it is represented as `UniquePtr<T>`
        /// whose stored pointer is the array base; the generic `Drop` frees the
        /// whole `malloc` block, and the lowering routes only a trivially-
        /// destructible `T` here, so no per-element destruction is owed.
        #[inline]
        pub unsafe fn make_array(n: usize) -> Self {
            let bytes = core::mem::size_of::<T>().saturating_mul(n).max(1);
            let p = libc::malloc(bytes) as *mut T;
            assert!(! p.is_null(), "std::make_unique<T[]> allocation failed");
            ptr::write_bytes(p, 0, n);
            UniquePtr { ptr: p }
        }
        /// `get()` / the storage location for `operator*` / `operator->`.
        #[inline]
        pub fn get(&self) -> *mut T {
            self.ptr
        }
        /// Adopt an already-allocated raw pointer: `std::unique_ptr<T>(p)`, or
        /// the `unique_ptr<T, Deleter>(p, deleter)` form that nlohmann's
        /// `create<T>` uses to guard freshly-allocated (not-yet-constructed)
        /// storage. Ownership transfers in; `Drop` frees it unless `release`
        /// relinquishes ownership first.
        #[inline]
        pub unsafe fn from_raw(p: *mut T) -> Self {
            UniquePtr { ptr: p }
        }
        /// `release()`: relinquish ownership and return the owned raw pointer.
        /// The `UniquePtr` is emptied so its `Drop` no longer frees the object
        /// — the caller now owns it (`std::unique_ptr::release`).
        #[inline]
        pub fn release(&mut self) -> *mut T {
            let p = self.ptr;
            self.ptr = ptr::null_mut();
            p
        }
        /// `reset(p)`: take ownership of `p`, then delete the PREVIOUSLY-managed
        /// object (`std::unique_ptr::reset` — the new pointer is stored first, so
        /// a destructor that re-enters sees the updated state). `reset()` (a null
        /// `p`) just deletes the current object.
        #[inline]
        pub unsafe fn reset(&mut self, p: *mut T) {
            let old = self.ptr;
            self.ptr = p;
            if !old.is_null() {
                ptr::drop_in_place(old);
                if core::mem::size_of::<T>() != 0 {
                    libc::free(old as *mut libc::c_void);
                }
            }
        }
        /// `operator=(unique_ptr&&)`: MOVE assignment. Take the source's pointer
        /// (emptying the source so it stays a VALID, null moved-from object — as
        /// C++ leaves it, usable afterwards), then delete the previously-owned
        /// object — exactly `std::unique_ptr::operator=`'s `reset(src.release())`,
        /// including its store-new-before-delete-old order. The source is taken by
        /// `&mut` (not by value) so a NAMED moved-from `unique_ptr` is not
        /// consumed (a temporary source is materialised and dropped null).
        #[inline]
        pub unsafe fn move_assign(&mut self, src: &mut Self) {
            let p = src.release();
            self.reset(p);
        }
    }
    impl<T> Drop for UniquePtr<T> {
        fn drop(&mut self) {
            unsafe {
                if !self.ptr.is_null() {
                    ptr::drop_in_place(self.ptr);
                    if core::mem::size_of::<T>() != 0 {
                        libc::free(self.ptr as *mut libc::c_void);
                    }
                }
            }
        }
    }
    /// Control block shared by all `SharedPtr` owners of one object: the
    /// ATOMIC strong reference count plus the TYPE-ERASED destroy/free
    /// recipe for the managed object — exactly `std::shared_ptr`'s design,
    /// which lets a `shared_ptr<Base>` converted from a `shared_ptr<Derived>`
    /// still run the DERIVED destructor when the count reaches zero. The
    /// count is a `core::sync::atomic::AtomicUsize` driven with the canonical
    /// refcount orderings (`Relaxed` increment, `Release` decrement, an
    /// `Acquire` fence before the final destroy) — identical to
    /// `std::shared_ptr`'s atomic count and to Rust's own `Arc`, so the
    /// pointer stays correct when shared across threads instead of racing on
    /// a plain integer. Single-threaded programs observe the exact same count
    /// transitions, so translated output is unchanged for them.
    pub struct __SharedCtl {
        count: core::sync::atomic::AtomicUsize,
        /// Drops the managed value in place and frees its storage. Bound to
        /// the ORIGINAL (most-derived) `T` at `make` time.
        destroy: unsafe fn(*mut libc::c_void),
        /// The managed object's allocation (what `destroy` consumes).
        storage: *mut libc::c_void,
    }
    unsafe fn __shared_destroy<T>(p: *mut libc::c_void) {
        let p = p as *mut T;
        ptr::drop_in_place(p);
        libc::free(p as *mut libc::c_void);
    }
    /// `std::shared_ptr<T>`: shared-ownership reference-counted pointer — a
    /// control-block pointer plus the object pointer, mirroring
    /// `std::shared_ptr`'s two-pointer layout. Copy construction increments
    /// the count (`Clone`); destruction decrements it and, at zero, runs the
    /// control block's type-erased destroy (the original most-derived
    /// destructor) — so the derived-to-base conversion is sound. The strong
    /// count in the control block is ATOMIC (see `__SharedCtl`), so copies
    /// shared across threads are reference-counted correctly.
    pub struct SharedPtr<T> {
        ctl: *mut __SharedCtl,
        obj: *mut T,
    }
    impl<T> SharedPtr<T> {
        #[inline]
        pub fn null() -> Self {
            SharedPtr {
                ctl: ptr::null_mut(),
                obj: ptr::null_mut(),
            }
        }
        /// `std::make_shared<T>(args…)`: allocate and adopt one constructed `T`.
        #[inline]
        pub unsafe fn make(value: T) -> Self {
            let obj = libc::malloc(core::mem::size_of::<T>().max(1)) as *mut T;
            assert!(! obj.is_null(), "std::make_shared allocation failed");
            ptr::write(obj, value);
            let ctl = libc::malloc(core::mem::size_of::<__SharedCtl>())
                as *mut __SharedCtl;
            assert!(! ctl.is_null(), "std::make_shared allocation failed");
            ptr::write(
                ctl,
                __SharedCtl {
                    count: core::sync::atomic::AtomicUsize::new(1),
                    destroy: __shared_destroy::<T>,
                    storage: obj as *mut libc::c_void,
                },
            );
            SharedPtr { ctl, obj }
        }
        /// The implicit `shared_ptr<Derived>` -> `shared_ptr<Base>` pointer
        /// conversion: share the SAME control block (so the most-derived
        /// destructor still runs at count zero) and retarget the object
        /// pointer. The dispatch gates this on a base subobject at byte
        /// offset 0 — the single-inheritance layout the vtable support
        /// accepts — so the raw pointer cast IS the subobject address.
        #[inline]
        pub unsafe fn cast_base<B>(self) -> SharedPtr<B> {
            let r = SharedPtr {
                ctl: self.ctl,
                obj: self.obj as *mut B,
            };
            core::mem::forget(self);
            r
        }
        /// `get()` / the storage location for `operator*` / `operator->`.
        #[inline]
        pub fn get(&self) -> *mut T {
            self.obj
        }
        #[inline]
        pub fn use_count(&self) -> libc::c_long {
            if self.ctl.is_null() {
                0
            } else {
                unsafe {
                    (*self.ctl).count.load(core::sync::atomic::Ordering::Relaxed)
                        as libc::c_long
                }
            }
        }
    }
    impl<T> Clone for SharedPtr<T> {
        #[inline]
        fn clone(&self) -> Self {
            if !self.ctl.is_null() {
                unsafe {
                    (*self.ctl)
                        .count
                        .fetch_add(1, core::sync::atomic::Ordering::Relaxed);
                }
            }
            SharedPtr {
                ctl: self.ctl,
                obj: self.obj,
            }
        }
    }
    impl<T> Drop for SharedPtr<T> {
        fn drop(&mut self) {
            unsafe {
                if !self.ctl.is_null() {
                    if (*self.ctl)
                        .count
                        .fetch_sub(1, core::sync::atomic::Ordering::Release) == 1
                    {
                        core::sync::atomic::fence(core::sync::atomic::Ordering::Acquire);
                        ((*self.ctl).destroy)((*self.ctl).storage);
                        libc::free(self.ctl as *mut libc::c_void);
                    }
                }
            }
        }
    }
    /// `std::type_info`: the RTTI type descriptor. Models the surface used in
    /// practice — `name()` returns the Itanium-mangled type name (the exact bytes
    /// libstdc++'s `type_info::name()` returns). Exactly one `TypeInfo` static
    /// exists per type, so `operator==` is pointer identity over `&TypeInfo`
    /// (libstdc++'s single-`type_info`-per-type model).
    #[repr(C)]
    pub struct TypeInfo {
        pub __name: *const libc::c_char,
    }
    unsafe impl Sync for TypeInfo {}
    impl TypeInfo {
        #[inline]
        pub fn name(&self) -> *const libc::c_char {
            self.__name
        }
    }
    /// `std::pair<A, B>`: a two-field aggregate. The members are named `first` /
    /// `second`, matching the C++ field access exactly. `Clone`/`Copy` are
    /// derived conditionally on the members (as in C++). `PartialEq` /
    /// `PartialOrd` are derived too: the derived field-wise `==` and the derived
    /// lexicographical `partial_cmp` (compare `first`, then `second`) match
    /// `std::pair`'s `operator==` / `operator<` EXACTLY, so a `std::map`
    /// (ordered `Vector<Pair>`) compares element-wise via these.
    #[derive(Clone, Copy, PartialEq, PartialOrd)]
    pub struct Pair<A, B> {
        pub first: A,
        pub second: B,
    }
    /// `std::atomic<T>`: a single value with genuinely ATOMIC loads, stores
    /// and read-modify-write operations, so copies of the object pointer
    /// shared across threads observe `std::atomic`'s guarantees instead of
    /// racing on a plain cell.
    ///
    /// * For a `T` whose size is 1/2/4/8 bytes and whose alignment covers
    ///   that size — every integer, enum, bool, pointer and float the
    ///   corpus instantiates — operations are performed directly on the
    ///   storage through the matching `core::sync::atomic::AtomicU{8,16,32,64}`
    ///   (whose layout the standard library guarantees identical to the
    ///   underlying integer). RMW ops use a compare-exchange loop on the bit
    ///   representation, exactly how C++ implements `fetch_add` for
    ///   non-integral atomics.
    /// * Any other `T` falls back to an address-hashed global spinlock —
    ///   the same strategy libatomic uses for `is_lock_free() == false`
    ///   types, preserving atomicity (not just tearing-freedom).
    ///
    /// Every operation uses sequentially-consistent ordering: `seq_cst` is
    /// the C++ DEFAULT for all of these member functions, and for a call
    /// site that passed a weaker explicit `std::memory_order` it is a sound
    /// strengthening (every behaviour permitted under `seq_cst` is permitted
    /// under the weaker order, never the reverse).
    ///
    /// `fetch_add`/`fetch_sub` apply `T`'s own `+`/`-` inside the CAS loop,
    /// preserving the lowered arithmetic semantics for the stored type.
    pub struct Atomic<T> {
        value: core::cell::UnsafeCell<T>,
    }
    /// SAFETY: all access to the cell is through the atomic/lock-protected
    /// operations below, which serialise concurrent use — the whole point of
    /// `std::atomic<T>`.
    unsafe impl<T: Copy + Send> Sync for Atomic<T> {}
    /// Address-hashed spinlocks backing `Atomic<T>` for a `T` with no
    /// native-width representation (libatomic's lock-table strategy).
    static __ATOMIC_FALLBACK_LOCKS: [core::sync::atomic::AtomicBool; 16] = [const {
        core::sync::atomic::AtomicBool::new(false)
    }; 16];
    struct __AtomicFallbackGuard(&'static core::sync::atomic::AtomicBool);
    fn __atomic_fallback_lock(addr: usize) -> __AtomicFallbackGuard {
        use core::sync::atomic::Ordering;
        let lock = &__ATOMIC_FALLBACK_LOCKS[(addr >> 3) & 0xF];
        while lock
            .compare_exchange_weak(false, true, Ordering::Acquire, Ordering::Relaxed)
            .is_err()
        {
            core::hint::spin_loop();
        }
        __AtomicFallbackGuard(lock)
    }
    impl Drop for __AtomicFallbackGuard {
        fn drop(&mut self) {
            self.0.store(false, core::sync::atomic::Ordering::Release);
        }
    }
    impl<T: Copy> Atomic<T> {
        #[inline]
        pub const fn new(value: T) -> Self {
            Atomic {
                value: core::cell::UnsafeCell::new(value),
            }
        }
        /// Whether `T`'s storage can be operated on through a native atomic
        /// integer of the same size (size 1/2/4/8, alignment covering it).
        #[inline]
        fn native_width() -> Option<usize> {
            let (s, a) = (core::mem::size_of::<T>(), core::mem::align_of::<T>());
            (matches!(s, 1 | 2 | 4 | 8) && a >= s).then_some(s)
        }
        #[inline]
        pub fn load(&self) -> T {
            use core::sync::atomic::*;
            let p = self.value.get();
            unsafe {
                match Self::native_width() {
                    Some(1) => {
                        let v = (*(p as *const AtomicU8)).load(Ordering::SeqCst);
                        core::mem::transmute_copy(&v)
                    }
                    Some(2) => {
                        let v = (*(p as *const AtomicU16)).load(Ordering::SeqCst);
                        core::mem::transmute_copy(&v)
                    }
                    Some(4) => {
                        let v = (*(p as *const AtomicU32)).load(Ordering::SeqCst);
                        core::mem::transmute_copy(&v)
                    }
                    Some(8) => {
                        let v = (*(p as *const AtomicU64)).load(Ordering::SeqCst);
                        core::mem::transmute_copy(&v)
                    }
                    _ => {
                        let _g = __atomic_fallback_lock(p as usize);
                        *p
                    }
                }
            }
        }
        #[inline]
        pub fn store(&self, v: T) {
            use core::sync::atomic::*;
            let p = self.value.get();
            unsafe {
                match Self::native_width() {
                    Some(1) => {
                        (*(p as *const AtomicU8))
                            .store(core::mem::transmute_copy(&v), Ordering::SeqCst)
                    }
                    Some(2) => {
                        (*(p as *const AtomicU16))
                            .store(core::mem::transmute_copy(&v), Ordering::SeqCst)
                    }
                    Some(4) => {
                        (*(p as *const AtomicU32))
                            .store(core::mem::transmute_copy(&v), Ordering::SeqCst)
                    }
                    Some(8) => {
                        (*(p as *const AtomicU64))
                            .store(core::mem::transmute_copy(&v), Ordering::SeqCst)
                    }
                    _ => {
                        let _g = __atomic_fallback_lock(p as usize);
                        *p = v;
                    }
                }
            }
        }
        /// One CAS on the bit representation: `true` iff the stored bits
        /// equalled `current`'s and were replaced by `new`'s.
        #[inline]
        fn compare_exchange_rep(&self, current: T, new: T) -> bool {
            use core::sync::atomic::*;
            let p = self.value.get();
            unsafe {
                match Self::native_width() {
                    Some(1) => {
                        (*(p as *const AtomicU8))
                            .compare_exchange(
                                core::mem::transmute_copy(&current),
                                core::mem::transmute_copy(&new),
                                Ordering::SeqCst,
                                Ordering::SeqCst,
                            )
                            .is_ok()
                    }
                    Some(2) => {
                        (*(p as *const AtomicU16))
                            .compare_exchange(
                                core::mem::transmute_copy(&current),
                                core::mem::transmute_copy(&new),
                                Ordering::SeqCst,
                                Ordering::SeqCst,
                            )
                            .is_ok()
                    }
                    Some(4) => {
                        (*(p as *const AtomicU32))
                            .compare_exchange(
                                core::mem::transmute_copy(&current),
                                core::mem::transmute_copy(&new),
                                Ordering::SeqCst,
                                Ordering::SeqCst,
                            )
                            .is_ok()
                    }
                    Some(8) => {
                        (*(p as *const AtomicU64))
                            .compare_exchange(
                                core::mem::transmute_copy(&current),
                                core::mem::transmute_copy(&new),
                                Ordering::SeqCst,
                                Ordering::SeqCst,
                            )
                            .is_ok()
                    }
                    _ => {
                        let _g = __atomic_fallback_lock(p as usize);
                        let same = libc::memcmp(
                            p as *const libc::c_void,
                            core::ptr::addr_of!(current) as *const libc::c_void,
                            core::mem::size_of::<T>(),
                        ) == 0;
                        if same {
                            *p = new;
                        }
                        same
                    }
                }
            }
        }
        #[inline]
        pub fn exchange(&self, v: T) -> T {
            loop {
                let old = self.load();
                if self.compare_exchange_rep(old, v) {
                    return old;
                }
            }
        }
    }
    impl<T: Copy + core::ops::Add<Output = T> + core::ops::Sub<Output = T>> Atomic<T> {
        /// `fetch_add` returns the PREVIOUS value (C++ semantics).
        #[inline]
        pub fn fetch_add(&self, v: T) -> T {
            loop {
                let old = self.load();
                if self.compare_exchange_rep(old, old + v) {
                    return old;
                }
            }
        }
        #[inline]
        pub fn fetch_sub(&self, v: T) -> T {
            loop {
                let old = self.load();
                if self.compare_exchange_rep(old, old - v) {
                    return old;
                }
            }
        }
        /// `operator+=` returns the NEW value (C++ atomic compound assignment).
        #[inline]
        pub fn add_assign(&self, v: T) -> T {
            self.fetch_add(v) + v
        }
        #[inline]
        pub fn sub_assign(&self, v: T) -> T {
            self.fetch_sub(v) - v
        }
    }
    /// `std::list<T>`: a sequence with front/back insertion. libstdc++ backs it
    /// with a doubly-linked node chain; the support type backs it with the
    /// contiguous `Vector<T>` instead. For the push-then-iterate usage the
    /// corpus exercises (ACE_Parse_Node builds a list with `push_front`, then
    /// walks it front-to-back) the OBSERVABLE behaviour — iteration ORDER,
    /// element VALUES, and size — is identical, and a contiguous backing lets a
    /// list iterator be a plain element pointer that reuses every pointer-
    /// iterator operation (`*it`, `++it`, `it != end`). Two representation
    /// boundaries versus libstdc++: iterator/reference STABILITY across
    /// insertions is not modelled (no corpus use interleaves insertion with a
    /// held iterator), and `push_front` is O(n) (a front insert shifts) rather
    /// than O(1) — neither is observable to the corpus.
    pub struct List<T> {
        items: Vector<T>,
    }
    impl<T> List<T> {
        #[inline]
        pub fn new() -> Self {
            List { items: Vector::new() }
        }
        #[inline]
        pub fn size(&self) -> libc::c_ulong {
            self.items.size()
        }
        #[inline]
        pub fn empty(&self) -> bool {
            self.items.empty()
        }
        /// `begin()` — pointer to the first element.
        #[inline]
        pub unsafe fn begin_ptr(&self) -> *mut T {
            self.items.begin_ptr()
        }
        /// `end()` — one past the last element.
        #[inline]
        pub unsafe fn end_ptr(&self) -> *mut T {
            self.items.end_ptr()
        }
        /// `clear()` — drop every element; the backing capacity is retained.
        #[inline]
        pub unsafe fn clear(&mut self) {
            self.items.clear();
        }
        /// `push_front(v)` — PREPEND. Insert at the front of the contiguous
        /// backing (shifting the existing elements up), so a forward walk
        /// `begin()..end()` visits the most-recently-pushed element first —
        /// exactly `std::list::push_front`. `relocate` runs the element's
        /// move/destroy recipe on the shift for a non-bitwise-relocatable
        /// element (`None` shifts bitwise, proven identical by the classifier).
        #[inline]
        pub unsafe fn push_front(
            &mut self,
            value: T,
            relocate: Option<unsafe fn(*mut T, *mut T)>,
        ) {
            let front = self.items.begin_ptr();
            self.items.insert_one_relocating(front, value, relocate);
        }
        /// `push_back(v)` — APPEND.
        #[inline]
        pub unsafe fn push_back(
            &mut self,
            value: T,
            relocate: Option<unsafe fn(*mut T, *mut T)>,
        ) {
            self.items.push_back_relocating(value, relocate);
        }
    }
    /// `std::list<T>(const std::list<T>&)` — deep copy: clone the backing
    /// element array, so the copy owns independent elements (requires
    /// `T: Clone`) in the same order.
    impl<T: Clone> Clone for List<T> {
        fn clone(&self) -> Self {
            List { items: self.items.clone() }
        }
    }
    /// `std::map<K, V>` / `std::unordered_map<K, V>`: an associative container
    /// stored as a key-sorted array of `Pair<K, V>` entries (binary search for
    /// lookup, shift-insert to keep order). `std::map` iterates in ascending
    /// key order — reproduced exactly; `std::unordered_map`'s iteration order
    /// is unspecified, so the same deterministic ordering is a conforming
    /// observable choice. Backed by the support `Vector` (heap storage, dropped
    /// with the map). `K: Ord` orders the keys; `V: Copy` matches these POD
    /// value types (a later rung generalises to non-trivial values).
    ///
    /// Entry storage moves BITWISE on insert (growth and shift) by design:
    /// C++'s node-based `std::map` runs NO element code on insert (existing
    /// nodes are untouched), so bitwise — which also runs none — is the
    /// faithful per-element behaviour; the array representation's residual
    /// observable difference is REFERENCE STABILITY (C++ guarantees
    /// references/iterators into a map survive unrelated inserts; an array
    /// reallocation invalidates them), a documented representation boundary
    /// of this rung.
    pub struct Map<K, V> {
        entries: Vector<Pair<K, V>>,
    }
    impl<K, V> Map<K, V> {
        #[inline]
        pub fn new() -> Self {
            Map { entries: Vector::new() }
        }
        #[inline]
        pub fn size(&self) -> libc::c_ulong {
            self.entries.size()
        }
        /// `max_size()`: the theoretical entry-count ceiling — delegated to the
        /// backing entry vector, exactly as `size()` is. `std::map::max_size()`
        /// (which nlohmann's `basic_json::max_size()` calls for an object) maps
        /// here.
        #[inline]
        pub fn max_size(&self) -> libc::c_ulong {
            self.entries.max_size()
        }
        #[inline]
        pub fn empty(&self) -> bool {
            self.entries.empty()
        }
        /// `begin()` — pointer to the first entry (ascending key order).
        #[inline]
        pub unsafe fn begin_ptr(&self) -> *mut Pair<K, V> {
            self.entries.begin_ptr()
        }
        /// `end()` — one past the last entry.
        #[inline]
        pub unsafe fn end_ptr(&self) -> *mut Pair<K, V> {
            self.entries.end_ptr()
        }
        /// `erase(pos)`: remove the entry at iterator `pos` and return the
        /// iterator to the following entry — `std::map::erase(iterator)`.
        /// Position-based, so it neither orders nor copies keys (no bound).
        #[inline]
        pub unsafe fn erase(&mut self, pos: *mut Pair<K, V>) -> *mut Pair<K, V> {
            self.entries.erase(pos)
        }
        /// `clear()`: drop every entry; capacity is retained.
        #[inline]
        pub unsafe fn clear(&mut self) {
            self.entries.clear();
        }
    }
    /// `std::map<K, V>(const std::map<K, V>&)` — the deep copy constructor:
    /// clone the key-sorted entry array (each `Pair<K, V>` clones its key and
    /// value), so the copy owns independent keys/values (requires `K: Clone`,
    /// `V: Clone`) and never aliases the source's. The element order is
    /// preserved, so the clone is already in sorted-key order.
    impl<K: Clone, V: Clone> Clone for Map<K, V> {
        fn clone(&self) -> Self {
            Map {
                entries: self.entries.clone(),
            }
        }
    }
    /// `std::map::operator==`: equal size and element-wise `==` over the entries
    /// in (shared) key-sorted order — exactly the backing entry vector's
    /// `PartialEq` (`Vector<Pair<K, V>>` -> `Pair` -> key/value `==`). Both maps
    /// keep their entries sorted, so positional comparison is the ordered
    /// comparison.
    impl<K: PartialEq, V: PartialEq> PartialEq for Map<K, V> {
        #[inline]
        fn eq(&self, other: &Self) -> bool {
            self.entries == other.entries
        }
    }
    impl<K: PartialOrd, V: PartialOrd> Map<K, V> {
        /// `std::map::operator<`: lexicographical comparison of the key-sorted
        /// entries using each `Pair`'s `<` (key then value) — the entry vector's
        /// `lex_less`. Faithful to `std::lexicographical_compare` (see
        /// [`Vector::lex_less`]); the orderings `>`/`<=`/`>=` derive from it.
        #[inline]
        pub fn lex_less(&self, other: &Self) -> bool {
            self.entries.lex_less(&other.entries)
        }
    }
    impl<K: Ord + Clone, V> Map<K, V> {
        /// Binary search for `k`: `Ok(i)` = present at index `i`; `Err(i)` =
        /// the sorted insertion point. Compares the stored key BY REFERENCE so a
        /// non-`Copy` key is never moved out of the entry.
        #[inline]
        unsafe fn search(&self, k: &K) -> Result<usize, usize> {
            let mut lo = 0usize;
            let mut hi = self.entries.len;
            while lo < hi {
                let mid = lo + (hi - lo) / 2;
                match (*self.entries.at(mid)).first.cmp(k) {
                    core::cmp::Ordering::Less => lo = mid + 1,
                    core::cmp::Ordering::Greater => hi = mid,
                    core::cmp::Ordering::Equal => return Ok(mid),
                }
            }
            Err(lo)
        }
        /// `operator[]`: storage location for key `k`, inserting a
        /// VALUE-INITIALISED slot if `k` is absent — exactly
        /// `std::map::operator[]`. This zero-initialising form is emitted only
        /// when the lowerer PROVED zero-init equals C++ value-initialisation
        /// for the mapped type (scalars, pointers, the support containers'
        /// empty states, records with neither user constructors nor in-class
        /// initialisers nor a vtable); a mapped type whose value-init runs
        /// code dispatches to [`Self::index_mut_or_insert_with`] instead. The
        /// key is COPIED into a newly-inserted entry (libstdc++'s node copy).
        #[inline]
        pub unsafe fn index_mut(&mut self, k: &K) -> *mut V {
            self.index_mut_or_insert_with(k, || core::mem::zeroed())
        }
        /// `operator[]` with an explicit value-initialiser for the absent-key
        /// insert: `init` is evaluated ONLY when the key is missing, exactly
        /// when C++ constructs the new mapped value (a present key constructs
        /// nothing, so a constructor with side effects fires identically).
        #[inline]
        pub unsafe fn index_mut_or_insert_with<F: FnOnce() -> V>(
            &mut self,
            k: &K,
            init: F,
        ) -> *mut V {
            match self.search(k) {
                Ok(i) => ptr::addr_of_mut!((* self.entries.at(i)).second),
                Err(i) => {
                    let n = self.entries.len;
                    self.entries.grow_to(n + 1);
                    let base = self.entries.ptr;
                    let mut j = n;
                    while j > i {
                        ptr::write(base.add(j), ptr::read(base.add(j - 1)));
                        j -= 1;
                    }
                    ptr::write(
                        base.add(i),
                        Pair {
                            first: k.clone(),
                            second: init(),
                        },
                    );
                    self.entries.len = n + 1;
                    ptr::addr_of_mut!((* base.add(i)).second)
                }
            }
        }
        /// `count(k)` — 0 or 1 for a unique-key map.
        #[inline]
        pub unsafe fn count(&self, k: &K) -> libc::c_ulong {
            match self.search(k) {
                Ok(_) => 1,
                Err(_) => 0,
            }
        }
        /// `find(k)` — the iterator (`*mut Pair`) to the entry with key `k`, or
        /// `end()` when absent (`std::map::find`).
        #[inline]
        pub unsafe fn find(&self, k: &K) -> *mut Pair<K, V> {
            match self.search(k) {
                Ok(i) => self.entries.at(i),
                Err(_) => self.entries.end_ptr(),
            }
        }
        /// `at(k)` — pointer to the mapped value for key `k`, aborting (the
        /// faithful observable of `std::map::at`'s `std::out_of_range` throw
        /// under `panic = abort`) when absent. Unlike `operator[]` it never
        /// inserts. Returns `*mut V` so the caller dereferences to a place.
        #[inline]
        pub unsafe fn at(&self, k: &K) -> *mut V {
            match self.search(k) {
                Ok(i) => ptr::addr_of_mut!((* self.entries.at(i)).second),
                Err(_) => panic!("std::map::at: key not found"),
            }
        }
        /// `std::map::erase(const key_type&)` — the by-KEY erase. Remove the
        /// entry whose key equals `k`, if present, and return the number of
        /// elements removed: 1 for a unique-key map, 0 when the key is absent.
        /// Distinct from the iterator-position [`Self::erase`] (Rust has no
        /// overloading, hence the separate name); the removal itself delegates
        /// to the backing entry vector's positional erase, which drops the
        /// entry (its key and value) and shifts the tail down, keeping the
        /// sorted order. The key is compared BY REFERENCE, so a non-`Copy` key
        /// is never moved out — matching the other key operations.
        #[inline]
        pub unsafe fn erase_key(&mut self, k: &K) -> libc::c_ulong {
            match self.search(k) {
                Ok(i) => {
                    let pos = self.entries.at(i);
                    self.entries.erase(pos);
                    1
                }
                Err(_) => 0,
            }
        }
        /// `emplace(k, v)` — insert if (and only if) the key is absent, exactly
        /// `std::map::emplace`'s unique-key behaviour, returning the iterator
        /// to the (existing or new) entry and whether the insertion happened.
        /// The duplicate-key arguments are dropped, matching the discarded
        /// node libstdc++ destroys.
        #[inline]
        pub unsafe fn emplace(&mut self, k: K, v: V) -> Pair<*mut Pair<K, V>, bool> {
            match self.search(&k) {
                Ok(i) => {
                    Pair {
                        first: self.entries.at(i),
                        second: false,
                    }
                }
                Err(i) => {
                    let n = self.entries.len;
                    self.entries.grow_to(n + 1);
                    let base = self.entries.ptr;
                    let mut j = n;
                    while j > i {
                        ptr::write(base.add(j), ptr::read(base.add(j - 1)));
                        j -= 1;
                    }
                    ptr::write(base.add(i), Pair { first: k, second: v });
                    self.entries.len = n + 1;
                    Pair {
                        first: base.add(i),
                        second: true,
                    }
                }
            }
        }
        /// `std::map::insert(const value_type&)` — the single-`pair` insert:
        /// unique-key insert (keeping the existing entry on a duplicate key),
        /// returning the iterator to the (existing or new) entry and whether the
        /// insertion happened — exactly `emplace`'s contract, but taking the
        /// `value_type` (`Pair<K, V>`) the way `std::map::insert` does. The pair
        /// is COPIED into the map (nlohmann's object `push_back(const
        /// object_t::value_type&)`); on a duplicate key the copy is dropped,
        /// matching the discarded node libstdc++ destroys.
        #[inline]
        pub unsafe fn insert_value(
            &mut self,
            val: Pair<K, V>,
        ) -> Pair<*mut Pair<K, V>, bool> {
            self.emplace(val.first, val.second)
        }
        /// `insert({k, v})` keeping the FIRST entry for a duplicate key — the
        /// observable libstdc++ behaviour, so a brace-initialised map
        /// (`std::map<K, V> m = {{k1, v1}, …}`) is built byte-identically. The
        /// key is taken by value (the brace element already owns it).
        #[inline]
        pub unsafe fn insert(&mut self, k: K, v: V) {
            match self.search(&k) {
                Ok(_) => {}
                Err(i) => {
                    let n = self.entries.len;
                    self.entries.grow_to(n + 1);
                    let base = self.entries.ptr;
                    let mut j = n;
                    while j > i {
                        ptr::write(base.add(j), ptr::read(base.add(j - 1)));
                        j -= 1;
                    }
                    ptr::write(base.add(i), Pair { first: k, second: v });
                    self.entries.len = n + 1;
                }
            }
        }
    }
    /// The `<stdexcept>` / `<exception>` hierarchy (std::exception,
    /// std::runtime_error, std::logic_error, …) collapsed onto one support
    /// type: an owned, NUL-terminated copy of the what() message. Throwing
    /// constructs one and `panic_any`s it; a `catch` downcasts the payload to
    /// this same type, so catching any base matches a thrown derived. The raw
    /// message pointer is not `Send`, but these programs are single-threaded and
    /// the panic payload never crosses a thread, so the `Send` assertion is
    /// sound.
    pub struct Exception {
        msg: *mut libc::c_char,
    }
    unsafe impl Send for Exception {}
    impl Exception {
        /// Construct from a C string (`std::runtime_error("boom")`), copying it
        /// into owned storage so the message outlives the throw expression.
        #[inline]
        pub unsafe fn from_cstr(s: *const libc::c_char) -> Exception {
            let n = if s.is_null() { 0 } else { libc::strlen(s) };
            let p = libc::malloc(n + 1) as *mut libc::c_char;
            assert!(! p.is_null(), "std::exception message allocation failed");
            if n > 0 {
                ptr::copy_nonoverlapping(s, p, n);
            }
            *p.add(n) = 0;
            Exception { msg: p }
        }
        /// `what()` — the stored NUL-terminated message.
        #[inline]
        pub fn what(&self) -> *const libc::c_char {
            self.msg as *const libc::c_char
        }
    }
    /// The std exception copy constructor: a fresh OWNED copy of the message, so
    /// the copy and the original each free their own storage (no double free).
    /// Needed because C++ `throw ex` of a `const exception&` copy-initialises the
    /// exception object from `ex`.
    impl Clone for Exception {
        fn clone(&self) -> Self {
            unsafe { Exception::from_cstr(self.msg as *const libc::c_char) }
        }
    }
    impl Drop for Exception {
        fn drop(&mut self) {
            unsafe {
                if !self.msg.is_null() {
                    libc::free(self.msg as *mut libc::c_void);
                }
            }
        }
    }
    /// `std::ostream` (`std::cout` / `std::cerr` / `std::clog`): a thin handle
    /// over a file descriptor plus the two formatting-state fields the supported
    /// manipulators touch. The `<<` insertion operators write the formatted
    /// bytes straight to the descriptor and return `self` so chains compose.
    /// Writing through `libc::write` reproduces, byte for byte, what libstdc++'s
    /// `cout`/`cerr` ultimately emit.
    pub struct Ostream {
        fd: libc::c_int,
        /// Field width for the NEXT inserted value (`std::setw` /
        /// `std::ios_base::width`): the value is right-justified in a field of
        /// this width, padded with `fill`. Reset to 0 after one insertion,
        /// exactly as the standard specifies.
        width: usize,
        /// Fill character (`std::setfill` / `std::ios_base::fill`), default
        /// space; pads a value narrower than `width` on the left (right
        /// adjustment — libstdc++'s default `adjustfield`).
        fill: u8,
        /// Whether this handle OWNS its descriptor — true for a `std::ofstream`
        /// (opened a file, closes it on `close()`/drop), false for the process
        /// stream handles `cout`/`cerr`/`clog` (fd 1/2, never closed). Keeps the
        /// file-backed and process-stream uses of the one `Ostream` type from
        /// closing each other's descriptors.
        owns_fd: bool,
    }
    /// Stable backing objects for the stream GLOBALS, so `&std::cerr`
    /// (ACE's `ACE_DEFAULT_LOG_STREAM`, stored and used later) yields a
    /// pointer to a durable object rather than the address of a temporary
    /// (E0745). The descriptor side is fixed (fd 1/2) and the formatting
    /// state defaults; a VALUE reference (`std::cerr << x`) still reads a
    /// fresh `Ostream::cerr()` value, so only address/place uses observe
    /// these — exactly where C++ needs a real object identity.
    pub static mut __COUT: Ostream = Ostream {
        fd: 1,
        width: 0,
        fill: b' ',
        owns_fd: false,
    };
    pub static mut __CERR: Ostream = Ostream {
        fd: 2,
        width: 0,
        fill: b' ',
        owns_fd: false,
    };
    pub static mut __CLOG: Ostream = Ostream {
        fd: 2,
        width: 0,
        fill: b' ',
        owns_fd: false,
    };
    impl Ostream {
        /// `std::cout` / `std::cerr` / `std::clog`. The descriptor side is
        /// stateless, so a fresh value per reference is observationally
        /// identical to the single global (and avoids a `static mut`); the
        /// formatting state starts at the stream defaults (no width, space
        /// fill) and persists through a single insertion chain, which is where
        /// `setw`/`setfill`/`fill` are observed.
        #[inline]
        pub fn cout() -> Ostream {
            Ostream {
                fd: 1,
                width: 0,
                fill: b' ',
                owns_fd: false,
            }
        }
        #[inline]
        pub fn cerr() -> Ostream {
            Ostream {
                fd: 2,
                width: 0,
                fill: b' ',
                owns_fd: false,
            }
        }
        #[inline]
        pub fn clog() -> Ostream {
            Ostream {
                fd: 2,
                width: 0,
                fill: b' ',
                owns_fd: false,
            }
        }
        /// `std::ofstream(path)` / `std::ofstream(path, mode)` — open a file for
        /// output. libstdc++ `openmode` bits: `app = 1`, `out = 16` (see
        /// `ios`); a 0 mode (the one-argument constructor) defaults to
        /// `out | trunc`. Bytes written by `<<` go straight to this descriptor,
        /// so the file's contents match libstdc++'s flushed `ofstream` output.
        #[inline]
        pub unsafe fn from_file(
            path: *const libc::c_char,
            mode: libc::c_int,
        ) -> Ostream {
            let mut o = Ostream {
                fd: -1,
                width: 0,
                fill: b' ',
                owns_fd: true,
            };
            o.open_fd(path, mode);
            o
        }
        /// Shared open: translate the C++ `openmode` bitmask to `open(2)` flags
        /// and (re)open the file. `out` implies write+create; `app` appends,
        /// otherwise the file is truncated — matching `std::basic_filebuf`'s
        /// mode mapping for an output file stream.
        #[inline]
        unsafe fn open_fd(&mut self, path: *const libc::c_char, mode: libc::c_int) {
            let app = (mode & crate::__cxx_std::ios::app) != 0;
            let mut flags = libc::O_WRONLY | libc::O_CREAT;
            if app {
                flags |= libc::O_APPEND;
            } else {
                flags |= libc::O_TRUNC;
            }
            self.fd = libc::open(path, flags, 0o644 as libc::c_int);
            self.owns_fd = true;
        }
        /// `std::ofstream::open(path, mode)` — (re)open the stream on a file.
        #[inline]
        pub unsafe fn open(&mut self, path: *const libc::c_char, mode: libc::c_int) {
            self.open_fd(path, mode);
        }
        /// `std::ofstream::close()` — close an OWNED descriptor (a no-op for the
        /// process streams, whose fd 1/2 must survive).
        #[inline]
        pub unsafe fn close(&mut self) {
            if self.owns_fd && self.fd >= 0 {
                libc::close(self.fd);
                self.fd = -1;
            }
        }
        /// `std::basic_ios::rdstate()` — the `iostate` bitmask. A successfully
        /// opened descriptor is `goodbit` (0); a failed open reports `failbit`
        /// (`ACE_Logging_Strategy` compares `rdstate() != ios::goodbit` to detect
        /// a file that could not be opened).
        #[inline]
        pub fn rdstate(&self) -> libc::c_int {
            if self.fd >= 0 {
                crate::__cxx_std::ios::goodbit
            } else {
                crate::__cxx_std::ios::failbit
            }
        }
        /// `std::setw(n)` (and `std::ios_base::width(n)`): set the field width for
        /// the next inserted value. Returns `self` so it composes in a `<<`
        /// chain (`o << std::setw(6) << v`).
        #[inline]
        pub fn setw(&mut self, n: libc::c_int) -> &mut Self {
            self.width = if n > 0 { n as usize } else { 0 };
            self
        }
        /// `std::setfill(c)` as a `<<` manipulator: set the fill character and
        /// return `self` for chaining.
        #[inline]
        pub fn setfill(&mut self, c: libc::c_char) -> &mut Self {
            self.fill = c as u8;
            self
        }
        /// `o.fill()` member: the current fill character.
        #[inline]
        pub fn fill(&self) -> libc::c_char {
            self.fill as libc::c_char
        }
        /// `o.fill(c)` member: set the fill character, returning the PREVIOUS one
        /// (libstdc++'s `basic_ios::fill(char_type)` contract).
        #[inline]
        pub fn set_fill(&mut self, c: libc::c_char) -> libc::c_char {
            let old = self.fill;
            self.fill = c as u8;
            old as libc::c_char
        }
        /// `o.flush()` — force pending output. Every `<<` writes its bytes
        /// straight to the descriptor via `libc::write` (there is no user-space
        /// buffer), so the bytes are already delivered; flush is a no-op that
        /// returns the stream for chaining. The final byte stream is therefore
        /// identical to libstdc++'s flushed output.
        #[inline]
        pub fn flush(&mut self) -> &mut Self {
            self
        }
        /// `o.tellp()` — the current put position. Because every `<<` writes
        /// immediately (no user-space buffer), the descriptor's offset IS the
        /// number of bytes put, so `lseek(fd, 0, SEEK_CUR)` reproduces
        /// libstdc++'s `tellp()` exactly: a seekable target (a log file) reports
        /// the byte offset, a non-seekable one (a pipe/terminal) reports the
        /// same failure value libstdc++ returns. ACE's `ACE_Logging_Strategy`
        /// compares it to the configured max size to decide log rotation.
        #[inline]
        pub fn tellp(&self) -> libc::off_t {
            unsafe { libc::lseek(self.fd, 0, libc::SEEK_CUR) }
        }
        /// Write `len` bytes of `s` right-justified in the pending field width
        /// (`fill`-padded on the left), then clear the width — the shared
        /// back-end of every `put_*`. `width == 0` (the common case) writes the
        /// bytes verbatim.
        #[inline]
        unsafe fn put_field(&mut self, s: *const libc::c_char, len: usize) {
            let w = self.width;
            self.width = 0;
            if w > len {
                let f = self.fill;
                for _ in 0..(w - len) {
                    libc::write(self.fd, &f as *const u8 as *const libc::c_void, 1);
                }
            }
            if len > 0 {
                libc::write(self.fd, s as *const libc::c_void, len);
            }
        }
        /// `<< (const char*)` / `<< (string literal)`: write the NUL-terminated
        /// bytes (field-justified if a width is pending).
        #[inline]
        pub unsafe fn put_cstr(&mut self, s: *const libc::c_char) -> &mut Self {
            if !s.is_null() {
                let n = libc::strlen(s);
                self.put_field(s, n);
            }
            self
        }
        /// `<< (signed integer)`: decimal, as `printf("%ld")` (the digits are
        /// identical to libstdc++'s default `num_put` formatting), field-justified.
        #[inline]
        pub unsafe fn put_long(&mut self, v: libc::c_long) -> &mut Self {
            let mut buf = [0 as libc::c_char; 32];
            let n = libc::snprintf(
                buf.as_mut_ptr(),
                buf.len(),
                b"%ld\0".as_ptr() as *const libc::c_char,
                v,
            );
            if n > 0 {
                self.put_field(buf.as_ptr(), n as usize);
            }
            self
        }
        /// `<< (unsigned integer)`: decimal, as `printf("%lu")`.
        #[inline]
        pub unsafe fn put_ulong(&mut self, v: libc::c_ulong) -> &mut Self {
            let mut buf = [0 as libc::c_char; 32];
            let n = libc::snprintf(
                buf.as_mut_ptr(),
                buf.len(),
                b"%lu\0".as_ptr() as *const libc::c_char,
                v,
            );
            if n > 0 {
                self.put_field(buf.as_ptr(), n as usize);
            }
            self
        }
        /// `<< (double)`: libstdc++'s default float formatting is `%g` with
        /// precision 6.
        #[inline]
        pub unsafe fn put_double(&mut self, v: f64) -> &mut Self {
            let mut buf = [0 as libc::c_char; 64];
            let n = libc::snprintf(
                buf.as_mut_ptr(),
                buf.len(),
                b"%g\0".as_ptr() as *const libc::c_char,
                v,
            );
            if n > 0 {
                self.put_field(buf.as_ptr(), n as usize);
            }
            self
        }
        /// `<< (char)`: write the single byte (field-justified if a width is
        /// pending).
        #[inline]
        pub unsafe fn put_char(&mut self, c: libc::c_char) -> &mut Self {
            let b = c;
            self.put_field(&b as *const libc::c_char, 1);
            self
        }
    }
    /// A file-backed `Ostream` (a `std::ofstream`) closes its descriptor when it
    /// goes out of scope — the `std::basic_filebuf` destructor. The process
    /// stream handles (`cout`/`cerr`/`clog`, `owns_fd == false`) keep their fd
    /// 1/2 open, so their drop is a no-op; every `<<` already wrote its bytes
    /// straight to the descriptor, so there is nothing to flush.
    impl Drop for Ostream {
        fn drop(&mut self) {
            if self.owns_fd && self.fd >= 0 {
                unsafe {
                    libc::close(self.fd);
                }
            }
        }
    }
    /// `std::ios_base`'s `iostate` error-flag bits (libstdc++'s `_Ios_Iostate`
    /// enumerators) and `openmode` bits (`_Ios_Openmode`). The enums lower to
    /// `c_int`; these are their exact values, and the free `operator&`/
    /// `operator|` over the bitmasks lower to the bitwise operators on the
    /// underlying integer — libstdc++'s own definition.
    pub mod ios {
        pub const goodbit: libc::c_int = 0;
        pub const badbit: libc::c_int = 1;
        pub const eofbit: libc::c_int = 2;
        pub const failbit: libc::c_int = 4;
        pub const app: libc::c_int = 1;
        pub const ate: libc::c_int = 2;
        pub const binary: libc::c_int = 4;
        pub const r#in: libc::c_int = 8;
        pub const out: libc::c_int = 16;
        pub const trunc: libc::c_int = 32;
    }
    /// `std::basic_streambuf<char>` (`std::streambuf`): the byte buffer beneath
    /// an input stream. libstdc++ maintains a *get area* — a `[gptr, egptr)`
    /// window of buffered input bytes — and `sbumpc` returns the byte at `gptr`
    /// and advances it, refilling from the producer (`underflow`) when the
    /// window empties and returning `eof()` when there is nothing left. The
    /// support `Streambuf` models that window over raw bytes; with no producer
    /// attached an exhausted window IS end-of-input, so `sbumpc` returns EOF.
    pub struct Streambuf {
        /// `gptr()` — next input byte to consume.
        gptr: *const u8,
        /// `egptr()` — one past the last buffered input byte.
        egptr: *const u8,
    }
    impl Streambuf {
        /// `sbumpc()`: return the current byte as an `int_type` and advance the
        /// get pointer, or `char_traits<char>::eof()` (`-1`) when the get area
        /// is exhausted.
        #[inline]
        pub unsafe fn sbumpc(&mut self) -> libc::c_int {
            if self.gptr >= self.egptr {
                return -1;
            }
            let c = *self.gptr;
            self.gptr = self.gptr.add(1);
            c as libc::c_int
        }
        /// `sgetc()`: PEEK the current byte without advancing the get pointer, or
        /// `char_traits<char>::eof()` (`-1`) when the get area is exhausted.
        #[inline]
        pub unsafe fn sgetc(&self) -> libc::c_int {
            if self.gptr >= self.egptr { -1 } else { *self.gptr as libc::c_int }
        }
    }
    /// `std::basic_istream<char>` (`std::istream`): a formatted-input stream
    /// over a `Streambuf`, carrying the `iostate` error flags of its
    /// `basic_ios` base. Only the members `input_stream_adapter` exercises are
    /// modelled: `rdbuf` (reach the buffer), `rdstate` (read the flags) and
    /// `clear` (replace them).
    pub struct Istream {
        /// `basic_ios::rdbuf()` — the associated stream buffer.
        sb: *mut Streambuf,
        /// `basic_ios::rdstate()` — the `iostate` bitmask (see `ios`).
        state: libc::c_int,
    }
    impl Istream {
        /// `rdbuf()`: the associated `streambuf`.
        #[inline]
        pub fn rdbuf(&mut self) -> *mut Streambuf {
            self.sb
        }
        /// `rdstate()`: the current `iostate` flags.
        #[inline]
        pub fn rdstate(&self) -> libc::c_int {
            self.state
        }
        /// `clear(state)`: replace the `iostate` flags wholesale (the default
        /// `goodbit` argument is supplied explicitly at every call site).
        #[inline]
        pub fn clear(&mut self, state: libc::c_int) {
            self.state = state;
        }
        /// `operator>>(double&)` — formatted floating-point extraction, faithful
        /// to libstdc++'s classic-"C"-locale `std::num_get<char>::do_get`:
        ///   * `basic_istream::sentry` skips leading whitespace; an empty /
        ///     whitespace-only stream fails the sentry (`failbit|eofbit`) and
        ///     leaves the target UNCHANGED;
        ///   * `_M_extract_float` reads the C-locale float grammar — one optional
        ///     sign, a mantissa of digits collapsing leading zeros to one, a
        ///     single decimal point, and an `e`/`E` exponent with its own sign —
        ///     into a token, stopping at the first non-matching byte (left in the
        ///     stream) or end of input;
        ///   * `__convert_to_v` runs `strtod` over the token: a token `strtod`
        ///     does not consume IN FULL (empty, a lone `.`, a trailing `e`) stores
        ///     `0` and sets `failbit`; an overflow (`strtod` -> ±infinity) stores
        ///     the signed `DBL_MAX` and sets `failbit`; an underflow stores the
        ///     (zero) result with NO failbit;
        ///   * reaching end of input during extraction sets `eofbit`.
        /// Returns the stream for chaining. `_S_get_c_locale()` is the C locale,
        /// so `strtod` (not `__strtod_l`) reproduces the conversion exactly.
        #[inline]
        pub unsafe fn extract_f64(&mut self, out: *mut libc::c_double) -> &mut Self {
            if self.sb.is_null() {
                self.state |= ios::failbit;
                return self;
            }
            let sb = &mut *self.sb;
            let is_space = |c: libc::c_int| {
                matches!(c, 0x20 | 0x09 | 0x0a | 0x0b | 0x0c | 0x0d)
            };
            let mut c = sb.sgetc();
            while c != -1 && is_space(c) {
                sb.sbumpc();
                c = sb.sgetc();
            }
            if c == -1 {
                self.state |= ios::eofbit | ios::failbit;
                return self;
            }
            let mut xtrc: [u8; 64] = [0; 64];
            let mut n: usize = 0;
            let mut push = |b: u8| {
                if n < xtrc.len() - 1 {
                    xtrc[n] = b;
                    n += 1;
                }
            };
            let mut testeof = false;
            if c == b'+' as libc::c_int || c == b'-' as libc::c_int {
                push(if c == b'+' as libc::c_int { b'+' } else { b'-' });
                sb.sbumpc();
                c = sb.sgetc();
                if c == -1 {
                    testeof = true;
                }
            }
            let mut found_mantissa = false;
            while !testeof {
                if c == b'.' as libc::c_int {
                    break;
                }
                if c == b'0' as libc::c_int {
                    if !found_mantissa {
                        push(b'0');
                        found_mantissa = true;
                    }
                    sb.sbumpc();
                    c = sb.sgetc();
                    if c == -1 {
                        testeof = true;
                    }
                } else {
                    break;
                }
            }
            let mut found_dec = false;
            let mut found_sci = false;
            while !testeof {
                if (b'0' as libc::c_int..=b'9' as libc::c_int).contains(&c) {
                    push(c as u8);
                    found_mantissa = true;
                } else if c == b'.' as libc::c_int && !found_dec && !found_sci {
                    push(b'.');
                    found_dec = true;
                } else if (c == b'e' as libc::c_int || c == b'E' as libc::c_int)
                    && !found_sci && found_mantissa
                {
                    push(b'e');
                    found_sci = true;
                    sb.sbumpc();
                    c = sb.sgetc();
                    if c == -1 {
                        testeof = true;
                        break;
                    }
                    if c == b'+' as libc::c_int || c == b'-' as libc::c_int {
                        push(if c == b'+' as libc::c_int { b'+' } else { b'-' });
                    } else {
                        continue;
                    }
                } else {
                    break;
                }
                sb.sbumpc();
                c = sb.sgetc();
                if c == -1 {
                    testeof = true;
                }
            }
            xtrc[n] = 0;
            let mut endptr: *mut libc::c_char = ptr::null_mut();
            let start = xtrc.as_ptr() as *const libc::c_char;
            let d = libc::strtod(start, &mut endptr);
            let consumed_all = endptr as *const libc::c_char != start
                && *(endptr as *const u8) == 0;
            if consumed_all {
                if d.is_infinite() {
                    *out = if d > 0.0 { f64::MAX } else { -f64::MAX };
                    self.state |= ios::failbit;
                } else {
                    *out = d;
                }
            } else {
                *out = 0.0;
                self.state |= ios::failbit;
            }
            if testeof {
                self.state |= ios::eofbit;
            }
            self
        }
    }
    /// `std::basic_istringstream<char>` (`std::istringstream`): an input stream
    /// that OWNS its character buffer (a copy of the string it was constructed
    /// from) and reads from it. It IS-A `basic_istream`, so the formatted
    /// extractors and `basic_ios` state queries are provided here directly over
    /// an embedded `Streambuf` get-area + `iostate`.
    pub struct Istringstream {
        sb: Streambuf,
        state: libc::c_int,
        /// Owned heap copy of the constructed content; freed on drop.
        buf: *mut u8,
    }
    impl Istringstream {
        /// `istringstream(const string&[, openmode])`: copy the string's bytes
        /// into an owned buffer and open a get-area over it. The open-mode is
        /// always input here, so it is ignored.
        #[inline]
        pub unsafe fn from_string(s: &String) -> Self {
            let len = s.len;
            let buf = libc::malloc(if len == 0 { 1 } else { len }) as *mut u8;
            if len > 0 {
                ptr::copy_nonoverlapping(s.ptr, buf, len);
            }
            Istringstream {
                sb: Streambuf {
                    gptr: buf,
                    egptr: buf.add(len),
                },
                state: ios::goodbit,
                buf,
            }
        }
        /// `operator>>(double&)` — drive the shared `Istream` extractor over this
        /// stream's own get-area and write the resulting `iostate` back.
        #[inline]
        pub unsafe fn extract_f64(&mut self, out: *mut libc::c_double) -> &mut Self {
            let mut is = Istream {
                sb: &mut self.sb,
                state: self.state,
            };
            is.extract_f64(out);
            self.state = is.state;
            self
        }
        /// `basic_ios::fail()` — `failbit` or `badbit` set.
        #[inline]
        pub fn fail(&self) -> bool {
            (self.state & (ios::failbit | ios::badbit)) != 0
        }
        /// `basic_ios::eof()` — `eofbit` set.
        #[inline]
        pub fn eof(&self) -> bool {
            (self.state & ios::eofbit) != 0
        }
        /// `basic_ios::good()` — no error bits set.
        #[inline]
        pub fn good(&self) -> bool {
            self.state == ios::goodbit
        }
        /// `basic_ios::rdstate()` — the raw `iostate`.
        #[inline]
        pub fn rdstate(&self) -> libc::c_int {
            self.state
        }
        /// `basic_ios::clear(state)` — replace the `iostate`.
        #[inline]
        pub fn clear(&mut self, state: libc::c_int) {
            self.state = state;
        }
        /// `basic_ios::operator bool()` / `operator!` test `!fail()`.
        #[inline]
        pub fn is_ok(&self) -> bool {
            !self.fail()
        }
    }
    impl Drop for Istringstream {
        #[inline]
        fn drop(&mut self) {
            unsafe {
                if !self.buf.is_null() {
                    libc::free(self.buf as *mut libc::c_void);
                }
            }
        }
    }
    /// `std::sort(first, last)` over a raw pointer range, ascending by `<`.
    /// Every conforming `std::sort` yields the same final ordering (the
    /// algorithm itself is unspecified); a stable in-place insertion sort
    /// reproduces it deterministically.
    #[inline]
    pub unsafe fn sort_range<T: Copy + PartialOrd>(first: *mut T, last: *mut T) {
        let n = last.offset_from(first) as usize;
        let mut i = 1usize;
        while i < n {
            let key = *first.add(i);
            let mut j = i;
            while j > 0 && *first.add(j - 1) > key {
                *first.add(j) = *first.add(j - 1);
                j -= 1;
            }
            *first.add(j) = key;
            i += 1;
        }
    }
    /// `std::reverse(first, last)` over a raw pointer range: reverse the
    /// elements of `[first, last)` in place by swapping outward-in pairs —
    /// exactly the libstdc++ algorithm for random-access iterators (an odd
    /// middle element stays put).
    #[inline]
    pub unsafe fn reverse_range<T>(mut first: *mut T, mut last: *mut T) {
        while first < last {
            last = last.sub(1);
            if first == last {
                break;
            }
            ptr::swap(first, last);
            first = first.add(1);
        }
    }
    /// `std::fill(first, last, value)` over a raw-pointer range: assign a
    /// copy of `value` to each element (ACE_Select_Reactor_Handler_
    /// Repository's `std::fill(handlers_.begin(), handlers_.end(), …)`).
    /// Assignment on an initialized element — the `Clone` write drops the
    /// old value, the memberwise copy this support library models.
    pub unsafe fn fill_range<T: Clone>(mut first: *mut T, last: *mut T, value: &T) {
        while first < last {
            *first = value.clone();
            first = first.add(1);
        }
    }
    /// `std::fill_n(first, n, value)` — the counted form; returns one past
    /// the last written element, as C++17 does.
    pub unsafe fn fill_n_range<T: Clone>(
        mut first: *mut T,
        n: usize,
        value: &T,
    ) -> *mut T {
        let mut i = 0usize;
        while i < n {
            *first = value.clone();
            first = first.add(1);
            i += 1;
        }
        first
    }
    /// `std::uninitialized_fill_n(first, n, value)`: COPY-CONSTRUCT `n`
    /// elements into RAW storage (ACE_Array_Map::grow) — `ptr::write`, not
    /// assignment, so no uninitialized value is dropped. Returns one past
    /// the last constructed element.
    pub unsafe fn uninitialized_fill_n_range<T: Clone>(
        mut first: *mut T,
        n: usize,
        value: &T,
    ) -> *mut T {
        let mut i = 0usize;
        while i < n {
            ptr::write(first, value.clone());
            first = first.add(1);
            i += 1;
        }
        first
    }
    /// `std::copy(first, last, out)` over raw-pointer ranges: assign each
    /// element in order, returning one past the last written (the C++
    /// result iterator). Overlap follows C++'s precondition (out must not
    /// be inside [first, last)).
    pub unsafe fn copy_range<T: Clone>(
        mut first: *const T,
        last: *const T,
        mut out: *mut T,
    ) -> *mut T {
        while first < last {
            *out = (*first).clone();
            first = first.add(1);
            out = out.add(1);
        }
        out
    }
    /// `std::uninitialized_copy(first, last, out)`: COPY-CONSTRUCT into raw
    /// storage (`ptr::write`), returning one past the last constructed.
    pub unsafe fn uninitialized_copy_range<T: Clone>(
        mut first: *const T,
        last: *const T,
        mut out: *mut T,
    ) -> *mut T {
        while first < last {
            ptr::write(out, (*first).clone());
            first = first.add(1);
            out = out.add(1);
        }
        out
    }
    /// In-place displacement of a raw-pointer iterator, abstracted over the
    /// pointee's constness. `std::advance` mutates the iterator variable; a
    /// vector/map iterator is a `*mut T`, but an INPUT iterator over `const`
    /// elements (`iterator_input_adapter<const char*>::current`) is a `*const
    /// T`. Both displace identically (`wrapping_offset`), so one trait serves
    /// both and the call site needs no constness-specific cast.
    pub trait __PtrIter: Copy {
        unsafe fn __advance(self, n: isize) -> Self;
    }
    impl<T> __PtrIter for *mut T {
        #[inline]
        unsafe fn __advance(self, n: isize) -> Self {
            self.wrapping_offset(n)
        }
    }
    impl<T> __PtrIter for *const T {
        #[inline]
        unsafe fn __advance(self, n: isize) -> Self {
            self.wrapping_offset(n)
        }
    }
    /// `std::advance(it, n)` for a RANDOM-ACCESS pointer iterator (what the
    /// support `vector`/`map` expose, and the lexer's input adapter). `it` is
    /// taken by mutable reference (a pointer-to-the-iterator) and displaced by
    /// `n` elements in place, exactly like `std::advance`'s random-access
    /// specialisation (`it += n`). `n` may be negative.
    #[inline]
    pub unsafe fn advance_ptr<P: __PtrIter>(it: *mut P, n: isize) {
        *it = (*it).__advance(n);
    }
    /// `std::accumulate(first, last, init)` — left fold of the range with `+`
    /// onto the accumulator `init` (its own type drives the arithmetic).
    #[inline]
    pub unsafe fn accumulate_range<T, A>(
        first: *const T,
        last: *const T,
        mut init: A,
    ) -> A
    where
        T: Copy,
        A: Copy + core::ops::Add<T, Output = A>,
    {
        let mut p = first;
        while p != last {
            init = init + *p;
            p = p.add(1);
        }
        init
    }
    /// `std::accumulate(first, last, init, binary_op)` — left fold of the range
    /// applying `binary_op(acc, *it)` for each element. C++ takes the accumulator
    /// and the element BY VALUE/reference into the binary op; the translated
    /// closure binds the C++ `const&` parameters as raw pointers, so `op` is
    /// called with the address of the accumulator and of each element (it reads
    /// through them and returns the new accumulator). Mirrors libstdc++'s
    /// `init = binary_op(std::move(init), *first)` element order exactly.
    #[inline]
    pub unsafe fn accumulate_range_with<T, A, F>(
        first: *const T,
        last: *const T,
        mut init: A,
        mut op: F,
    ) -> A
    where
        F: FnMut(*const A, *const T) -> A,
    {
        let mut p = first;
        while p != last {
            let next = op(&init as *const A, p);
            init = next;
            p = p.add(1);
        }
        init
    }
    /// `std::accumulate(first, last, init, binary_op)` for a binary op whose
    /// ACCUMULATOR parameter is taken BY VALUE (`size_t result`, not `const
    /// size_t& result`) — nlohmann's BSON/UBJSON element-size folds. The
    /// element is still bound by reference (a raw pointer). `init` is MOVED
    /// into the op and replaced by its result each step, matching libstdc++'s
    /// `init = binary_op(std::move(init), *first)` exactly. The by-reference
    /// accumulator overload above (`accumulate_range_with`) is chosen instead
    /// when the lambda binds the accumulator by `const&`.
    #[inline]
    pub unsafe fn accumulate_range_with_byval<T, A, F>(
        first: *const T,
        last: *const T,
        mut init: A,
        mut op: F,
    ) -> A
    where
        F: FnMut(A, *const T) -> A,
    {
        let mut p = first;
        while p != last {
            init = op(init, p);
            p = p.add(1);
        }
        init
    }
    /// Contextual conversion to `bool` (C++ "contextually convertible to
    /// bool"): a translated predicate body returns `bool` or, when its last
    /// expression is a C-style comparison, a `c_int` — accept either.
    pub trait __Truthy {
        fn __truthy(self) -> bool;
    }
    impl __Truthy for bool {
        #[inline]
        fn __truthy(self) -> bool {
            self
        }
    }
    macro_rules! __impl_truthy_int {
        ($($t:ty),*) => {
            $(impl __Truthy for $t { #[inline] fn __truthy(self) -> bool { self != 0 }
            })*
        };
    }
    __impl_truthy_int!(i8, i16, i32, i64, isize, u8, u16, u32, u64, usize);
    /// `std::find_if(first, last, pred)` — the first element satisfying `pred`,
    /// or `last` if none does. The predicate's result is taken in a boolean
    /// context (`__Truthy`), so a translated body yielding `bool` or `c_int`
    /// both work.
    #[inline]
    pub unsafe fn find_if_range<T: Copy, R: __Truthy, F: FnMut(T) -> R>(
        first: *mut T,
        last: *mut T,
        mut pred: F,
    ) -> *mut T {
        let mut p = first;
        while p != last {
            if pred(*p).__truthy() {
                return p;
            }
            p = p.add(1);
        }
        last
    }
    /// `std::find(first, last, value)` over a contiguous (pointer-iterator)
    /// range: the first element byte-comparing equal to `*value`, else `last`.
    /// The value is passed BY POINTER (the algorithm's `const T&` parameter).
    pub unsafe fn find_range<T: Copy + PartialEq>(
        first: *mut T,
        last: *mut T,
        value: *const T,
    ) -> *mut T {
        let mut p = first;
        while p != last {
            if *p == *value {
                return p;
            }
            p = p.add(1);
        }
        last
    }
    /// `std::lower_bound(first, last, value, comp)` over a SORTED contiguous
    /// (pointer-iterator) range: the first element for which `comp(elem, value)`
    /// is FALSE — the first not ordered strictly before `value`, else `last`.
    /// `comp` is the C++ comparator; its first argument is the element (the
    /// algorithm passes `*it`, bound to the comparator's `const T&` → a pointer),
    /// its second the search value passed by VALUE (the comparator's by-value
    /// parameter). Halving search, identical step count to libstdc++'s.
    pub unsafe fn lower_bound_range_with<
        T,
        V: Copy,
        R: __Truthy,
        F: FnMut(*const T, V) -> R,
    >(first: *const T, last: *const T, value: *const V, mut comp: F) -> *const T {
        let mut lo = first;
        let mut count = last.offset_from(first);
        while count > 0 {
            let half = count / 2;
            let mid = lo.add(half as usize);
            if comp(mid, *value).__truthy() {
                lo = mid.add((1) as usize);
                count -= half + 1;
            } else {
                count = half;
            }
        }
        lo
    }
    /// `std::binary_search(first, last, value)` over a SORTED contiguous range:
    /// whether an element EQUIVALENT to `*value` under `<` is present. Equivalent
    /// to `lower_bound` followed by the `!(value < *it)` check — i.e. neither
    /// `*it < value` (lower_bound's post-condition) nor `value < *it`.
    pub unsafe fn binary_search_range<T: PartialOrd>(
        first: *const T,
        last: *const T,
        value: *const T,
    ) -> bool {
        let mut lo = first;
        let mut count = last.offset_from(first);
        while count > 0 {
            let half = count / 2;
            let mid = lo.add(half as usize);
            if *mid < *value {
                lo = mid.add((1) as usize);
                count -= half + 1;
            } else {
                count = half;
            }
        }
        lo != last && !(*value < *lo)
    }
    /// `std::vector<T>(InputIt first, InputIt last)` — the iterator-range
    /// constructor: build a fresh vector by constructing each element from the
    /// corresponding source `*it` via `conv`. The source element type `S` may
    /// DIFFER from `T` (nlohmann's `create<array_t>(init.begin(), init.end())`
    /// converts each `json_ref` to a `basic_json` through `moved_or_copied()`);
    /// for a same-type range `conv` is the element clone.
    pub unsafe fn vector_from_range<S, T>(
        first: *const S,
        last: *const S,
        mut conv: impl FnMut(*const S) -> T,
    ) -> Vector<T> {
        let mut v: Vector<T> = Vector::new();
        let mut p = first;
        while p != last {
            v.push_back(conv(p));
            p = p.add(1);
        }
        v
    }
}

pub mod __f80 {
    use core::arch::asm;
    #[repr(C, align(16))]
    #[derive(Clone, Copy)]
    pub struct F80(pub [u8; 16]);
    macro_rules! bin {
        ($m:ident, $ins:literal) => {
            #[inline] pub fn $m (self, b : F80) -> F80 { let mut o = F80([0u8; 16]);
            unsafe { asm!("fld tbyte ptr [{b}]", "fld tbyte ptr [{a}]", $ins,
            "fstp tbyte ptr [{o}]", a = in (reg) & self, b = in (reg) & b, o = in (reg) &
            mut o, out("st(0)") _, out("st(1)") _,); } o }
        };
    }
    macro_rules! rel {
        ($m:ident, $cc:literal) => {
            #[inline] pub fn $m (self, b : F80) -> bool { let mut r : u8; unsafe {
            asm!("fld tbyte ptr [{a}]", "fld tbyte ptr [{b}]", "fcomip st, st(1)",
            "fstp st(0)", concat!($cc, " {r}"), a = in (reg) & self, b = in (reg) & b, r
            = out(reg_byte) r, out("st(0)") _, out("st(1)") _,); } r != 0 }
        };
    }
    impl F80 {
        bin!(add, "faddp st(1), st");
        bin!(sub, "fsubrp st(1), st");
        bin!(mul, "fmulp st(1), st");
        bin!(div, "fdivrp st(1), st");
        #[inline]
        pub fn neg(self) -> F80 {
            let mut o = F80([0u8; 16]);
            unsafe {
                asm!(
                    "fld tbyte ptr [{a}]", "fchs", "fstp tbyte ptr [{o}]", a = in (reg) &
                    self, o = in (reg) & mut o, out("st(0)") _
                );
            }
            o
        }
        rel!(lt, "seta");
        rel!(le, "setae");
        #[inline]
        pub fn gt(self, b: F80) -> bool {
            b.lt(self)
        }
        #[inline]
        pub fn ge(self, b: F80) -> bool {
            b.le(self)
        }
        #[inline]
        pub fn eq(self, b: F80) -> bool {
            let mut r: u8;
            unsafe {
                asm!(
                    "fld tbyte ptr [{b}]", "fld tbyte ptr [{a}]", "fcomip st, st(1)",
                    "fstp st(0)", "sete al", "setnp dl", "and al, dl", "mov {r}, al", a =
                    in (reg) & self, b = in (reg) & b, r = out(reg_byte) r, out("al") _,
                    out("dl") _, out("st(0)") _, out("st(1)") _,
                );
            }
            r != 0
        }
        #[inline]
        pub fn ne(self, b: F80) -> bool {
            !self.eq(b)
        }
        #[inline]
        pub fn from_f64(x: f64) -> F80 {
            let mut o = F80([0u8; 16]);
            unsafe {
                asm!(
                    "fld qword ptr [{x}]", "fstp tbyte ptr [{o}]", x = in (reg) & x, o =
                    in (reg) & mut o, out("st(0)") _
                );
            }
            o
        }
        #[inline]
        pub fn from_f32(x: f32) -> F80 {
            let mut o = F80([0u8; 16]);
            unsafe {
                asm!(
                    "fld dword ptr [{x}]", "fstp tbyte ptr [{o}]", x = in (reg) & x, o =
                    in (reg) & mut o, out("st(0)") _
                );
            }
            o
        }
        #[inline]
        pub fn to_f64(self) -> f64 {
            let mut o: f64 = 0.0;
            unsafe {
                asm!(
                    "fld tbyte ptr [{a}]", "fstp qword ptr [{o}]", a = in (reg) & self, o
                    = in (reg) & mut o, out("st(0)") _
                );
            }
            o
        }
        #[inline]
        pub fn to_f32(self) -> f32 {
            let mut o: f32 = 0.0;
            unsafe {
                asm!(
                    "fld tbyte ptr [{a}]", "fstp dword ptr [{o}]", a = in (reg) & self, o
                    = in (reg) & mut o, out("st(0)") _
                );
            }
            o
        }
        #[inline]
        pub fn from_i64(x: i64) -> F80 {
            let mut o = F80([0u8; 16]);
            unsafe {
                asm!(
                    "fild qword ptr [{x}]", "fstp tbyte ptr [{o}]", x = in (reg) & x, o =
                    in (reg) & mut o, out("st(0)") _
                );
            }
            o
        }
        #[inline]
        pub fn from_i32(x: i32) -> F80 {
            F80::from_i64(x as i64)
        }
        #[inline]
        pub fn from_u64(x: u64) -> F80 {
            let lo = F80::from_i64(x as i64);
            if (x as i64) < 0 {
                lo.add(F80::from_bits(0x403F_8000_0000_0000_0000u128))
            } else {
                lo
            }
        }
        #[inline]
        pub fn from_u32(x: u32) -> F80 {
            F80::from_i64(x as i64)
        }
        #[inline]
        pub fn to_i64(self) -> i64 {
            let mut o: i64 = 0;
            unsafe {
                asm!(
                    "fld tbyte ptr [{a}]", "fisttp qword ptr [{o}]", a = in (reg) & self,
                    o = in (reg) & mut o, out("st(0)") _
                );
            }
            o
        }
        #[inline]
        pub fn to_i32(self) -> i32 {
            let mut o: i32 = 0;
            unsafe {
                asm!(
                    "fld tbyte ptr [{a}]", "fisttp dword ptr [{o}]", a = in (reg) & self,
                    o = in (reg) & mut o, out("st(0)") _
                );
            }
            o
        }
        #[inline]
        pub fn to_u64(self) -> u64 {
            let thr = F80::from_bits(0x403E_8000_0000_0000_0000u128);
            if self.ge(thr) {
                (self.sub(thr).to_i64() as u64).wrapping_add(0x8000_0000_0000_0000)
            } else {
                self.to_i64() as u64
            }
        }
        #[inline]
        pub fn to_u32(self) -> u32 {
            self.to_i64() as u32
        }
        #[inline]
        pub fn from_bits(b: u128) -> F80 {
            F80(b.to_le_bytes())
        }
        #[inline]
        pub fn to_bits(self) -> u128 {
            u128::from_le_bytes(self.0)
        }
    }
    impl core::ops::Add for F80 {
        type Output = F80;
        #[inline]
        fn add(self, b: F80) -> F80 {
            F80::add(self, b)
        }
    }
    impl core::ops::Sub for F80 {
        type Output = F80;
        #[inline]
        fn sub(self, b: F80) -> F80 {
            F80::sub(self, b)
        }
    }
    impl core::ops::Mul for F80 {
        type Output = F80;
        #[inline]
        fn mul(self, b: F80) -> F80 {
            F80::mul(self, b)
        }
    }
    impl core::ops::Div for F80 {
        type Output = F80;
        #[inline]
        fn div(self, b: F80) -> F80 {
            F80::div(self, b)
        }
    }
    impl core::ops::Neg for F80 {
        type Output = F80;
        #[inline]
        fn neg(self) -> F80 {
            F80::neg(self)
        }
    }
}

pub mod __cxx_exc {
    use std::any::Any;
    use std::cell::Cell;
    use std::sync::Once;
    thread_local! {
        static IN_THROW : Cell < bool > = const { Cell::new(false) };
    }
    fn ensure_silent_hook() {
        static ONCE: Once = Once::new();
        ONCE.call_once(|| {
            let default = std::panic::take_hook();
            std::panic::set_hook(
                Box::new(move |info| {
                    if IN_THROW.with(|f| f.replace(false)) {
                        return;
                    }
                    default(info);
                }),
            );
        });
    }
    /// Raise a C++ exception: preserve the thrown value as a downcastable panic
    /// payload (so a type-matched `catch` can recover it), but suppress the
    /// default panic message so a CAUGHT exception is silent, exactly as in
    /// C++. A genuine Rust panic is unaffected.
    #[inline]
    pub fn throw<T: Any + Send>(payload: T) -> ! {
        ensure_silent_hook();
        IN_THROW.with(|f| f.set(true));
        std::panic::panic_any(payload)
    }
}

pub mod __common;
pub mod full_ops_0;
pub mod ACE_1;
pub mod ACE_crc32_2;
pub mod ACE_crc_ccitt_3;
pub mod ATM_Acceptor_4;
pub mod ATM_Addr_5;
pub mod ATM_Connector_6;
pub mod ATM_Params_7;
pub mod ATM_QoS_8;
pub mod ATM_Stream_9;
pub mod Acceptor_10;
pub mod Activation_Queue_11;
pub mod Active_Map_Manager_12;
pub mod Active_Map_Manager_T_13;
pub mod Addr_14;
pub mod Arg_Shifter_15;
pub mod Argv_Type_Converter_16;
pub mod Array_Base_17;
pub mod Array_Map_18;
pub mod Assert_19;
pub mod Asynch_Acceptor_20;
pub mod Asynch_Connector_21;
pub mod Asynch_IO_22;
pub mod Asynch_IO_Impl_23;
pub mod Asynch_Pseudo_Task_24;
pub mod Atomic_Op_25;
pub mod Atomic_Op_T_26;
pub mod Auto_Functor_27;
pub mod Auto_IncDec_T_28;
pub mod Auto_Ptr_29;
pub mod Barrier_30;
pub mod Base_Thread_Adapter_31;
pub mod Based_Pointer_Repository_32;
pub mod Based_Pointer_T_33;
pub mod Basic_Stats_34;
pub mod Basic_Types_35;
pub mod Bound_Ptr_36;
pub mod CDR_Base_37;
pub mod CDR_Size_38;
pub mod CDR_Stream_39;
pub mod CE_Screen_Output_40;
pub mod Cached_Connect_Strategy_T_41;
pub mod Caching_Strategies_T_42;
pub mod Caching_Utility_T_43;
pub mod Capabilities_44;
pub mod Cleanup_45;
pub mod Cleanup_Strategies_T_46;
pub mod Codecs_47;
pub mod Codeset_IBM1047_48;
pub mod Codeset_Registry_49;
pub mod Codeset_Registry_db_50;
pub mod Condition_Attributes_51;
pub mod Condition_Recursive_Thread_Mutex_52;
pub mod Condition_T_53;
pub mod Condition_Thread_Mutex_54;
pub mod Configuration_55;
pub mod Configuration_Import_Export_56;
pub mod Connection_Recycling_Strategy_57;
pub mod Connector_58;
pub mod Containers_59;
pub mod Containers_T_60;
pub mod Copy_Disabled_61;
pub mod Countdown_Time_T_62;
pub mod DEV_63;
pub mod DEV_Addr_64;
pub mod DEV_Connector_65;
pub mod DEV_IO_66;
pub mod DLL_67;
pub mod DLL_Manager_68;
pub mod Date_Time_69;
pub mod Dev_Poll_Reactor_70;
pub mod Dirent_71;
pub mod Dirent_Selector_72;
pub mod Dump_73;
pub mod Dump_T_74;
pub mod Dynamic_75;
pub mod Dynamic_Message_Strategy_76;
pub mod Dynamic_Service_77;
pub mod Dynamic_Service_Base_78;
pub mod Dynamic_Service_Dependency_79;
pub mod Encoding_Converter_80;
pub mod Encoding_Converter_Factory_81;
pub mod Env_Value_T_82;
pub mod Event_Base_83;
pub mod Event_Handler_84;
pub mod Event_Handler_Handle_Timeout_Upcall_85;
pub mod Event_Handler_T_86;
pub mod FIFO_87;
pub mod FIFO_Recv_88;
pub mod FIFO_Recv_Msg_89;
pub mod FIFO_Send_90;
pub mod FIFO_Send_Msg_91;
pub mod FILE_92;
pub mod FILE_Addr_93;
pub mod FILE_Connector_94;
pub mod FILE_IO_95;
pub mod File_Lock_96;
pub mod Filecache_97;
pub mod Flag_Manip_98;
pub mod Framework_Component_99;
pub mod Framework_Component_T_100;
pub mod Free_List_101;
pub mod Functor_102;
pub mod Functor_String_103;
pub mod Functor_T_104;
pub mod Future_105;
pub mod Future_Set_106;
pub mod Get_Opt_107;
pub mod Guard_T_108;
pub mod Handle_Ops_109;
pub mod Handle_Set_110;
pub mod Hash_Cache_Map_Manager_T_111;
pub mod Hash_Map_Manager_T_112;
pub mod Hash_Map_With_Allocator_T_113;
pub mod Hash_Multi_Map_Manager_T_114;
pub mod Hashable_115;
pub mod High_Res_Timer_116;
pub mod ICMP_Socket_117;
pub mod INET_Addr_118;
pub mod IOStream_119;
pub mod IOStream_T_120;
pub mod IO_Cntl_Msg_121;
pub mod IO_SAP_122;
pub mod IPC_SAP_123;
pub mod Init_ACE_124;
pub mod Intrusive_Auto_Ptr_125;
pub mod Intrusive_List_126;
pub mod Intrusive_List_Node_127;
pub mod LOCK_SOCK_Acceptor_128;
pub mod LSOCK_129;
pub mod LSOCK_Acceptor_130;
pub mod LSOCK_CODgram_131;
pub mod LSOCK_Connector_132;
pub mod LSOCK_Dgram_133;
pub mod LSOCK_Stream_134;
pub mod Lib_Find_135;
pub mod Local_Memory_Pool_136;
pub mod Local_Name_Space_137;
pub mod Local_Name_Space_T_138;
pub mod Local_Tokens_139;
pub mod Lock_140;
pub mod Lock_Adapter_T_141;
pub mod Log_Category_142;
pub mod Log_Msg_143;
pub mod Log_Msg_Android_Logcat_144;
pub mod Log_Msg_Backend_145;
pub mod Log_Msg_Callback_146;
pub mod Log_Msg_IPC_147;
pub mod Log_Msg_NT_Event_Log_148;
pub mod Log_Msg_UNIX_Syslog_149;
pub mod Log_Record_150;
pub mod Logging_Strategy_151;
pub mod MEM_Acceptor_152;
pub mod MEM_Addr_153;
pub mod MEM_Connector_154;
pub mod MEM_IO_155;
pub mod MEM_SAP_156;
pub mod MEM_Stream_157;
pub mod MMAP_Memory_Pool_158;
pub mod MQX_Filesystem_159;
pub mod Malloc_160;
pub mod Malloc_Allocator_161;
pub mod Malloc_T_162;
pub mod Managed_Object_163;
pub mod Map_Manager_164;
pub mod Map_T_165;
pub mod Mem_Map_166;
pub mod Message_Block_167;
pub mod Message_Queue_168;
pub mod Message_Queue_NT_169;
pub mod Message_Queue_T_170;
pub mod Message_Queue_Vx_171;
pub mod Method_Request_172;
pub mod Metrics_Cache_T_173;
pub mod Module_174;
pub mod Monitor_Admin_175;
pub mod Monitor_Admin_Manager_176;
pub mod Monitor_Base_177;
pub mod Monitor_Control_Action_178;
pub mod Monitor_Control_Types_179;
pub mod Monitor_Point_Registry_180;
pub mod Monitor_Size_181;
pub mod Monotonic_Time_Policy_182;
pub mod Msg_WFMO_Reactor_183;
pub mod Multihomed_INET_Addr_184;
pub mod Mutex_185;
pub mod NT_Service_186;
pub mod Name_Proxy_187;
pub mod Name_Request_Reply_188;
pub mod Name_Space_189;
pub mod Naming_Context_190;
pub mod Netlink_Addr_191;
pub mod Node_192;
pub mod Notification_Queue_193;
pub mod Notification_Strategy_194;
pub mod Null_Mutex_195;
pub mod OS_Errno_196;
pub mod OS_Log_Msg_Attributes_197;
pub mod OS_NS_Thread_198;
pub mod OS_NS_arpa_inet_199;
pub mod OS_NS_ctype_200;
pub mod OS_NS_devctl_201;
pub mod OS_NS_dirent_202;
pub mod OS_NS_dlfcn_203;
pub mod OS_NS_errno_204;
pub mod OS_NS_fcntl_205;
pub mod OS_NS_math_206;
pub mod OS_NS_netdb_207;
pub mod OS_NS_poll_208;
pub mod OS_NS_pwd_209;
pub mod OS_NS_regex_210;
pub mod OS_NS_signal_211;
pub mod OS_NS_stdio_212;
pub mod OS_NS_stdlib_213;
pub mod OS_NS_string_214;
pub mod OS_NS_strings_215;
pub mod OS_NS_stropts_216;
pub mod OS_NS_sys_mman_217;
pub mod OS_NS_sys_msg_218;
pub mod OS_NS_sys_resource_219;
pub mod OS_NS_sys_select_220;
pub mod OS_NS_sys_sendfile_221;
pub mod OS_NS_sys_shm_222;
pub mod OS_NS_sys_socket_223;
pub mod OS_NS_sys_stat_224;
pub mod OS_NS_sys_time_225;
pub mod OS_NS_sys_uio_226;
pub mod OS_NS_sys_utsname_227;
pub mod OS_NS_sys_wait_228;
pub mod OS_NS_time_229;
pub mod OS_NS_unistd_230;
pub mod OS_NS_wchar_231;
pub mod OS_NS_wctype_232;
pub mod OS_QoS_233;
pub mod OS_TLI_234;
pub mod OS_Thread_Adapter_235;
pub mod OS_main_236;
pub mod Obchunk_237;
pub mod Object_Manager_238;
pub mod Object_Manager_Base_239;
pub mod Obstack_240;
pub mod Obstack_T_241;
pub mod PI_Malloc_242;
pub mod POSIX_Asynch_IO_243;
pub mod POSIX_CB_Proactor_244;
pub mod POSIX_Proactor_245;
pub mod Pagefile_Memory_Pool_246;
pub mod Pair_T_247;
pub mod Parse_Node_248;
pub mod Ping_Socket_249;
pub mod Pipe_250;
pub mod Priority_Reactor_251;
pub mod Proactor_252;
pub mod Proactor_Impl_253;
pub mod Process_254;
pub mod Process_Manager_255;
pub mod Process_Mutex_256;
pub mod Process_Semaphore_257;
pub mod Profile_Timer_258;
pub mod RB_Tree_259;
pub mod RW_Mutex_260;
pub mod RW_Process_Mutex_261;
pub mod RW_Thread_Mutex_262;
pub mod Reactor_263;
pub mod Reactor_Impl_264;
pub mod Reactor_Notification_Strategy_265;
pub mod Reactor_Timer_Interface_266;
pub mod Read_Buffer_267;
pub mod Recursive_Thread_Mutex_268;
pub mod Recyclable_269;
pub mod Refcountable_T_270;
pub mod Refcounted_Auto_Ptr_271;
pub mod Registry_272;
pub mod Registry_Name_Space_273;
pub mod Remote_Name_Space_274;
pub mod Remote_Tokens_275;
pub mod Reverse_Lock_T_276;
pub mod SOCK_277;
pub mod SOCK_Acceptor_278;
pub mod SOCK_CODgram_279;
pub mod SOCK_Connector_280;
pub mod SOCK_Dgram_281;
pub mod SOCK_Dgram_Bcast_282;
pub mod SOCK_Dgram_Mcast_283;
pub mod SOCK_IO_284;
pub mod SOCK_Netlink_285;
pub mod SOCK_SEQPACK_Acceptor_286;
pub mod SOCK_SEQPACK_Association_287;
pub mod SOCK_SEQPACK_Connector_288;
pub mod SOCK_Stream_289;
pub mod SPIPE_290;
pub mod SPIPE_Acceptor_291;
pub mod SPIPE_Addr_292;
pub mod SPIPE_Connector_293;
pub mod SPIPE_Stream_294;
pub mod SString_295;
pub mod SUN_Proactor_296;
pub mod SV_Message_297;
pub mod SV_Message_Queue_298;
pub mod SV_Semaphore_Complex_299;
pub mod SV_Semaphore_Simple_300;
pub mod SV_Shared_Memory_301;
pub mod Sample_History_302;
pub mod Sbrk_Memory_Pool_303;
pub mod Sched_Params_304;
pub mod Select_Reactor_Base_305;
pub mod Select_Reactor_T_306;
pub mod Semaphore_307;
pub mod Service_Config_308;
pub mod Service_Gestalt_309;
pub mod Service_Manager_310;
pub mod Service_Object_311;
pub mod Service_Repository_312;
pub mod Service_Types_313;
pub mod Shared_Memory_314;
pub mod Shared_Memory_MM_315;
pub mod Shared_Memory_Pool_316;
pub mod Shared_Memory_SV_317;
pub mod Shared_Object_318;
pub mod Sig_Adapter_319;
pub mod Sig_Handler_320;
pub mod Signal_321;
pub mod Singleton_322;
pub mod Sock_Connect_323;
pub mod Stack_Trace_324;
pub mod Stats_325;
pub mod Strategies_T_326;
pub mod Stream_327;
pub mod Stream_Modules_328;
pub mod String_Base_329;
pub mod String_Base_Const_330;
pub mod Svc_Conf_Lexer_331;
pub mod Svc_Conf_y_332;
pub mod Svc_Handler_333;
pub mod Synch_Options_334;
pub mod System_Time_335;
pub mod TLI_336;
pub mod TLI_Acceptor_337;
pub mod TLI_Connector_338;
pub mod TLI_Stream_339;
pub mod TP_Reactor_340;
pub mod TSS_Adapter_341;
pub mod TSS_T_342;
pub mod TTY_IO_343;
pub mod Task_344;
pub mod Task_Ex_T_345;
pub mod Task_T_346;
pub mod Test_and_Set_347;
pub mod Thread_348;
pub mod Thread_Adapter_349;
pub mod Thread_Control_350;
pub mod Thread_Exit_351;
pub mod Thread_Hook_352;
pub mod Thread_Manager_353;
pub mod Thread_Mutex_354;
pub mod Thread_Semaphore_355;
pub mod Throughput_Stats_356;
pub mod Time_Policy_357;
pub mod Time_Policy_T_358;
pub mod Time_Value_359;
pub mod Time_Value_T_360;
pub mod Timeprobe_361;
pub mod Timeprobe_T_362;
pub mod Timer_Hash_T_363;
pub mod Timer_Heap_T_364;
pub mod Timer_List_T_365;
pub mod Timer_Queue_Adapters_366;
pub mod Timer_Queue_T_367;
pub mod Timer_Wheel_T_368;
pub mod Token_369;
pub mod Token_Collection_370;
pub mod Token_Invariants_371;
pub mod Token_Manager_372;
pub mod Token_Request_Reply_373;
pub mod Trace_374;
pub mod Typed_SV_Message_375;
pub mod Typed_SV_Message_Queue_376;
pub mod UNIX_Addr_377;
pub mod UPIPE_Acceptor_378;
pub mod UPIPE_Connector_379;
pub mod UPIPE_Stream_380;
pub mod UTF16_Encoding_Converter_381;
pub mod UTF32_Encoding_Converter_382;
pub mod UTF8_Encoding_Converter_383;
pub mod UUID_384;
pub mod Unbounded_Queue_385;
pub mod Unbounded_Set_386;
pub mod Unbounded_Set_Ex_387;
pub mod Vector_T_388;
pub mod WFMO_Reactor_389;
pub mod WIN32_Asynch_IO_390;
pub mod WIN32_Proactor_391;
pub mod XML_Svc_Conf_392;
pub mod XTI_ATM_Mcast_393;
pub mod ace_wchar_394;
