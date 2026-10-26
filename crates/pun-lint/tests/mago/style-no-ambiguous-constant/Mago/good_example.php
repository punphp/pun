<?php

namespace App;

use const PHP_VERSION;

// OK: Explicitly imported
$version1 = PHP_VERSION;

// OK: Explicitly global
$version2 = \PHP_VERSION;
