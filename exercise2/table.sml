(* Datatype defenition expr *)
datatype expr = Var of string 
              | Not of expr 
              | And of expr * expr 
              | Or of expr * expr;

(* Evaluate the expression given environment and variable s *)
fun evaluate (Var s) env =
    let 
        fun find ((name, v)::ts) = if name = s then v else find ts
          | find [] = false     (* If s was not found, return false as default *)
    in 
        find env 
    end
  | evaluate (Not e) env = not (evaluate e env)
  | evaluate (And(e1, e2)) env = (evaluate e1 env) andalso (evaluate e2 env)
  | evaluate (Or(e1, e2)) env = (evaluate e1 env) orelse (evaluate e2 env);

(* Generate all possible combinations *)
fun generate [] = [[]]      (* Base case *)
  | generate (x::tail) =
    let 
        val rest = generate tail
    in
        (List.map (fn env => env @ [(x, true)]) rest) @ (* Recursion on tail - rest of the variables *)
        (List.map (fn env => env @ [(x, false)]) rest)
    end;

(* Main function *)
fun table vars e =
    let 
        val envs = generate vars
    in
        List.map (fn env => (env, evaluate e env)) envs
    end;