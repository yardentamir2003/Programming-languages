#!/usr/bin/env sh

# element_at
echo "#!/usr/bin/env swipl
" >> temp.pl
cat element_at.pl >> temp.pl
echo "
:- forall(element_at(X,[a,b,c,d,e],3), writeln(X)).
:- halt.
" >> temp.pl
sudo chmod 777 temp.pl
./temp.pl >> results 
rm temp.pl

# remove_at
echo "#!/usr/bin/env swipl
" >> temp.pl
cat remove_at.pl >> temp.pl
echo "
:- forall(remove_at(X,[a,b,c,d],2,L), writeln([\"L =\" ,L, \"X =\", X])).
:- halt.
" >> temp.pl
sudo chmod 777 temp.pl
./temp.pl >> results 
rm temp.pl

# is_prime
echo "#!/usr/bin/env swipl
" >> temp.pl
cat is_prime.pl >> temp.pl
echo "
:- forall(is_prime(7), writeln(true)).
:- forall(is_prime(15), writeln(true)).
:- halt.
" >> temp.pl
sudo chmod 777 temp.pl
./temp.pl >> results 
rm temp.pl


# goldbach
echo "#!/usr/bin/env swipl
" >> temp.pl
cat goldbach.pl >> temp.pl
echo "
:- forall(goldbach(28, X), writeln(X)).
:- halt.
" >> temp.pl
sudo chmod 777 temp.pl
./temp.pl >> results 
rm temp.pl

# table
echo "#!/usr/bin/env swipl
" >> temp.pl
cat table.pl >> temp.pl
echo "
:- table(A,B, and(A,(or(A,not(B))))).
:- halt.
" >> temp.pl
sudo chmod 777 temp.pl
./temp.pl >> results 
rm temp.pl

# end
echo -n "number of error: " >> res
diff -y --suppress-common-lines test.txt results  | wc -l >> res
cat res
rm res
rm results