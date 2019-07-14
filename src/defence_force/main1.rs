use dq10tools::defence_force;

fn main() {
    let state = defence_force::army::get_current_state();

    println!("現在の敵は{}です", state.army.colorized_name());
    println!("{}分後に{}に変わります", state.next_in, state.next_army.colorized_name());
}

