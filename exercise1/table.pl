% Define logical predicates
not(true, fail).
not(fail, true).
or(true, _, true).
or(fail, X, X).
equal(X, X, true).
equal(X, Y, fail) :- X \= Y.
and(true, true, true).
and(true, fail, fail).
and(fail, true, fail).
and(fail, fail, fail).
xor(true, fail, true).
xor(fail, true, true).
xor(true, true, fail).
xor(fail, fail, fail).
nand(A, B, R) :- and(A, B, T), not(T, R).
nor(A, B, R) :- or(A, B, T), not(T, R).

% Recursion
evaluate(true, true) :- !.
evaluate(fail, fail) :- !.
evaluate(and(A, B), R) :- !, evaluate(A, RA), evaluate(B, RB), and(RA, RB, R).
evaluate(or(A, B), R) :- !, evaluate(A, RA), evaluate(B, RB), or(RA, RB, R).
evaluate(not(A), R) :- !, evaluate(A, RA), not(RA, R).
evaluate(xor(A, B), R) :- !, evaluate(A, RA), evaluate(B, RB), xor(RA, RB, R).
evaluate(nand(A, B), R) :- !, evaluate(A, RA), evaluate(B, RB), nand(RA, RB, R).
evaluate(nor(A, B), R) :- !, evaluate(A, RA), evaluate(B, RB), nor(RA, RB, R).
evaluate(equal(A, B), R) :- !, evaluate(A, RA), evaluate(B, RB), equal(RA, RB, R).

% Create table predicate, check all combinations of A and B
table(A, B, Expr) :-
    bind(A),
    bind(B),
    evaluate(Expr, Result),

    % If it is the last line of the table, do not add new line
    (  (A == fail, B == fail) 
    -> write(A), write('  '), write(B), write('  '), write(Result)
    ;  write(A), write('  '), write(B), write('  '), write(Result), nl
    ),
    fail.
table(_, _, _).

% Bind variables to boolean atoms
bind(true).
bind(fail).