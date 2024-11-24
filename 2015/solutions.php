<?php

$days = glob("./2015/days/*.php");
foreach ($days as $day) {
  require($day);
}

$d1 = day1solution1();
$d1b = day1solution1b();

