pub mod year_2015;
pub mod year_2016;
pub mod year_2017;
pub mod year_2018;
pub mod year_2019;
pub mod year_2020;
pub mod year_2021;
pub mod year_2022;

pub fn print_implemented() {
    print_implemented_year("2015", year_2015::get_implemented());
    print_implemented_year("2016", year_2016::get_implemented());
    print_implemented_year("2017", year_2017::get_implemented());
    print_implemented_year("2018", year_2018::get_implemented());
    print_implemented_year("2019", year_2019::get_implemented());
    print_implemented_year("2020", year_2020::get_implemented());
    print_implemented_year("2021", year_2021::get_implemented());
    print_implemented_year("2022", year_2022::get_implemented());
}

fn print_implemented_year(year: &str, implemented_challenges: Vec<&str>) {
    println!("== {} ==", year);
    for challenge in implemented_challenges {
        println!("{}", challenge);
    }
    println!();
}

fn print_not_implemented(year: u16, day: u8) {
    println!("{}: day {} not implemented yet", year, day);
}
