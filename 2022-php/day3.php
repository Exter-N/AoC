#!/usr/bin/env php
<?php

declare(strict_types=1);

/**
 * @return string[]
 */
function parseLine(string $rawLine): array
{
    $line = trim($rawLine);
    $len = strlen($line);
    if (0 !== ($len % 2)) {
        throw new Exception('Imbalanced halfs');
    }

    return [substr($line, 0, $len >> 1), substr($line, $len >> 1)];
}

function charsIntersect(string $s1, string $s2): string
{
    return implode('', array_keys(array_intersect_key(array_flip(str_split($s1)), array_flip(str_split($s2)))));
}

function priority(int $ord): int
{
    if ($ord >= 97 && $ord <= 122) {
        return $ord + 1 - 97;
    } elseif ($ord >= 65 && $ord <= 90) {
        return $ord + 27 - 65;
    } else {
        throw new Exception('Bad item type '.chr($ord));
    }
}

$p1PrioritySum = 0;
$p2PrioritySum = 0;
$intersect = null;
$iInGroup = -1;
foreach (file($argv[1] ?? 'input') as $line) {
    $items = parseLine($line);
    if (!strlen($items[0])) {
        continue;
    }
    $p1PrioritySum += priority(ord(charsIntersect(...$items)));
    $iInGroup = ($iInGroup + 1) % 3;
    if (0 === $iInGroup) {
        $intersect = implode('', $items);
    } else {
        $intersect = charsIntersect($intersect, implode('', $items));
    }
    if (2 === $iInGroup) {
        $p2PrioritySum += priority(ord($intersect));
    }
}

echo $p1PrioritySum, ', ', $p2PrioritySum, PHP_EOL;
