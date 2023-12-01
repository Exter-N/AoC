#!/usr/bin/env php
<?php

$digits = [
    '0' => 0,
    '1' => 1,
    '2' => 2,
    '3' => 3,
    '4' => 4,
    '5' => 5,
    '6' => 6,
    '7' => 7,
    '8' => 8,
    '9' => 9,
    'zero' => 0,
    'one' => 1,
    'two' => 2,
    'three' => 3,
    'four' => 4,
    'five' => 5,
    'six' => 6,
    'seven' => 7,
    'eight' => 8,
    'nine' => 9,
];

$tigids = [];
foreach ($digits as $digit => $value) {
    $tigids[\strrev($digit)] = $value;
}

$calibrationSum = 0;
foreach (\file($argv[1] ?? 'input') as $line) {
    $line = \trim($line);
    $first = null;
    $firstPos = \INF;
    foreach ($digits as $digit => $value) {
        $pos = \strpos($line, \strval($digit));
        if (false !== $pos && $pos < $firstPos) {
            $first = $value;
            $firstPos = $pos;
        }
    }
    $enil = \strrev($line);
    $last = null;
    $lastPos = \INF;
    foreach ($tigids as $tigid => $value) {
        $pos = \strpos($enil, \strval($tigid));
        if (false !== $pos && $pos < $lastPos) {
            $last = $value;
            $lastPos = $pos;
        }
    }
	$calibration = $first * 10 + $last;
	$calibrationSum += $calibration;
}
\printf("Sum of calibration values: %s\n", $calibrationSum);
