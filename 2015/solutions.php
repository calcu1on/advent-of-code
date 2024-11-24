<?php

$days = glob('./days/*.php');
foreach ($days as $day) {
  require($day);
}

$day1 = day1solution1();
$day1b = day1solution1b();

