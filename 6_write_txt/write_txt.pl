#!/usr/bin/perl

use strict;
use warnings;

open IN, "<", "data.txt" || die "Couldn't open data.txt for reading\n";

my $is_first = 1;
my ($first, $last);
while(<IN>) {
    if ($is_first) {
	$first = $_;
	$is_first = 0;
    }
    $last = $_;
}
close IN;

open OUT, ">", "write_pl.txt" || die "Couldn't open write_pl.txt for writing.\n";

print OUT "$first\n$last";

close OUT;
