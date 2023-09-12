#[cfg(test)]
mod tests {
    use rgb_to_hex::rgb;

    #[test]
    fn test_zero() {
        assert_eq!(rgb(0, 0, 0), "000000");
    }

    #[test]
    fn test_123() {
        assert_eq!(rgb(1, 2, 3), "010203");
    }

    #[test]
    fn test_full() {
        assert_eq!(rgb(255, 255, 255), "FFFFFF");
    }

    #[test]
    fn test_nextl() {
        assert_eq!(rgb(254, 253, 252), "FEFDFC");
    }

    #[test]
    fn test_outside_range() {
        assert_eq!(rgb(-20, 275, 125), "00FF7D");
    }
}
