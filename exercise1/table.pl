% Define logical predicates
bool(true).
bool(fail).
and(X,Y):- X,Y.
or(X,_):- X.
or(_,Y):- Y.
not(X):- \+X.
xor(X,Y):- X, \+Y.
xor(X,Y):- Y, \+X.
nor(X,Y):- \+X, \+Y.
nand(X,Y):- \+ (X,Y).
equal(X,Y):- X,Y.
equal(X,Y):- \+X, \+Y.

% Define table predicate
table(A, B, Expr) :-
    bool(A),
    bool(B),
    (Expr -> Result = true ; Result = fail),
    
    % Print table to screen
    write(A), write('  '), 
    write(B), write('  '), 
    write(Result),
    nl,
    fail.

table(_, _, _).
