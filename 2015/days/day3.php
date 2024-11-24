<?php

function part1() {
  // Get elf instructions.
  $directions = file_get_contents('./2015/inputs/day3.txt');
  // Convert the string to an array for processing.
  $directions = str_split($directions);
  $position = [0,0];
  $deliveries = [];
  foreach ($directions as $direction) {
    // First, deliver a present at current location.
    $location_key = $position[0] . '_' . $position[1];
    $location_key = crypt($location_key, '123');
    if (isset($deliveries[$location_key])) {
      $deliveries[$location_key]++;
    }
    else {
      $deliveries[$location_key] = 1;
    }
    if ($direction == '^') {
      $position[1]++;
    }
    elseif($direction == 'v') {
      $position[1]--; 
    }
    elseif ($direction == '>') {
      $position[0]++;
    }
    elseif ($direction == '<') {
      $position[0]--; 
    }
  }

  return $deliveries;
}

function part2() {
  // Get elf instructions.
  $directions = file_get_contents('./2015/inputs/day3.txt');
  // Convert the string to an array for processing.
  $directions = str_split($directions);
  $santa_position = [0,0];
  $robo_position = [0,0];
  $deliveries = [];
  $turn = 'santa';
  foreach ($directions as $direction) {
    if ($turn == 'santa') {
      $deliveries = updateDeliveryList($deliveries, $santa_position);
      $santa_position = changeCoordinates($santa_position, $direction);
      $turn = 'robo';
    }
    elseif ($turn == 'robo') {
      $deliveries = updateDeliveryList($deliveries, $robo_position);
      $robo_position = changeCoordinates($robo_position, $direction);
      $turn = 'santa';
    }
  }

  return $deliveries;
}

function updateDeliveryList(array $list, $coords) {
  $location_key = $coords[0] . '_' . $coords[1];
  $location_key = crypt($location_key, '123');
  if (!isset($list[$location_key])) {
    $list[$location_key] = 1;
  }
  else {
    $list[$location_key]++;
  }

  return $list;
}

function changeCoordinates($coords, $move) {
  match ($move) {
    '^' => $coords[1]++,
    'v' => $coords[1]--,
    '>' => $coords[0]++,
    '<' => $coords[0]--,
    default => null,
  };

  return $coords;
}


function day3($part = 1) {
  if ($part == 1) {
    return count(part1());
  }
  elseif ($part == 2) {
    return count(part2());
  }
}

