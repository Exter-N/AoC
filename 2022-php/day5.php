#!/usr/bin/env php
<?php

declare(strict_types=1);

const PARSING_STACKS = 0;
const PARSING_MOVES = 1;

/**
 * @return string[]
 */
function parseStacksLine(string $rawLine): array
{
	$line = rtrim($rawLine);
    $len = strlen($line);
    $chars = [];
    for ($i = 1; $i < $len; $i += 4) {
        $chars[] = $line[$i];
    }

	return $chars;
}

/**
 * @return int[]
 */
function parseMovesLine(string $rawLine): array
{
    $line = trim($rawLine);
	if (!\preg_match_all('/\d+/', $line, $matches)) {
		return [];
	}

	return array_map('intval', $matches[0]);
}

$state = PARSING_STACKS;
$stacks = [];
foreach (file($argv[1] ?? 'input') as $line) {
    switch ($state) {
        case PARSING_STACKS:
            $chars = parseStacksLine($line);
            if (!count($chars)) {
                $state = PARSING_MOVES;
                continue 2;
            }
            foreach ($chars as $i => $char) {
                if (!isset($stacks[$i])) {
                    $stacks[$i] = [];
                }
                if (' ' !== $char) {
                    array_unshift($stacks[$i], $char);
                }
            }
            break;
        case PARSING_MOVES:
            $nums = parseMovesLine($line);
            if (!count($nums)) {
                continue 2;
            }
            [$howMany, $from, $to] = $nums;
            $fromStack =& $stacks[$from - 1];
            $toStack =& $stacks[$to - 1];
            $moved = array_splice($fromStack, count($fromStack) - $howMany, $howMany);
            // $moved = array_reverse($moved);
            array_splice($toStack, count($toStack), 0, $moved);
            break;
    }
}

foreach ($stacks as $stack) {
    echo end($stack);
}
echo PHP_EOL;
