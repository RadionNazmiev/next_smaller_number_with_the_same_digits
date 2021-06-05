fn next_smaller_number(n: u64) -> Option<u64> {
    let mut digs = n.to_string().chars()
        .map(|c| c.to_digit(10).unwrap())
        .collect::<Vec<u32>>();
    let bnp = digs.windows(2).rposition(|d| d[0] > d[1])?;
    let bn = digs[bnp];//2
    let mn = digs[bnp+1..].iter().filter(|&d| d < &bn).max()?;
    let mnp = digs.iter().rposition(|&d| d == *mn)?;
    digs.swap(bnp, mnp);
    let (_, tail) = digs.split_at_mut(bnp+1);
    tail.reverse();
    if digs[0] == 0 { return None; }
    digs.into_iter()
        .map(|d| std::char::from_digit(d, 10).unwrap()).collect::<String>()
        .parse::<u64>().ok()
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example() {
        assert_eq!(Some(12), next_smaller_number(21));
        assert_eq!(Some(790), next_smaller_number(907));
        assert_eq!(Some(513), next_smaller_number(531));
        assert_eq!(None, next_smaller_number(1027));
        assert_eq!(Some(414), next_smaller_number(441));
    }
}
