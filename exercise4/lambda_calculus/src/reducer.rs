use std::collections::HashSet;
use crate::parser::Term;
use crate::utils;

/// Returns the set of free variables in a term.
fn fv(t: &Term) -> HashSet<String> {
    match t {
        // FV(x) = {x}
        Term::Variable(x) => {
            let mut set = HashSet::new();
            set.insert(x.clone());
            set
        }
        // FV(\x. t) = FV(t) - {x}
        Term::Abstraction(x, body) => {
            let mut set = fv(body);
            set.remove(x);
            set
        }
        // FV(t1 t2) = FV(t1) U FV(t2)
        Term::Application(t1, t2) => {
            let mut set1 = fv(t1);
            let set2 = fv(t2);
            set1.extend(set2);
            set1
        }
    }
}

/// Substitute: replace occurrences of `x` with `t1` inside `t2`.
/// Notation: t2 [x := t1]
fn substitute(x: &str, t1: &Term, t2: &Term) -> Term {
    match t2 {
        Term::Variable(y) => {
            if y == x {
                // x[x := t1] = t1
                t1.clone()
            } else {
                // y[x := t1] = y   (if y != x)
                Term::Variable(y.clone())
            }
        }
        Term::Application(t2_left, t2_right) => {
            // (t2_left t2_right)[x := t1] = (t2_left[x := t1]) (t2_right[x := t1])
            Term::Application(
                Box::new(substitute(x, t1, t2_left)),
                Box::new(substitute(x, t1, t2_right)),
            )
        }
        Term::Abstraction(y, body) => {
            if y == x {
                // (\x. body)[x := t1] = \x. body
                Term::Abstraction(y.clone(), body.clone())
            } else {
                let fv_t1 = fv(t1);
                if !fv_t1.contains(y) {
                    // (\y. body)[x := t1] = \y. (body[x := t1])
                    // if y != x and y not in FV(t1)
                    Term::Abstraction(y.clone(), Box::new(substitute(x, t1, body)))
                } else {
                    // (\y. body)[x := t1] = \z. (body[y := z])[x := t1]
                    // if y != x and y in FV(t1)
                    // z not in FV(t1) U FV(body) U {x}
                    
                    let mut used_vars = fv_t1;
                    used_vars.extend(fv(body));
                    used_vars.insert(x.to_string());
                    
                    let z = utils::fresh_var(&used_vars);
                    
                    // Rename y to z: body[y := z]
                    let z_term = Term::Variable(z.clone());
                    let body_renamed = substitute(y, &z_term, body);
                    
                    // Substitute x with t1: (body[y := z])[x := t1]
                    let final_body = substitute(x, t1, &body_renamed);
                    
                    Term::Abstraction(z, Box::new(final_body))
                }
            }
        }
    }
}

/// Helper function, check if an expression is a value (like we defined in class)
fn is_value(t: &Term) -> bool {
    matches!(t, Term::Abstraction(_, _))
}

/// Call-by-Value reducer.
/// Returns Some(reduced_term) or None if no reduction is possible.
pub fn reduce_cbv(t: &Term) -> Option<Term> {
    match t {
        // Variables can't be reduced
        Term::Variable(_) => None,
        
        // Reduction inside a function body is not allowed (E-Abs)
        Term::Abstraction(_, _) => None,
        
        Term::Application(t1, t2) => {
            // Try to reduce the left side first (E-App1)
            if let Some(t1_prime) = reduce_cbv(t1) {
                return Some(Term::Application(Box::new(t1_prime), t2.clone()));
            }

            // If the left side is a value, try to reduce the right side (E-App2)
            if is_value(t1) {
                if let Some(t2_prime) = reduce_cbv(t2) {
                    return Some(Term::Application(t1.clone(), Box::new(t2_prime)));
                }
            }

            // Beta-reduction, if t1 is a function and t2 is a value (E-AppAbs)
            if let Term::Abstraction(x, body) = &**t1 {
                if is_value(t2) {
                    return Some(substitute(x, t2, body));
                }
            }

            // If we can't apply any rule
            None
        }
    }
}

/// Call-by-Name reducer.
/// Returns Some(reduced_term) or None if no reduction is possible.
pub fn reduce_cbn(t: &Term) -> Option<Term> {
    match t {
        // Variables can't be reduced
        Term::Variable(_) => None,
        
        // Reduction inside a function body is not allowed (E-Abs)
        Term::Abstraction(_, _) => None,
        
        Term::Application(t1, t2) => {
            // Try to reduce the left side first (E-App1)
            if let Some(t1_prime) = reduce_cbn(t1) {
                return Some(Term::Application(Box::new(t1_prime), t2.clone()));
            }

            // Beta-reduction (E-AppAbs rule)
            // don't evaluate t2 before substitution. If t1 is a function, substitute t2 directly into its body
            if let Term::Abstraction(x, body) = &**t1 {
                // t12 [x := t2]
                return Some(substitute(x, t2, body));
            }

            // If we can't apply any rule
            None
        }
    }
}