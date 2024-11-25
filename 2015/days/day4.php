<?php

function day4(int $part = 1) {
  $key = 'bgvyzdsv';
  $stop_value = 0;
  for ($i = 0; $i < 10000000; $i++) {
    $stop_value = $key . $i;
    $md5 = md5($stop_value);
    if ($part == 1) {
      $identifier = '00000';
    }
    elseif ($part == 2) {
      $identifier = '000000';
    }
    if (str_starts_with($md5, $identifier)) {
      break;
    }
  }
  return str_replace($key, '', $stop_value);
}

