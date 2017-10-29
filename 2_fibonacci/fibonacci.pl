#!/usr/bin/perl

use strict;
use warnings;

my @fibs = (1,2);

while (scalar @fibs < 10) {
    push @fibs, $fibs[-1] + $fibs[-2];
}

print "@fibs\n";
