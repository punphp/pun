<?php

$foo = 'bar';
$varName = 'foo';

echo $$varName; // Outputs 'bar'
-----
6:6: Usage of variable variables (e.g., `$$var`) is discouraged.
