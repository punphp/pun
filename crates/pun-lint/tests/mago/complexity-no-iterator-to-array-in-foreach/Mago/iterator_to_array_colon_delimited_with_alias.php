<?php

use function iterator_to_array as to_Array;

foreach (to_Array($iterator) as $value):
    echo $value;
endforeach;
