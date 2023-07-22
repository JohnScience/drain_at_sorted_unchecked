use core::{num::NonZeroUsize, ops::RangeInclusive};

/// Convenience standalone function for [`DrainAtSortedUnchecked::drain_at_sorted_unchecked`].
/// 
/// # Safety
/// 
/// * The indices must be sorted in ascending order.
/// * The indices must be within bounds of the collection.
/// * The indices must be unique.
/// * The items must be trivially [movable].
/// 
/// [movable]: https://doc.rust-lang.org/std/pin/#:~:text=By%20default%2C%20all%20types%20in,can%20use%20mem%3A%3Aswap%20.
pub unsafe fn drain_at_sorted_unchecked<C,I>(collection: &mut C, into_iter: I)
where
    C: DrainAtSortedUnchecked,
    I: IntoIterator<Item = usize>,
{
    collection.drain_at_sorted_unchecked(into_iter);
}

/// The trait-extension that provides the `drain_at_sorted_unchecked` method
/// similar to [`drain`] or [`drain_filter`].
/// 
/// [`drain`]: https://doc.rust-lang.org/std/vec/struct.Vec.html#method.drain
/// [`drain_filter`]: https://doc.rust-lang.org/std/vec/struct.Vec.html#method.drain_filter
pub trait DrainAtSortedUnchecked {
    /// Removes the items at the given indices.
    /// 
    /// # Safety
    /// 
    /// * The indices must be sorted in ascending order.
    /// * The indices must be within bounds of the collection.
    /// * The indices must be unique.
    /// * The items must be trivially [movable].
    /// 
    /// [movable]: https://doc.rust-lang.org/std/pin/#:~:text=By%20default%2C%20all%20types%20in,can%20use%20mem%3A%3Aswap%20.
    unsafe fn drain_at_sorted_unchecked<I>(&mut self, into_iter: I)
    where
        I: IntoIterator<Item = usize>;
}

impl<T> DrainAtSortedUnchecked for Vec<T> {
    unsafe fn drain_at_sorted_unchecked<I>(&mut self, into_iter: I)
    where
        I: IntoIterator<Item = usize>,
    {
        // The iterator that yields the indices of the items to be removed.
        let mut rem_idx_iter: <I as IntoIterator>::IntoIter = into_iter.into_iter();
        let last_idx: usize = self.len() - 1;

        // The first consecutive index of the items to be removed.
        let Some(mut fst_consec_rem_idx) = rem_idx_iter.next() else {
            return;
        };
        // The last consecutive index of the items to be removed.
        let mut last_consec_rem_idx: usize = fst_consec_rem_idx;
        // Non-consecutive index of the items to be removed.
        let mut non_consec_rem_idx_opt: Option<NonZeroUsize> = None;
        // A while-loop is used instead of a for-loop in order to avoid
        // moving the iterator.
        while let Some(rem_idx) = rem_idx_iter.next() {
            if rem_idx == last_consec_rem_idx + 1 {
                last_consec_rem_idx = rem_idx;
            } else {
                // Since the indices are sorted and the first index is handled
                // before the loop, the non-consecutive index must be greater than 0.
                non_consec_rem_idx_opt = Some(NonZeroUsize::new_unchecked(rem_idx));
                break;
            }
        }
        // The range of the consecutive indices of the items to be removed.
        let consec_rem_range: RangeInclusive<usize> = fst_consec_rem_idx..=last_consec_rem_idx;
        // The length of the range of the consecutive indices of the items to be removed.
        let consec_rem_range_len: usize = last_consec_rem_idx - fst_consec_rem_idx + 1;

        // We don't use drain here because it would shift the indices of the items.
        for rem_ptr in consec_rem_range.map(|i| self.as_mut_ptr().add(i)) {
            // Some types have custom drop implementations. Therefore, we need to
            // drop the items before overwriting them with "replacement items" or "forgetting"
            // them by setting the length of the vector manually.
            core::ptr::drop_in_place(rem_ptr);
        }

        // The number of items that have been removed.
        let mut items_remd: usize = consec_rem_range_len;

        let mut non_consec_rem_idx: usize = match (non_consec_rem_idx_opt, last_consec_rem_idx) {
            (Some(non_consec_rem_idx), _) => NonZeroUsize::get(non_consec_rem_idx),
            (None, last_consec_rem_idx) if last_consec_rem_idx == last_idx => {
                self.set_len(self.len() - items_remd);
                return;
            },
            (None, _) => {
                // This index won't be accessed, so it's safe.
                self.len()
            }
        };

        // The end of the replacement item indices.
        let rmnt_end: usize = non_consec_rem_idx - 1;
        // The start of the replacement item indices.
        let rmnt_begin: usize = last_consec_rem_idx + 1;
        // The range of the replacement item indices.
        let rmnt_range_len: usize = rmnt_end - rmnt_begin + 1;
        // The pointer to the first [consecutive] replacement item.
        let rmnt_ptr: *const T = self.as_ptr().add(rmnt_begin);

        // The pointer to the first consecutive item to be removed.
        let fst_consec_rem_ptr: *mut T = self.as_mut_ptr().add(fst_consec_rem_idx);

        // NOTE: after shifting the [consecutive] replacement items,
        // there will be a "gap".
        core::ptr::copy(rmnt_ptr, fst_consec_rem_ptr, rmnt_range_len);

        if non_consec_rem_idx == self.len() {
            self.set_len(self.len() - items_remd);
            return;
        }
        
        loop {
            [fst_consec_rem_idx, last_consec_rem_idx] = [non_consec_rem_idx; 2];
            non_consec_rem_idx_opt = None;
            while let Some(rem_idx) = rem_idx_iter.next() {
                if rem_idx == last_consec_rem_idx + 1 {
                    last_consec_rem_idx = rem_idx;
                } else {
                    // Since the indices are sorted and the first index is handled
                    // before the loop, the non-consecutive index must be greater than 0.
                    non_consec_rem_idx_opt = Some(NonZeroUsize::new_unchecked(rem_idx));
                    break;
                }
            }

            // The range of the consecutive indices of the items to be removed.
            let consec_rem_range: RangeInclusive<usize> = fst_consec_rem_idx..=last_consec_rem_idx;
            // The length of the range of the consecutive indices of the items to be removed.
            let consec_rem_range_len: usize = last_consec_rem_idx - fst_consec_rem_idx + 1;

            // We don't use drain here because it would shift the indices of the items.
            for rem_idx in consec_rem_range.map(|i| self.as_mut_ptr().add(i)) {
                // Some types have custom drop implementations. Therefore, we need to
                // drop the items before overwriting them with "replacement items" or "forgetting"
                // them by setting the length of the vector manually.
                core::ptr::drop_in_place(rem_idx);
            }

            items_remd += consec_rem_range_len;

            non_consec_rem_idx = match (non_consec_rem_idx_opt, last_consec_rem_idx) {
                (Some(non_consec_rem_idx), _) => NonZeroUsize::get(non_consec_rem_idx),
                (None, last_consec_rem_idx) if last_consec_rem_idx == last_idx => {
                    self.set_len(self.len() - items_remd);
                    return;
                },
                (None, _) => {
                    last_idx + 1
                }
            };

            // The start of the replacement item indices.
            let rmnt_begin: usize = last_consec_rem_idx + 1;

            // The destination index of the replacement items.
            let rmnt_dst_idx: usize = rmnt_begin - items_remd;
            // The pointer to the destination of the replacement items.
            let rmnt_dst_ptr: *mut T = self.as_mut_ptr().add(rmnt_dst_idx);

            // The end of the replacement item indices.
            let rmnt_end: usize = match non_consec_rem_idx_opt {
                Some(idx) => NonZeroUsize::get(idx) - 1,
                None => last_idx,
            };
            // The range of the replacement item indices.
            let rmnt_range_len: usize = rmnt_end + 1 - rmnt_begin;
            // The pointer to the first [consecutive] replacement item.
            let rmnt_ptr: *const T = self.as_ptr().add(rmnt_begin);

            // NOTE: after shifting the [consecutive] replacement items,
            // there will be a "gap".
            core::ptr::copy(rmnt_ptr, rmnt_dst_ptr, rmnt_range_len);

            if non_consec_rem_idx == self.len() {
                self.set_len(self.len() - items_remd);
                return;
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn rem_1_2_3_5_7() {
        let mut v = vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9];
        unsafe { v.drain_at_sorted_unchecked(vec![1, 2, 3, 5, 7]) };
        assert_eq!(v, vec![0, 4, 6, 8, 9]);
    }

    #[test]
    fn rem_0th() {
        let mut v = vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9];
        unsafe { v.drain_at_sorted_unchecked(vec![0]) };
        assert_eq!(v, vec![1, 2, 3, 4, 5, 6, 7, 8, 9]);
    }

    #[test]
    fn rem_0_2_4_6_8_9() {
        let mut v = vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9];
        unsafe { v.drain_at_sorted_unchecked(vec![1, 3, 5, 7]) };
        assert_eq!(v, vec![0, 2, 4, 6, 8, 9]);
    }

    #[test]
    fn rem_last() {
        let mut v = vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9];
        unsafe { v.drain_at_sorted_unchecked(vec![9]) };
        assert_eq!(v, vec![0, 1, 2, 3, 4, 5, 6, 7, 8]);
    }

    #[test]
    fn rem_last_two() {
        let mut v = vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9];
        unsafe { v.drain_at_sorted_unchecked(vec![8, 9]) };
        assert_eq!(v, vec![0, 1, 2, 3, 4, 5, 6, 7]);
    }

    #[test]
    fn rem_middle() {
        let mut v = vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9];
        unsafe { v.drain_at_sorted_unchecked(vec![4]) };
        assert_eq!(v, vec![0, 1, 2, 3, 5, 6, 7, 8, 9]);
    }

    #[test]
    fn rem_multiple_non_consecutive() {
        let mut v = vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9];
        unsafe { v.drain_at_sorted_unchecked(vec![0, 3, 5, 8]) };
        assert_eq!(v, vec![1, 2, 4, 6, 7, 9]);
    }

    #[test]
    fn rem_empty() {
        let mut v = vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9];
        unsafe { v.drain_at_sorted_unchecked(Vec::new()) };
        assert_eq!(v, vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9]);
    }

    #[test]
    fn rem_single_elem_vector() {
        let mut v = vec![0];
        unsafe { v.drain_at_sorted_unchecked(vec![0]) };
        assert_eq!(v, Vec::<i32>::new());
    }

    #[test]
    fn rem_consecutive_elements() {
        let mut v = vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9];
        unsafe { v.drain_at_sorted_unchecked(vec![1, 2, 3, 4]) };
        assert_eq!(v, vec![0, 5, 6, 7, 8, 9]);
    }

    #[test]
    fn rem_consecutive_elements_and_last() {
        let mut v = vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9];
        unsafe { v.drain_at_sorted_unchecked(vec![1, 2, 3, 4, 9]) };
        assert_eq!(v, vec![0, 5, 6, 7, 8]);
    }
}
