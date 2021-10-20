<?php

$i = 0;

while (true) {
    if ($i > 3) {
        break;
    }

    echo $i;

    $i = $i + 1;
}

echo "Done!";