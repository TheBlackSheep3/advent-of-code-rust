#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Error {
    OrderingRuleParsingFailed,
    InvalidOrderingRule,
    PrintOrderParsingFailed,
    InvalidPrintOrder,
    InputSplitFailed,
    IntOverflow,
    OrderFixFailed,
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::IntOverflow => write!(f, "an integer overflow occurred"),
            Error::InputSplitFailed => write!(f, "unable to split input correctly"),
            Error::InvalidPrintOrder => write!(f, "encountered invalid print order"),
            Error::InvalidOrderingRule => write!(f, "encountered invalid ordering rule"),
            Error::PrintOrderParsingFailed => write!(f, "unable to parse print order"),
            Error::OrderingRuleParsingFailed => write!(f, "unable to parse ordering rule"),
            Error::OrderFixFailed => write!(f, "unable to fix print order"),
        }
    }
}

#[derive(Debug, PartialEq)]
struct OrderingRule {
    first: u32,
    second: u32,
}

impl std::fmt::Display for OrderingRule {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} before {}", self.first, self.second)
    }
}

impl OrderingRule {
    fn is_satisfied(&self, print_order: &Vec<u32>) -> bool {
        match (
            print_order.iter().position(|&i| i == self.first),
            print_order.iter().position(|&i| i == self.second),
        ) {
            (Some(index1), Some(index2)) => index1 < index2,
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

fn parse_input(input: &str) -> Result<(Vec<OrderingRule>, Vec<Vec<u32>>), Error> {
    let lines = input.lines().collect::<Vec<&str>>();
    let pivot_line_index = lines
        .iter()
        .position(|l| l.is_empty())
        .ok_or_else(|| Error::InputSplitFailed)?;
    let rules = lines[..pivot_line_index]
        .iter()
        .map(|&line| line.try_into())
        .collect::<Vec<Result<OrderingRule, Error>>>();
    let rules = if rules.iter().all(|x| x.is_ok()) {
        Ok(rules
            .into_iter()
            .map(|x| x.unwrap())
            .collect::<Vec<OrderingRule>>())
    } else {
        Err(*rules
            .iter()
            .find(|&x| x.is_err())
            .unwrap()
            .as_ref()
            .unwrap_err())
    }?;
    let orders = lines[pivot_line_index + 1..]
        .iter()
        .map(|&line| parse_print_order(line))
        .collect::<Vec<Result<Vec<u32>, Error>>>();
    let orders = if orders.iter().all(|x| x.is_ok()) {
        Ok(orders
            .into_iter()
            .map(|x| x.unwrap())
            .collect::<Vec<Vec<u32>>>())
    } else {
        Err(*orders
            .iter()
            .find(|&x| x.is_err())
            .unwrap()
            .as_ref()
            .unwrap_err())
    }?;
    Ok((rules, orders))
}

pub fn sum_middle_page_numbers_of_valid_print_orders(input: &str) -> Result<u32, Error> {
    let (rules, orders) = parse_input(input)?;
    orders
        .iter()
        .filter(|&o| rules.iter().all(|r| r.is_satisfied(o)))
        .fold(Some(0u32), |acc, o| match acc {
            Some(x) => {
                let middle_index = o.len() / 2usize;
                x.checked_add(o[middle_index])
            }
            _ => None,
        })
        .ok_or(Error::IntOverflow)
}

fn fix_order(order: &Vec<u32>, rules: &Vec<OrderingRule>) -> Option<Vec<u32>> {
    let mut order: Vec<u32> = order.clone();
    let mut iterations: usize = 0usize;
    while rules.iter().any(|r| !r.is_satisfied(&order)) {
        if iterations > 1 << 10 {
            return None;
        }
        let mut fixed = order.clone();
        for rule in rules.iter().filter(|r| !r.is_satisfied(&order)) {
            fixed.swap(
                order.iter().position(|&i| i == rule.first).unwrap(),
                order.iter().position(|&i| i == rule.second).unwrap(),
            );
            if rules.iter().all(|r| r.is_satisfied(&order)) {
                break;
            }
        }
        order = fixed;
        iterations += 1;
    }
    Some(order)
}

pub fn sum_middle_page_numbers_of_fixed_invalid_print_orders(input: &str) -> Result<u32, Error> {
    let (rules, orders) = parse_input(input)?;
    orders
        .iter()
        .filter(|&o| rules.iter().any(|r| !r.is_satisfied(o)))
        .fold(Ok(0u32), |acc, o| match acc {
            Ok(x) => {
                let fixed = fix_order(o, &rules).ok_or(Error::OrderFixFailed)?;
                let middle_index = o.len() / 2usize;
                x.checked_add(fixed[middle_index]).ok_or(Error::IntOverflow)
            }
            e => e,
        })
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
        assert_eq!(
            sum_middle_page_numbers_of_valid_print_orders(TEST_STR),
            Ok(143)
        )
    }

    #[test]
    fn fix_orders() {
        let (rules, _) = parse_input(TEST_STR).unwrap();
        assert_eq!(
            fix_order(&vec![75u32, 97u32, 47u32, 61u32, 53u32], &rules),
            Some(vec![97u32, 75u32, 47u32, 61u32, 53u32])
        );
        assert_eq!(
            fix_order(&vec![61u32, 13u32, 29u32], &rules),
            Some(vec![61u32, 29u32, 13u32])
        );
        assert_eq!(
            fix_order(&vec![97u32, 13u32, 75u32, 29u32, 47u32], &rules),
            Some(vec![97u32, 75u32, 47u32, 29u32, 13u32])
        );
    }

    #[test]
    fn sum_fixed_orders() {
        assert_eq!(
            sum_middle_page_numbers_of_fixed_invalid_print_orders(TEST_STR),
            Ok(123)
        );
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
