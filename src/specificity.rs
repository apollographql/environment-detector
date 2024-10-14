use std::{cmp::Ordering, collections::HashSet, hash::Hash};

pub trait Specificity<Rhs = Self> {
    fn specificity_cmp(&self, other: &Rhs) -> Option<Ordering>;
}

impl<T> Specificity for Option<T>
where
    T: Eq,
{
    fn specificity_cmp(&self, other: &Self) -> Option<Ordering> {
        match (self.as_ref(), other.as_ref()) {
            (Some(left), Some(right)) => {
                if left == right {
                    Some(Ordering::Equal)
                } else {
                    None
                }
            }
            (Some(_), None) => Some(Ordering::Greater),
            (None, Some(_)) => Some(Ordering::Less),
            (None, None) => Some(Ordering::Equal),
        }
    }
}

impl<T> Specificity for HashSet<T>
where
    T: Eq + Hash,
{
    fn specificity_cmp(&self, other: &Self) -> Option<Ordering> {
        let self_subset = self.iter().all(|i| other.contains(i));
        let other_subset = other.iter().all(|i| self.contains(i));

        match (self_subset, other_subset) {
            (true, true) => Some(Ordering::Equal),
            (true, false) => Some(Ordering::Less),
            (false, true) => Some(Ordering::Greater),
            (false, false) => None,
        }
    }
}

#[cfg(test)]
#[rstest::rstest]
#[case(&["a", "b", "c"], &["a", "b"], Some(Ordering::Greater))]
#[case(&["a", "b", "c"], &["a", "b", "c"], Some(Ordering::Equal))]
#[case(&["a", "b"], &["a", "b", "c"], Some(Ordering::Less))]
#[case(&["a", "b"], &["a", "c"], None)]
fn test_hashset(
    #[case] left: &'static [&'static str],
    #[case] right: &'static [&'static str],
    #[case] expected: Option<Ordering>,
) {
    let left: HashSet<_> = left.into_iter().collect();
    let right: HashSet<_> = right.into_iter().collect();

    let result = left.specificity_cmp(&right);
    assert_eq!(result, expected);
}

pub trait OrderingExt {
    fn merge_specificity(self, other: Self) -> Self;
}

impl OrderingExt for Option<Ordering> {
    fn merge_specificity(self, other: Self) -> Self {
        match (self, other) {
            (Some(Ordering::Greater), Some(Ordering::Greater | Ordering::Equal)) => {
                Some(Ordering::Greater)
            }
            (Some(Ordering::Greater), Some(Ordering::Less)) => None,
            (Some(Ordering::Equal), Some(Ordering::Greater)) => Some(Ordering::Greater),
            (Some(Ordering::Equal), Some(Ordering::Equal)) => Some(Ordering::Equal),
            (Some(Ordering::Equal), Some(Ordering::Less)) => Some(Ordering::Less),
            (Some(Ordering::Less), Some(Ordering::Equal | Ordering::Less)) => Some(Ordering::Less),
            (Some(Ordering::Less), Some(Ordering::Greater)) => None,
            (_, None) | (None, _) => None,
        }
    }
}
