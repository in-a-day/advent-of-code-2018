use regex::Regex;
use std::{collections::HashMap, io::Read, str::FromStr};

#[macro_use]
extern crate lazy_static;

type ShifGroup = HashMap<usize, Vec<Record>>;
type SleepInfo = HashMap<usize, (usize, [usize; 60])>;

fn main() {
    let mut input = std::io::stdin();
    let mut content = String::new();
    if let Err(_) = input.read_to_string(&mut content) {
        return;
    }

    let mut records = vec![];
    for line in content.lines() {
        let rc = Record::from_str(line).unwrap();
        records.push(rc);
    }
    records.sort_by(|a, b| a.date().cmp(b.date()));

    let mut current_id = None;
    let mut sg = ShifGroup::new();
    for r in records {
        if let Record::Shift(_, id) = r {
            current_id = Some(id);
        }
        if let Some(id) = current_id {
            sg.entry(id).or_default().push(r);
        }
    }

    // gurad_id -> (sum_sleep_time, minutes)
    let mut sleep_info = SleepInfo::new();
    for (guard_id, records) in sg {
        sleep_info.insert(guard_id, sum_and_minutes(&records));
    }

    println!("part1: {}", part1(&sleep_info));
    println!("part2: {}", part2(&sleep_info));
}

fn part1(sleep_info: &SleepInfo) -> usize {
    let (guard_id, (_, minutes)) = sleep_info.iter().max_by_key(|(_, (sum, _))| *sum).unwrap();
    let minute = minutes
        .iter()
        .enumerate()
        .max_by_key(|(_, val)| *val)
        .map(|(minute, _)| minute)
        .unwrap();

    guard_id * minute
}

fn part2(sleep_info: &SleepInfo) -> usize {
    let (guard_id, (minute, _)) = sleep_info
        .iter()
        .map(|(guard_id, (_, minutes))| {
            (
                guard_id,
                minutes
                    .iter()
                    .enumerate()
                    .max_by_key(|(_, &val)| val)
                    .unwrap(),
            )
        })
        .max_by_key(|(_, (_, &val))| val)
        .unwrap();

    guard_id * minute
}

fn sum_and_minutes(records: &Vec<Record>) -> (usize, [usize; 60]) {
    let mut last_status: Option<&Record> = None;
    let mut sum = 0;
    let mut minutes = [0; 60];
    for record in records {
        match record {
            Record::Shift(_, _) => (),
            Record::Asleep(_) => {
                last_status = Some(record);
            }
            Record::Wakeup(end) => {
                let rc = last_status.expect("unexcepted status");
                match rc {
                    Record::Asleep(dt) => {
                        sum += end.minute as usize - dt.minute as usize;
                        for minute in dt.minute..end.minute {
                            minutes[minute as usize] += 1;
                        }
                    }
                    _ => panic!("unexcepted status"),
                }
            }
        }
    }

    (sum, minutes)
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
struct Date {
    year: u16,
    month: u8,
    day: u8,
    hour: u8,
    minute: u8,
}

#[derive(Debug)]
enum Record {
    Shift(Date, usize),
    Asleep(Date),
    Wakeup(Date),
}

impl Record {
    fn date(&self) -> &Date {
        match self {
            Self::Shift(d, _) => d,
            Self::Asleep(d) => d,
            Self::Wakeup(d) => d,
        }
    }
}

impl FromStr for Record {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        lazy_static! {
            static ref RE: Regex = Regex::new(
                r"(?x)
                  \[
                  (?P<year>\d+)-
                  (?P<month>\d+)-
                  (?P<day>\d+)\s+
                  (?P<hour>\d+):
                  (?P<minute>\d+)
                  \]\s+
                  (?P<action>falls\sasleep|wakes\sup|Guard\s\#(?P<id>\d+)\sbegins\sshift)
                  "
            )
            .unwrap();
        }

        let caps = RE.captures(s).unwrap();

        let dt = Date {
            year: caps["year"].parse().unwrap(),
            month: caps["month"].parse().unwrap(),
            day: caps["day"].parse().unwrap(),
            hour: caps["hour"].parse().unwrap(),
            minute: caps["minute"].parse().unwrap(),
        };
        if let Some(id) = caps.name("id") {
            Ok(Record::Shift(dt, id.as_str().parse().unwrap()))
        } else if &caps["action"] == "falls asleep" {
            Ok(Record::Asleep(dt))
        } else {
            Ok(Record::Wakeup(dt))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn split() {
        let re = Regex::new(
            r"(?x)
                            \[(.*)\]",
        )
        .unwrap();
        let s = "[1234]";
        println!("{:?}", re.captures(s).unwrap().get(1).unwrap().as_str());
        println!("{}", re.is_match(s));
    }

    #[test]
    fn re_should_compile() {
        let re: Regex = Regex::new(
            r"(?x)
              \[
              (?P<year>\d+)-
              (?P<month>\d+)-
              (?P<day>\d+)\s+
              (?P<hour>\d+):
              (?P<minute>\d+)
              \]\s+
              (?P<action>falls\sasleep|wakes\sup|Guard\s\#(?P<id>\d+)\sbegins\sshift)
              ",
        )
        .unwrap();

        let s = "[1518-04-12 00:36] falls asleep";

        println!("{}", s);
        if let Some(ap) = re.captures(s) {
            println!("{:?}", ap);
        }
    }
}
