<?php

$days = glob("./2015/days/*.php");
foreach ($days as $day) {
  require($day);
}

$solutions = [];
$solutions['day_1']['part_1'] = day1solution1();
$solutions['day_1']['part_2'] = day1solution1b();
$solutions['day_2']['part_1'] = day2part1();
$solutions['day_2']['part_2'] = day2part1b();

// OUTPUT THE SOLUTIONS WE HAVE.
foreach ($solutions as $day => $parts) {
  foreach ($parts as $part => $answer) {
    echo "*** The answer to $day $part is: $answer ***\n";
  }
}
