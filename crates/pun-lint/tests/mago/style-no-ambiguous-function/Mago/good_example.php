<?php

namespace App;

use function strlen;

// OK: Explicitly imported
$length1 = strlen("hello");

// OK: Explicitly global
$length2 = \strlen("hello");

// OK: Explicitly namespaced
$value = namespace\my_function();
