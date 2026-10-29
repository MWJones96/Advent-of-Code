mod aoc2024;
mod aoc2025;
mod aoc2026;

const YEARS: &[(&str, fn(u32), fn())] = &[
    ("2024", aoc2024::run_day, aoc2024::run_all),
    ("2025", aoc2025::run_day, aoc2025::run_all),
    ("2026", aoc2026::run_day, aoc2026::run_all),
];

fn print_usage() {
    println!("Usage:");
    println!("  cargo run -- <year>                  Run all days for the year");
    println!("  cargo run -- <year> <day>            Run specific day for the year");
    println!("  cargo run -- all                     Run all days for all years");
    println!();
    println!("Years: {}", YEARS.iter().map(|(y, _, _)| *y).collect::<Vec<_>>().join(", "));
    println!();
    println!("Examples:");
    println!("  cargo run -- 2025                    Run all 2025 days");
    println!("  cargo run -- 2025 7                  Run 2025 day 7");
    println!("  cargo run -- all                     Run all years");
}

fn main() {
    let mut args = std::env::args();
    args.next(); // Skip program name

    let year_arg = match args.next() {
        Some(y) => y,
        None => {
            print_usage();
            return;
        }
    };

    let day_arg = args.next();

    if year_arg == "all" {
        if day_arg.is_some() {
            eprintln!("Cannot specify day with 'all' year");
            print_usage();
        } else {
            for (_, _, run_all) in YEARS {
                run_all();
            }
        }
        return;
    }

    let Some((_, run_day, run_all)) = YEARS.iter().find(|(y, _, _)| *y == year_arg) else {
        eprintln!("Invalid year: '{}'", year_arg);
        print_usage();
        return;
    };

    match day_arg {
        Some(day_str) => match day_str.parse::<u32>() {
            Ok(day) => run_day(day),
            Err(_) => {
                eprintln!("Invalid day: '{}'", day_str);
                print_usage();
            }
        },
        None => run_all(),
    }
}
