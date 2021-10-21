<?php

$array = [];

for ($i = 0; $i < 100000; $i = $i + 1) {
    $array[$i] = 1;
}

echo count($array);