use core::mem;
use core::ptr::copy_nonoverlapping;

use alloc::alloc::Layout;

use crate::allocator::linked_list::IterMut;
use crate::allocator::linked_list::LinkedList;
use crate::allocator::linked_list::Node;
use crate::allocator::util::*;

/// A simple allocator that allocates based on size classes.
pub struct Allocator {
    start: usize,
    end: usize,
    num_bins: usize,
    bins: *mut LinkedList,
}

fn k_value(layout: &Layout) -> usize {
    let power = (layout.size() as f32).log2().ceil() as usize;
    k_value_usize(power)
}

fn k_value_usize(power: usize) -> usize {
    if power < 3 {
        0
    } else {
        power - 3
    }
}

fn bin_size_fn(k: usize) -> usize {
    1 << (k + 3)
}

fn k_value_from_free_space(free_space: usize) -> usize {
    let power = (free_space as f32).log2().floor() as usize;
    k_value_usize(power)
}

static SIZE_OF_LINKED_LIST: usize = mem::size_of::<LinkedList>();

impl Allocator {
    /// Creates a new bin allocator that will allocate memory from the region
    /// starting at address `start` and ending at address `end`.
    pub fn new(start: usize, end: usize) -> Allocator {
        // let size = end - start;
        // let exponent = (size as f32).log2().floor() as usize;
        let bin_prototype = &mut LinkedList::new();
        let mut new_start = start;
        let mut num_bins = 0;
        let mut remaining = end - new_start;
        // let mut bin_size = bin_size_fn(num_bins);

        while {
            unsafe {
                copy_nonoverlapping(
                    bin_prototype as *const LinkedList,
                    new_start as *mut LinkedList,
                    1,
                );
            }

            new_start += SIZE_OF_LINKED_LIST;
            remaining -= SIZE_OF_LINKED_LIST;
            num_bins += 1;
            let bin_size = bin_size_fn(num_bins);
            remaining > bin_size
        } {}

        // let mut k = num_bins - 1;
        // while remaining > 0 && k >= 0 {
        //     let bin_size = bin_size_fn(k);
        //     if bin_size <= remaining {
        //       unsafe {
        //         let bin = &mut (*((start + (k * SIZE_OF_LINKED_LIST)) as *mut LinkedList));

        //         bin.push(new_start as *mut usize);
        //       }

        //       new_start += bin_size;
        //       remaining = end - new_start;
        //     } else {
        //         k -= 1;
        //     }
        // }

        Allocator {
            start: new_start,
            end,
            num_bins,
            bins: start as *mut LinkedList,
        }
    }

    fn get_bin(&self, k: usize) -> &mut LinkedList {
        let bin: &mut LinkedList;
        unsafe {
            bin = &mut *(((self.bins as usize) + (k * SIZE_OF_LINKED_LIST)) as *mut LinkedList);
        }
        return bin;
    }

    /// Allocates memory. Returns a pointer meeting the size and alignment
    /// properties of `layout.size()` and `layout.align()`.
    ///
    /// If this method returns an `Ok(addr)`, `addr` will be non-null address
    /// pointing to a block of storage suitable for holding an instance of
    /// `layout`. In particular, the block will be at least `layout.size()`
    /// bytes large and will be aligned to `layout.align()`. The returned block
    /// of storage may or may not have its contents initialized or zeroed.
    ///
    /// # Safety
    ///
    /// The _caller_ must ensure that `layout.size() > 0` and that
    /// `layout.align()` is a power of two. Parameters not meeting these
    /// conditions may result in undefined behavior.
    ///
    /// # Errors
    ///
    /// Returning `Err` indicates that either memory is exhausted
    /// (`AllocError::Exhausted`) or `layout` does not meet this allocator's
    /// size or alignment constraints (`AllocError::Unsupported`).
    pub fn alloc(&mut self, layout: Layout) -> *mut u8 {
        let initial_k = k_value(&layout);
        let mut k = initial_k;

        while k < self.num_bins {
            let bin_size = bin_size_fn(k);
            let bin: &mut LinkedList = self.get_bin(k);
            // let mut itr = bin.iter_mut();
            // let result = self.alloc_from_back_of_bin(k, bin_size, itr.next(), &mut itr, &layout);
            let result = self.alloc_from_bin(k, bin_size, bin, &layout);
            if result != (0 as *mut u8) {
                return result;
            }

            k += 1;
        }

        let aligned_addr = align_up(self.start, layout.align());
        let bin_size = bin_size_fn(initial_k);
        let addr_end = aligned_addr + bin_size;
        if addr_end <= self.end {
            let left = self.start;
            self.bin_free_space(left, aligned_addr);

            self.start = addr_end;
            return aligned_addr as *mut u8;
        } else {
            return 0 as *mut u8;
        }
    }

    fn alloc_from_bin(
        &self,
        k: usize,
        bin_size: usize,
        bin: &mut LinkedList,
        layout: &Layout,
    ) -> *mut u8 {
        for node in bin.iter_mut() {
            let result = self.alloc_from_node(k, bin_size, node, layout);
            if result != (0 as *mut u8) {
                return result;
            }
        }

        return 0 as *mut u8;
    }

    fn alloc_from_back_of_bin(
        &self,
        k: usize,
        bin_size: usize,
        node: Option<Node>,
        itr: &mut IterMut,
        layout: &Layout,
    ) -> *mut u8 {
        if node.is_none() {
            return 0 as *mut u8;
        } else {
            let result = self.alloc_from_back_of_bin(k, bin_size, itr.next(), itr, layout);
            if result != (0 as *mut u8) {
                return result;
            } else {
                return self.alloc_from_node(k, bin_size, node.unwrap(), layout);
            }
        }
    }

    fn alloc_from_node(&self, _k: usize, bin_size: usize, node: Node, layout: &Layout) -> *mut u8 {
        let node_addr = node.value() as usize;
        let bin_end = node_addr + bin_size;
        let alignment = node_addr % layout.align();
        if alignment == 0 {
            let ptr = node.pop();
            let min_k = k_value(layout);
            let min_bin_size = bin_size_fn(min_k);
            let request_end = node_addr + min_bin_size;
            self.bin_free_space(request_end, bin_end);

            return ptr as *mut u8;
        } else {
            let aligned_addr = align_up(node_addr, layout.align());
            let min_k = k_value(layout);
            let min_bin_size = bin_size_fn(min_k);
            let request_end = aligned_addr + min_bin_size;
            if request_end <= bin_end {
                let ptr = node.pop();
                self.bin_free_space(node_addr, aligned_addr);

                self.bin_free_space(request_end, bin_end);
                return ptr as *mut u8;
            } else {
                return 0 as *mut u8;
            }
        }
    }

    fn bin_free_space(&self, mut left: usize, right: usize) {
        let mut free_space = right - left;
        while free_space > 0 {
            let k = k_value_from_free_space(free_space);
            let bin: &mut LinkedList = self.get_bin(k);
            unsafe {
                bin.push(left as *mut usize);
            }

            let bin_size = bin_size_fn(k);
            left += bin_size;
            free_space -= bin_size;
        }
    }

    /// Deallocates the memory referenced by `ptr`.
    ///
    /// # Safety
    ///
    /// The _caller_ must ensure the following:
    ///
    ///   * `ptr` must denote a block of memory currently allocated via this
    ///     allocator
    ///   * `layout` must properly represent the original layout used in the
    ///     allocation call that returned `ptr`
    ///
    /// Parameters not meeting these conditions may result in undefined
    /// behavior.
    pub fn dealloc(&mut self, ptr: *mut u8, layout: Layout) {
        let k = k_value(&layout);
        let bin: &mut LinkedList = self.get_bin(k);
        unsafe { bin.push(ptr as *mut usize) }

        // let location = ptr as usize;
        // for k in 0..self.num_bins {
        //     unsafe {
        //        bin = &mut (*(((self.bins as usize) + (k * SIZE_OF_LINKED_LIST)) as *mut LinkedList));
        //     }

        //     let bin_size = bin_size(k)
        //     while !bin.is_empty() && (bin.peek().unwrap() as usize + bin_size) == self.start {
        //         let removed = bin.pop().unwrap();
        //     }
        // }
    }
}
//
// FIXME: Implement `Debug` for `Allocator`.
