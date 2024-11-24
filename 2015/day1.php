<?php

// Problem: https://adventofcode.com/2015/day/1

/*
 * Solution 1
 *
 * Runtime: ~0.158ms
 */
function solution1() {
  $start_time = microtime(true);
  $instructions = file_get_contents('./inputs/day1.txt');
  $up_floors = substr_count($instructions, "(");
  $down_floors = substr_count($instructions, ")");
  $total = $up_floors - $down_floors;
  echo "The answer is $total\n";
  $end_time = microtime(true);
  $execution_time = ($end_time - $start_time);
  echo $execution_time * 1000 . "\n";
}
solution1();

/*
 * Solution 2
 *
 * Runtime: ~0.622ms
 */
function solution2() {
  $start_time = microtime(true);
  $instructions = file_get_contents('./inputs/day1.txt');
  $chars = str_split($instructions);
  $floor = 0;
  foreach ($chars as $char) {
    match ($char) {
      "(" => $floor++,
      ")" => $floor--,
      default => 0,
    };
  }
  echo "The answer is $floor\n";
  $end_time = microtime(true);
  $execution_time = ($end_time - $start_time);
  echo $execution_time * 1000 . "\n";
}
solution2();

/*
 * Solution 3
 *
 * Runtime: ~0
 */
function solution3() {
  $start_time = microtime(true);
  $instructions = file_get_contents('./inputs/day1.txt');
  $chars = str_split($instructions);
  $values = array_count_values($chars);
  $floor = $values['('] - $values[')'];
  echo "The answer is $floor\n";
  $end_time = microtime(true);
  $execution_time = ($end_time - $start_time);
  echo $execution_time * 1000 . "\n";
}
solution3();
