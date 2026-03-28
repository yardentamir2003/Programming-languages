% Base case - X is the first element in the list
element_at(X, [X|_], 1).

% Recursion on list's tail
element_at(X, [_|Tail], k) :-
    K > 1
    New_K = K - 1,
    element_at(X, [Tail|_], New_K).
