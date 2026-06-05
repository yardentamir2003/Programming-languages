use crate::ast::*;
use crate::semantics::*;

// The main Natural Operational Semantics function:
// nos: (Stm, State) -> State
pub fn nos(c: (Stm, State)) -> State {
    let (stm, state) = c;

    match stm {
        // Assignment: [ass]
        Stm::Ass(x, e) => update(&x, &e, &state),

        // Skip: [skip]
        Stm::Skip => state,

        // Composition: [comp]
        Stm::Comp(s1, s2) => {
            let state_after_s1 = nos((*s1, state));
            nos((*s2, state_after_s1))
        }

        // If: [if_tt] and [if_ff]
        Stm::If(b, s1, s2) => {
            if solve_b(&b, &state) == "tt" {
                nos((*s1, state))
            } else {
                nos((*s2, state))
            }
        }

        // While: [while_tt] and [while_ff]
        Stm::While(b, body) => {
            if solve_b(&b, &state) == "tt" {
                let state_after_body = nos((*body.clone(), state));
                nos((Stm::While(b, body), state_after_body))
            } else {
                state
            }
        }

        // DoWhile
        Stm::DoWhile(body, b) => {
            let state_after_body = nos((*body.clone(), state));
            if solve_b(&b, &state_after_body) == "tt" {
                nos((Stm::DoWhile(body, b), state_after_body))
            } else {
                state_after_body
            }
        }
    }
}