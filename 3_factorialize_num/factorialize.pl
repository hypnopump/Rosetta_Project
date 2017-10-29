#!/usr/bin/perl

use strict;
use warnings;

sub factorialize {
    my ($num) = @_;
    if ($num == 0) {
	return 1;
    } else {
	return $num * factorialize($num - 1);
    }
}

factorialize(0) == 1 || die "Bad return value for 1\n";
factorialize(5) == 120 || die "Bad return value for 5\n";
factorialize(7) == 5040 || die "Bad return value for 7\n";
factorialize(9) == 362880 || die "Bad return value for 9\n";

print "all tests passed\n";

