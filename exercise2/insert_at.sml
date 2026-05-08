fun insert_at (x, 0, l) = x :: l    (* If index is 0, add x to the begining *)
  | insert_at (x, n, h::t) = h :: insert_at (x, n - 1, t)   (* Else, recusion on tail *)
  | insert_at (_, _, []) = []