<?php

if (is_object($x) && method_exists($y, '__toString')) {
    echo (string) $y;
}
-----
