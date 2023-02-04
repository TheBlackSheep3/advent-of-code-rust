pub mod year_2022;

pub fn print_implemented() {
    print_implemented_year("2015", Vec::<&str>::new());
    print_implemented_year("2016", Vec::<&str>::new());
    print_implemented_year("2017", Vec::<&str>::new());
    print_implemented_year("2018", Vec::<&str>::new());
    print_implemented_year("2019", Vec::<&str>::new());
    print_implemented_year("2020", Vec::<&str>::new());
    print_implemented_year("2021", Vec::<&str>::new());
    print_implemented_year("2022", year_2022::get_implemented());
}

fn print_implemented_year(year: &str, implemented_challenges: Vec<&str>) {
    println!("== {} ==", year);
    for challenge in implemented_challenges {
        println!("{}", challenge);
    }
    println!();
}
