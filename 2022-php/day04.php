#!/usr/bin/env php
<?php

declare(strict_types=1);

/**
 * @return int[]
 */
function parseLine(string $rawLine): array
{
	$line = trim($rawLine);
	if (!\preg_match_all('/\d+/', $line, $matches)) {
		return [];
	}

	return array_map('intval', $matches[0]);
}

$contain = 0;
$overlap = 0;
foreach (file($argv[1] ?? 'input') as $line) {
    $nums = parseLine($line);
    if (!count($nums)) {
        continue;
    }
    [$start1, $end1, $start2, $end2] = $nums;
    // these two conditions are equivalent:
    // if ($start1 >= $start2 && $end1 <= $end2 || $start2 >= $start1 && $end2 <= $end1) {
    if (($start1 <=> $start2) * ($end1 <=> $end2) <= 0) {
	    ++$contain;
    }
    if ($start1 <= $end2 && $start2 <= $end1) {
	    ++$overlap;
    }
}

echo $contain, ', ', $overlap, PHP_EOL;
