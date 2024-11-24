<?php

// Problem: https://adventofcode.com/2015/day/1

/*
 * Solution 1
 *
 * Runtime: ~0.158ms
 */
function day1solution1() {
  $instructions = file_get_contents('./2015/inputs/day1.txt');
  $up_floors = substr_count($instructions, "(");
  $down_floors = substr_count($instructions, ")");
  $total = $up_floors - $down_floors;

  return $total;
}

/*
 * Solution 1b
 *
 * Runtime: ~.33ms
 */
function day1solution1b() {
  $instructions = file_get_contents('./2015/inputs/day1.txt');
  $chars = str_split($instructions);
  $position = 1;
  $floor = 0;
  foreach ($chars as $char) {
    match ($char) {
      "(" => $floor++,
      ")" => $floor--,
    };
    if ($floor == -1) {
      break;
    }
    else {
      $position++;
    }
  };

  return $position;
}
