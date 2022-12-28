use std::collections::HashMap;

/// `Palindrome` is a newtype which only exists when the contained value is a palindrome number in base ten.
///
/// A struct with a single field which is used to constrain behavior like this is called a "newtype", and its use is
/// often referred to as the "newtype pattern". This is a fairly common pattern in Rust.

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct Palindrome(u64);

struct ProductRange {
    min: u64,
    max: u64,
    last_min: Option<u64>,
    last_max: Option<u64>,
    data: HashMap<u64, u64>,
}

impl ProductRange {
    pub fn new(min: u64, max: u64) -> ProductRange {
        ProductRange { 
            min, 
            max, 
            last_min: None,
            last_max: None,
            data: HashMap::from_iter((min..=max).map(|i| (i, i * i) )) }
    }

    fn find_keys(&self, value: u64) -> Vec<u64> {
        self.data.iter()
            .filter_map(|(k, &v)| if v == value {Some(k)} else {None} )
            .cloned()
            .collect()
    }
}

impl DoubleEndedIterator for ProductRange {
    fn next_back(&mut self) -> Option<Self::Item> {
        if self.data.is_empty() {
            return None;
        }

        let (_, &max_val) = self.data.iter().max_by_key(|(_k, &v)| v).unwrap();
        let to_update: Vec<u64> = self.find_keys(max_val);

        for k in to_update {
            let v = self.data.get_mut(&k).unwrap();
            *v -= k;
        }

        self.data.retain(|&k, &mut v| v >= k * self.min);
        if let Some(lm) = self.last_min {
            self.data.retain(|_, &mut v| v > lm);
        }

        self.last_max = Some(max_val);

        Some(max_val)
    }
}

impl Iterator for ProductRange {
    type Item = u64;
    fn next(&mut self) -> Option<Self::Item> {
        if self.data.is_empty() {
            return None;
        }

        let (_, &min_val) = self.data.iter().min_by_key(|(_k, &v)| v).unwrap();

        let to_update: Vec<u64> = self.find_keys(min_val);
        for k in to_update {
            let v = self.data.get_mut(&k).unwrap();
            *v += k;
        }

        self.data.retain(|&k, &mut v| v <= k * self.max);
        if let Some(lm) = self.last_max {
            self.data.retain(|_, &mut v| v < lm);
        }

        self.last_min = Some(min_val);

        Some(min_val)
    }
}

impl Palindrome {
    /// Create a `Palindrome` only if `value` is in fact a palindrome when represented in base ten. Otherwise, `None`.
    pub fn new(value: u64) -> Option<Palindrome> {
        if value < 10 {
            return Some(Palindrome(value));
        }

        let p1 = value.to_string();
        let p2 = p1.chars().rev().collect::<String>();

        match p1 == p2 {
            true => Some(Palindrome(value)),
            false => None
        }
    }

    /// Get the value of this palindrome.
    pub fn into_inner(self) -> u64 {
        self.0
    }
}

pub fn palindrome_products(min: u64, max: u64) -> Option<(Palindrome, Palindrome)> {
    // 
    let min_pal = ProductRange::new(min, max)
        .find_map(|i| Palindrome::new(i));

    let max_pal = ProductRange::new(min, max)
        .rev()
        .find_map(|i| Palindrome::new(i));

    match (min_pal, max_pal) {
        (Some(min), Some(max)) => Some((min, max)),
        _ => None,
    }
}
