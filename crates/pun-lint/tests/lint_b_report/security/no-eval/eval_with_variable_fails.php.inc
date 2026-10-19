<?php

$code = '$a = 1 + 2;';
eval($code);
-----
4:1: Unsafe use of `eval` construct.
