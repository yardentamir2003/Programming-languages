% Base case - X is the first element in the list
element_at(X, [X|_], 1).

% Recursion on lists tail
element_at(X, [_|Tail], K) :-
    K > 1,
    New_K is K - 1,
    element_at(X, Tail, New_K).
    