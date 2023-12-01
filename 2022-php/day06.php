#!/usr/bin/env php
<?php

declare(strict_types=1);

function hasAllDistinctChars(string $str, int $offset = 0, ?int $length = null): bool
{
    if (null === $length) {
        $length = strlen($str) - $offset;
    }
    $chars = [];
    for ($i = 0, $j = $offset; $i < $length; ++$i, ++$j) {
        if (isset($chars[$str[$j]])) {
            return false;
        }
        $chars[$str[$j]] = true;
    }

    return true;
}

const MARKER_LENGTH = 14;

foreach (file($argv[1] ?? 'input') as $line) {
    $length = strlen($line);
    $start = null;
    for ($i = MARKER_LENGTH - 1, $j = 0; $i < $length; ++$i, ++$j) {
        if (hasAllDistinctChars($line, $j, MARKER_LENGTH)) {
            $start = $i + 1;
            break;
        }
    }
    echo $start, PHP_EOL;
}
