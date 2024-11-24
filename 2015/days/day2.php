<?php

class Box {

  protected $length;
  protected $width;
  protected $height;

  /**
   * @param mixed $property
   */
  public function get($property): int|string {
    return $this->{$property};
  }

  /**
   * @param mixed $property
   * @param mixed $value
   */
  public function set($property, $value): void {
    $this->{$property} = $value;
  }

  /**
   * Calculates the required sq footage of paper needed.
   */ 
  public function calculateRequiredFootage(): int {
    $side_1 = $this->length * $this->width;
    $side_2 = $this->width * $this->height;
    $side_3 = $this->height * $this->length;
    $extra = min($side_1, $side_2, $side_3);
    $area = 
      (2 * $side_1) +
      (2 * $side_2) +
      (2 * $side_3);

    return $area + $extra;
  }

  public function calculateShortestPerimeter() {
    $perimeter_1 = 2 * $this->length + 2 * $this->width;
    $perimeter_2 = 2 * $this->width + 2 * $this->height;
    $perimeter_3 = 2 * $this->height + 2 * $this->length;

    return min($perimeter_1, $perimeter_2, $perimeter_3);
  }

  /*
   * Extracts the dimensions from a string.
   */
  public function extractDimensions(string $dimensions): void {
    $dimensions_exploded = explode("x", $dimensions);
    $this->length = $dimensions_exploded[0];
    $this->width = $dimensions_exploded[1];
    $this->height = $dimensions_exploded[2];
  }

  public function calculateRibbonRequirements() {
    $bow = $this->length * $this->width * $this->height;
    $shortest_perimeter = $this->calculateShortestPerimeter();

    return $bow + $shortest_perimeter;
  }

}

function day2part1() {
  $box_dimension_list = file_get_contents('./2015/inputs/day2.txt');
  $box_dimension_list = explode(PHP_EOL, $box_dimension_list);
  $wrapping_paper_total = 0;
  foreach ($box_dimension_list as $box_dimensions) {
    if (strlen($box_dimensions) < 1) {
      continue;
    }
    $box = new Box();
    $box->extractDimensions($box_dimensions);
    $wrapping_paper_total = $wrapping_paper_total + $box->calculateRequiredFootage();
  }
  return $wrapping_paper_total;
}

function day2part1b() {
  $box_dimension_list = file_get_contents('./2015/inputs/day2.txt');
  $box_dimension_list = explode(PHP_EOL, $box_dimension_list);
  $ribbon_requirements = 0;
  foreach ($box_dimension_list as $box_dimensions) {
    if (strlen($box_dimensions) < 1) {
      continue;
    }
    $box = new Box();
    $box->extractDimensions($box_dimensions);
    $ribbon_requirements = $ribbon_requirements + $box->calculateRibbonRequirements();
  }

  return $ribbon_requirements;
}


