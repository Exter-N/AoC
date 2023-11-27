#!/usr/bin/env php
<?php

declare(strict_types=1);

const ROCK = 0;
const PAPER = 1;
const SCISSORS = 2;

const LOSE = 0;
const DRAW = 1;
const WIN = 2;

/**
 * @return int[]
 */
function parseLine(string $rawLine): array
{
    static $lut = [
        '' => null,
        'A' => ROCK,
        'B' => PAPER,
        'C' => SCISSORS,
        'X' => LOSE,
        'Y' => DRAW,
        'Z' => WIN,
    ];

    $line = trim($rawLine);
    $words = explode(' ', $line);
    $nums = array_map(fn (string $word): ?int => $lut[$word] ?? throw new Exception('Bad word '.$word), $words);

    return array_values(array_filter($nums, fn (?int $value) => null !== $value));
}

function play(int $ours, int $theirs): int
{
    return (4 + $ours - $theirs) % 3;
}

function predictOurs(int $result, int $theirs): int
{
    return (2 + $result + $theirs) % 3;
}

$totalScore = 0;
foreach (file($argv[1] ?? 'input') as $line) {
    $nums = parseLine($line);
    switch (count($nums)) {
    case 0:
        // empty line → ignore
        break;
    case 2:
        [$theirs, $ours] = $nums;
        $result = play($ours, $theirs);
        // [$theirs, $result] = $nums;
        // $ours = predictOurs($result, $theirs);
        $score = (1 + $ours) /* for gesture */
            + (3 * $result) /* for result */;
        $totalScore += $score;
        break;
    default:
        throw new Exception('Bad word count in line '.trim($line));
    }
}

echo $totalScore, PHP_EOL;
