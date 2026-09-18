pub fn clamp<T: PartialOrd>(value: T, min: T, max: T) -> T {
    if value < min {
        min
    } else if value > max {
        max
    } else {
        value
    }
}

pub fn average<T>(slice: &[T]) -> Option<f64>
where
    T: Into<f64> + Copy,
{
    if slice.is_empty() {
        return None;
    }
    
    let sum: f64 = slice.iter().map(|&x| x.into()).sum();
    Some(sum / slice.len() as f64)
}

pub fn sum<T>(slice: &[T]) -> T
where
    T: std::iter::Sum + Copy,
{
    slice.iter().copied().sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_clamp_within_range() {
        assert_eq!(clamp(5, 0, 10), 5);
    }

    #[test]
    fn test_clamp_below_min() {
        assert_eq!(clamp(-5, 0, 10), 0);
    }

    #[test]
    fn test_clamp_above_max() {
        assert_eq!(clamp(15, 0, 10), 10);
    }

    #[test]
    fn test_average_normal() {
        let nums = vec![10, 20, 30];
        assert_eq!(average(&nums), Some(20.0));
    }

    #[test]
    fn test_average_empty() {
        let nums: Vec<i32> = vec![];
        assert_eq!(average(&nums), None);
    }

    #[test]
    fn test_sum_integers() {
        let nums = vec![1, 2, 3, 4];
        assert_eq!(sum(&nums), 10);
    }
}
