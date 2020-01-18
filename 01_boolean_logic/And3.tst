// File name: projects/01/And3.tst

load And3.hdl,
output-file And3.out,
compare-to And3.cmp,
output-list a%B3.1.3 b%B3.1.3 c%B3.1.3 out%B3.1.3;

set a 0,
set b 0,
set c 1,
eval,
output;

set a 0,
set b 1,
set c 1,
eval,
output;

set a 1,
set b 0,
set c 1,
eval,
output;

set a 1,
set b 1,
set c 1,
eval,
output;
