use time::error::ComponentRange;

use crate::error;

use super::{time_wrapper, Rule};

pub fn werh_from_quex(as_str: &str) -> zemen::Werh {
    match as_str {
        "mes" | "መስከ" => zemen::Werh::Meskerem,
        "tik" | "ጥቅም" => zemen::Werh::Tikimit,
        "hed" | "ህዳር" => zemen::Werh::Hedar,
        "tah" | "ታኅሣ" => zemen::Werh::Tahasass,
        "tir" | "ጥር" => zemen::Werh::Tir,
        "yek" | "የካቲ" => zemen::Werh::Yekatit,
        "meg" | "መጋቢ" => zemen::Werh::Megabit,
        "miy" | "ሚያዝ" => zemen::Werh::Miyazia,
        "gin" | "ግንቦ" => zemen::Werh::Ginbot,
        "sen" | "ሴኒ" => zemen::Werh::Sene,
        "ham" | "ሐምሌ" => zemen::Werh::Hamle,
        "neh" | "ነሐሴ" => zemen::Werh::Nehase,
        "pua" | "ጳጉሜ" => zemen::Werh::Puagme,
        _ => unreachable!(),
    }
}

pub fn month_from_quex(month: &str) -> time::Month {
    match month {
        "jan" => time::Month::January,
        "feb" => time::Month::February,
        "mar" => time::Month::March,
        "apr" => time::Month::April,
        "may" => time::Month::May,
        "jun" => time::Month::June,
        "jul" => time::Month::July,
        "aug" => time::Month::August,
        "sep" => time::Month::September,
        "oct" => time::Month::October,
        "nov" => time::Month::November,
        "dec" => time::Month::December,
        _ => unreachable!(),
    }
}

pub fn get_time_description(
    schedule: &mut pest::iterators::Pairs<Rule>,
) -> Result<(Option<time_wrapper::TimeWrapper>, String), ComponentRange> {
    let description;
    let mut time: Option<time_wrapper::TimeWrapper> = None;

    let pos = schedule.next().unwrap();
    if pos.as_rule() == Rule::time {
        let time_pair = pos.into_inner().next().unwrap(); // won't fail
        match time_pair.as_rule() {
            Rule::am_pm => {
                let am_pm: String = time_pair
                    .as_str()
                    .chars()
                    .filter(|c| char::is_alphabetic(*c))
                    .collect();
                let hms = time_pair
                    .into_inner()
                    .as_str()
                    .split(':')
                    .collect::<Vec<_>>();

                let mut hour = hms[0].parse().unwrap(); // won't fail
                if am_pm == "PM" {
                    hour += 12;
                }
                let minute = hms[1].parse().unwrap(); // won't fail
                let second = hms.get(2).unwrap_or(&"0").parse().unwrap(); // won't fail

                time = Some(time_wrapper::TimeWrapper::from_hms(hour, minute, second)?);
            }
            Rule::clock => {
                let hms = time_pair.as_str().split(':').collect::<Vec<_>>();
                let hour = hms[0].parse().unwrap(); // won't fail
                let minute = hms[1].parse().unwrap(); // won't fail
                let second = hms.get(2).unwrap_or(&"0").parse().unwrap(); // won't fail

                time = Some(time_wrapper::TimeWrapper::from_hms(hour, minute, second)?);
            }
            _ => unreachable!(),
        }
        description = schedule.next().unwrap().as_str().to_string();
    } else {
        description = pos.as_str().to_string();
    }

    Ok((time, description))
}
