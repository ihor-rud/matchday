use tabled::{Table, Tabled};

pub fn print_table<T: Tabled>(
    table_name: &str,
    players: impl IntoIterator<Item = T>,
    limit: usize,
) {
    println!("{table_name}:");
    println!("{}", Table::new(players.into_iter().take(limit)));
}
