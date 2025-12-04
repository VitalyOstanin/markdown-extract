use chrono::NaiveDate;

/// Repeater type and interval
#[derive(Debug, Clone, PartialEq)]
pub struct Repeater {
    pub repeater_type: RepeaterType,
    pub value: u32,
    pub unit: RepeaterUnit,
}

/// Type of repeater
#[derive(Debug, Clone, PartialEq)]
pub enum RepeaterType {
    Cumulative,    // +
    CatchUp,       // ++
    Restart,       // .+
}

/// Repeater unit
#[derive(Debug, Clone, PartialEq)]
pub enum RepeaterUnit {
    Day,
    Week,
    Month,
    Year,
    Hour,
}

/// Parse repeater string like "+1d", "++2w", ".+1m"
pub fn parse_repeater(s: &str) -> Option<Repeater> {
    let s = s.trim();
    
    let (repeater_type, rest) = if let Some(r) = s.strip_prefix(".+") {
        (RepeaterType::Restart, r)
    } else if let Some(r) = s.strip_prefix("++") {
        (RepeaterType::CatchUp, r)
    } else if let Some(r) = s.strip_prefix('+') {
        (RepeaterType::Cumulative, r)
    } else {
        return None;
    };
    
    if rest.is_empty() {
        return None;
    }
    
    let unit_char = rest.chars().last()?;
    let value_str = &rest[..rest.len() - 1];
    let value: u32 = value_str.parse().ok()?;
    
    let unit = match unit_char {
        'd' => RepeaterUnit::Day,
        'w' => RepeaterUnit::Week,
        'm' => RepeaterUnit::Month,
        'y' => RepeaterUnit::Year,
        'h' => RepeaterUnit::Hour,
        _ => return None,
    };
    
    Some(Repeater {
        repeater_type,
        value,
        unit,
    })
}

/// Calculate next occurrence date for a repeater
pub fn next_occurrence(base_date: NaiveDate, repeater: &Repeater, from_date: NaiveDate) -> Option<NaiveDate> {
    use chrono::Datelike;
    
    match repeater.repeater_type {
        RepeaterType::Cumulative => {
            let mut current = base_date;
            let days = match repeater.unit {
                RepeaterUnit::Day => repeater.value as i64,
                RepeaterUnit::Week => (repeater.value * 7) as i64,
                RepeaterUnit::Month => return add_months(base_date, repeater.value as i32),
                RepeaterUnit::Year => return add_months(base_date, (repeater.value * 12) as i32),
                RepeaterUnit::Hour => 1,
            };
            
            while current < from_date {
                current += chrono::Duration::days(days);
            }
            Some(current)
        }
        RepeaterType::CatchUp => {
            let days = match repeater.unit {
                RepeaterUnit::Day => repeater.value as i64,
                RepeaterUnit::Week => (repeater.value * 7) as i64,
                RepeaterUnit::Month => return add_months(from_date, repeater.value as i32),
                RepeaterUnit::Year => return add_months(from_date, (repeater.value * 12) as i32),
                RepeaterUnit::Hour => 1,
            };
            
            if repeater.unit == RepeaterUnit::Week {
                let target_weekday = base_date.weekday();
                let mut current = from_date;
                while current.weekday() != target_weekday || current <= base_date {
                    current += chrono::Duration::days(1);
                }
                Some(current)
            } else {
                Some(from_date + chrono::Duration::days(days))
            }
        }
        RepeaterType::Restart => {
            let days = match repeater.unit {
                RepeaterUnit::Day => repeater.value as i64,
                RepeaterUnit::Week => (repeater.value * 7) as i64,
                RepeaterUnit::Month => return add_months(from_date, repeater.value as i32),
                RepeaterUnit::Year => return add_months(from_date, (repeater.value * 12) as i32),
                RepeaterUnit::Hour => 1,
            };
            Some(from_date + chrono::Duration::days(days))
        }
    }
}

fn add_months(date: NaiveDate, months: i32) -> Option<NaiveDate> {
    use chrono::Datelike;
    
    let mut year = date.year();
    let mut month = date.month() as i32 + months;
    
    while month > 12 {
        month -= 12;
        year += 1;
    }
    while month < 1 {
        month += 12;
        year -= 1;
    }
    
    let day = date.day().min(days_in_month(year, month as u32));
    NaiveDate::from_ymd_opt(year, month as u32, day)
}

fn days_in_month(year: i32, month: u32) -> u32 {
    match month {
        1 | 3 | 5 | 7 | 8 | 10 | 12 => 31,
        4 | 6 | 9 | 11 => 30,
        2 => {
            if year % 4 == 0 && (year % 100 != 0 || year % 400 == 0) {
                29
            } else {
                28
            }
        }
        _ => 30,
    }
}
