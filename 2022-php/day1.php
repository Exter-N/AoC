#!/usr/bin/env php
<?php

declare(strict_types=1);

function parseLine(string $rawLine): ?int
{
    $line = trim($rawLine);
    if (0 === strlen($line)) {
        return null;
    }

    return intval($line);
}

function finishList(array &$top, int &$running): void
{
    $top[] = $running;
    $running = 0;
    rsort($top);
    array_pop($top);
}

$top = array_fill(0, 3, 0);
$running = 0;
foreach (file($argv[1] ?? 'input') as $line) {
    $num = parseLine($line);
    if (null === $num) {
        finishList($top, $running);
    } else {
        $running += $num;
    }
}
finishList($top, $running);

echo implode(' + ', $top), ' = ', array_sum($top), PHP_EOL;
