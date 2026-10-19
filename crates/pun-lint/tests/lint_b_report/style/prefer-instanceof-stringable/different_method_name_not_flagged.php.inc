<?php

if (is_object($x) && method_exists($x, 'serialize')) {
    echo $x->serialize();
}
-----
