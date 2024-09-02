use time::error::ComponentRange;

use super::{time_span, Rule};

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
) -> Result<(Option<time_span::TimeSpan>, String), ComponentRange> {
    let description;
    let mut time: Option<time_span::TimeSpan> = None;

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
                let hm = time_pair
                    .into_inner()
                    .as_str()
                    .split(':')
                    .collect::<Vec<_>>();

                let mut hour = hm[0].parse().unwrap(); // won't fail
                if am_pm == "PM" {
                    hour += 12;
                }
                let minute = hm[1].parse().unwrap(); // won't fail

                time = Some(time_span::TimeSpan::new_unit(time::Time::from_hms(
                    hour, minute, 0,
                )?));
            }
            Rule::clock => {
                let hm = time_pair.as_str().split(':').collect::<Vec<_>>();
                let hour = hm[0].parse().unwrap(); // won't fail
                let minute = hm[1].parse().unwrap(); // won't fail

                time = Some(time_span::TimeSpan::new_unit(time::Time::from_hms(
                    hour, minute, 0,
                )?));
            }
            _ => unreachable!(),
        }
        description = schedule.next().unwrap().as_str().to_string();
    } else {
        description = pos.as_str().to_string();
    }

    Ok((time, description))
}
