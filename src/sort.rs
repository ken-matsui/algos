mod radix;

#[cfg(test)]
pub(crate) fn is_sorted<T>(data: Vec<T>) -> bool
where
    T: Ord,
{
    data.windows(2).all(|w| w[0] <= w[1])
}
