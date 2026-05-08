(* Helper function *)
fun insert_at (x, 0, l) = x :: l
  | insert_at (x, n, h::t) = h :: insert_at (x, n - 1, t)
  | insert_at (_, _, []) = []

(* New Function *)
fun insert_none_at n l = insert_at ("None", n, l) (* Call insert_at with fixed value using curring *)