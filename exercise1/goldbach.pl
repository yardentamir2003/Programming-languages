% check if N is even and greater than 2, then find the pair.
goldbach(N, [P1, P2]) :-
    N > 2,
    N mod 2 =:= 0,

    % Use cut to stop after the first pair, return only one solution
    find_pair(N, 2, P1, P2), !. 

% Return an empty list if no pair of prime numbers is found
goldbach(_, []).

% find_pair(Target, Current, P1, P2)
find_pair(N, P1, P1, P2) :-
    is_prime(P1),
    P2 is N - P1,
    is_prime(P2).

find_pair(N, Current, P1, P2) :-
    Current < N // 2,
    Next is Current + 1,
    find_pair(N, Next, P1, P2).

% Check if number is prime
is_prime(2).
is_prime(3).
is_prime(P) :-
    P > 3,
    P mod 2 =\= 0,
    \+ has_divisor(P, 3).

has_divisor(N, D) :-
    N mod D =:= 0.
has_divisor(N, D) :-
    D * D < N,
    New_D is D + 2,
    has_divisor(N, New_D).