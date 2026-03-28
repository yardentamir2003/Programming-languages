% Base case - remove the first element
remove_at(X, [X|Tail], 1, Tail).

% Recursion, save head and search at the rest of the list
remove_at(X, [H|Tail], K, [H|Rest]) :-
    K > 1,
    New_K is K - 1,
    remove_at(X, Tail, New_K, Rest).