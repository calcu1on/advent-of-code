<?php
/**
* It contains at least three vowels (aeiou only), like aei, xazegov, or aeiouaeiouaeiou.
* It contains at least one letter that appears twice in a row, like xx, abcdde (dd), or aabbccdd (aa, bb, cc, or dd).
* It does not contain the strings ab, cd, pq, or xy, even if they are part of one of the other requirements.
*/


// What do i need to do for each string?
// Count vowels, check if there are >=3
// Count unique chars, ensure one char appears twice
// Check for a certain set of strings to omit.

function checkNice(string $string): bool {
  preg_match_all('/[aeiou]/i', $string, $vowels);
  if (count($vowels[0]) < 3) {
    return FALSE;
  }
  if (!preg_match('/(.)\1/', $string, $match)){
    return FALSE;
  }
  $omit_strings = [
    'ab',
    'cd',
    'pq',
    'xy',
  ];
  foreach ($omit_strings as $omit) {
    if (str_contains($string, $omit))  {
      return FALSE;
    }
  }

  return TRUE;
}

function checkNice2(string $string): bool {
  $two_char_sets = str_split($string, 2);
  $str_to_arr = str_split($string);
  $counts = array_count_values($two_char_sets);
  $is_nice = FALSE;
  foreach ($counts as $substr => $count) {
    if ($count >= 2) {
      $is_nice = TRUE;
      preg_match_all('/(.)(.)\1/', $string, $matches); 
      if (count($matches[0]) > 1) {
        $is_nice = TRUE;
      }
      else {
        $is_nice = FALSE;
      }
    }
  }

  return $is_nice;
}

function day5($part = 1) {
  $file = new SplFileObject("./2015/inputs/day5.txt");
  // Loop until we reach the end of the file.
  $nice_strings = 0;
  while (!$file->eof()) {
    // Echo one line from the file.
    if ($part == 1) {
      $is_nice = checkNice($file->fgets());
    }
    elseif ($part == 2) {
      $is_nice = checkNice2($file->fgets());
    }
    if ($is_nice) {
      $nice_strings++;
    }
  }
  // Unset the file to call __destruct(), closing the file handle.
  $file = null;

  return $nice_strings;
}
