use dq10tools::defence_force;

fn main() {
    let opt_state = defence_force::state::get_current_state();
    if let Some(state) = opt_state {
        println!("現在の敵は{}です", state.troop.colorized_name());
        println!("{}分後に{}に変わります", state.next_in, state.next_troop.colorized_name());
    }
}

