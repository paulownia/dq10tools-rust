use dq10tools::panigarm;

fn main() {
    let boss = panigarm::get_current_boss();

    println!("今週のボスは{}です", boss.name);
}
