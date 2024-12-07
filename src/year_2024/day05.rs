#[derive(Debug, PartialEq)]
pub enum Error {
    OrderingRuleParsingFailed,
    InvalidOrderingRule,
    PrintOrderParsingFailed,
    InvalidPrintOrder,
}

#[derive(Debug, PartialEq)]
struct OrderingRule {
    first: u32,
    second: u32,
}

impl OrderingRule {
    fn is_satisfied(&self, print_order: &Vec<u32>) -> bool {
        match (
            print_order.iter().position(|&i| i == self.first),
            print_order.iter().position(|&i| i == self.second),
        ) {
            (Some(index1), Some(index2)) => {
                println!("first index: {}, second index: {}", index1, index2);
                index1 < index2
            }
            _ => true,
        }
    }
}

impl TryFrom<&str> for OrderingRule {
    type Error = Error;
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let re: regex::Regex = regex::Regex::new(r"(\d+)\|(\d+)").unwrap();
        match re
            .captures(value)
            .map_or(None, |c| Some((c.get(1)?, c.get(2)?)))
        {
            Some((first, second)) => {
                let p1 = first
                    .as_str()
                    .parse::<u32>()
                    .map_err(|_| Error::OrderingRuleParsingFailed)?;
                let p2 = second
                    .as_str()
                    .parse::<u32>()
                    .map_err(|_| Error::OrderingRuleParsingFailed)?;
                if p1 == p2 {
                    Err(Error::InvalidOrderingRule)
                } else {
                    Ok(OrderingRule {
                        first: p1,
                        second: p2,
                    })
                }
            }
            None => Err(Error::OrderingRuleParsingFailed),
        }
    }
}

fn parse_print_order(order_str: &str) -> Result<Vec<u32>, Error> {
    let x: Vec<Option<u32>> = order_str
        .split(',')
        .into_iter()
        .map(|item| item.parse::<u32>().ok())
        .collect();
    if x.iter().all(|item| item.is_some()) {
        if x.len() % 2 == 1 && x.len() == x.iter().collect::<std::collections::HashSet<_>>().len() {
            Ok(x.iter().map(|item| item.unwrap()).collect())
        } else {
            Err(Error::InvalidPrintOrder)
        }
    } else {
        Err(Error::PrintOrderParsingFailed)
    }
}

pub fn sum_middle_page_numbers_of_valid_print_orders(input: &str) -> u32 {
    let mut x: OrderingRule = OrderingRule {
        first: 1,
        second: 2,
    };
    x.is_satisfied(&vec![]);
    0
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_STR: &str = "47|53
97|13
97|61
97|47
75|29
61|13
75|53
29|13
97|29
53|29
61|53
97|53
61|29
47|13
75|47
97|75
47|61
75|61
47|29
75|13
53|13

75,47,61,53,29
97,61,53,29,13
75,29,13
75,97,47,61,53
61,13,29
97,13,75,29,47";

    #[test]
    fn sum() {
        assert_eq!(sum_middle_page_numbers_of_valid_print_orders(TEST_STR), 143)
    }

    #[test]
    fn parse_rule() {
        assert_eq!(
            OrderingRule::try_from("891|23"),
            Ok(OrderingRule {
                first: 891,
                second: 23
            })
        );
        assert_eq!(
            OrderingRule::try_from("1|2"),
            Ok(OrderingRule {
                first: 1,
                second: 2
            })
        );
        assert_eq!(
            OrderingRule::try_from(" 12|2 "),
            Ok(OrderingRule {
                first: 12,
                second: 2
            })
        );
        assert_eq!(
            OrderingRule::try_from("9|4\n"),
            Ok(OrderingRule {
                first: 9,
                second: 4
            })
        );
        assert_eq!(
            OrderingRule::try_from("2|2"),
            Err(Error::InvalidOrderingRule)
        );
        assert_eq!(
            OrderingRule::try_from("123"),
            Err(Error::OrderingRuleParsingFailed)
        );
        assert_eq!(
            OrderingRule::try_from("123|"),
            Err(Error::OrderingRuleParsingFailed)
        );
        assert_eq!(
            OrderingRule::try_from("|123"),
            Err(Error::OrderingRuleParsingFailed)
        );
    }

    #[test]
    fn check_rule() {
        let rule = OrderingRule {
            first: 1u32,
            second: 2u32,
        };
        assert!(rule.is_satisfied(&vec![1u32, 2u32, 3u32]));
        assert!(rule.is_satisfied(&vec![1u32, 3u32, 2u32]));
        assert!(rule.is_satisfied(&vec![3u32, 2u32]));
        assert!(rule.is_satisfied(&vec![1u32, 3u32]));
        assert!(!rule.is_satisfied(&vec![2u32, 1u32, 3u32]));
        assert!(!rule.is_satisfied(&vec![2u32, 3u32, 1u32]));
    }

    #[test]
    fn parse_order() {
        assert_eq!(parse_print_order("11,2,4"), Ok(vec![11u32, 2u32, 4u32]));
        assert_eq!(
            parse_print_order("32,11,2,8,4"),
            Ok(vec![32u32, 11u32, 2u32, 8u32, 4u32])
        );
        assert_eq!(
            parse_print_order("12,3,4,"),
            Err(Error::PrintOrderParsingFailed)
        );
        assert_eq!(
            parse_print_order(",12,3,4"),
            Err(Error::PrintOrderParsingFailed)
        );
        assert_eq!(parse_print_order("8,12,3,4"), Err(Error::InvalidPrintOrder));
        assert_eq!(parse_print_order("8,4,4"), Err(Error::InvalidPrintOrder));
    }
}
