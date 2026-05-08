(* Tree datatype defenition *)
datatype 'a tree = Empty 
                 | Node of 'a * 'a tree * 'a tree;


(* Main function *)
fun add_to_search_tree Empty x = Node(x, Empty, Empty)
  | add_to_search_tree (Node(v, left, right)) x =
    if x < v then
        Node(v, add_to_search_tree left x, right)
    else
        Node(v, left, add_to_search_tree right x);


(* In order to support a binary tree that all of its elements are floating point numbers,
we need to add type annotation to the function. By default, compiler assumes that comparison operators 
(like '<' in line 9), are being used with integers. We need to explicitly tell the compiler that the 
input is real. That is, we should write: (x: real) in the function's signature.
Once we label x as a real, the compiler infers that the entire tree is real, and the comparison x < v,
should use floating point logic.)