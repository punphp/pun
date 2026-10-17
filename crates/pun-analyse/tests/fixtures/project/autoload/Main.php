<?php

// Classes Composer's psr-4 directories hold, which Pun scans once a scanned file names them.

namespace Autoload;

use Acme\Deep\Child;
use Acme\Deep\{Grouped, Sub\Nested};
use Acme\Deep\Sub;
use Acme\Two\Extra\Thing;

// declared in Base, which only Child's file names (through a namespace alias in another case)
Child::fromParent();
Child::notDeclared();
// a group use
new Grouped();
new Nested();
// relative to an imported namespace
new Sub\Other();
// a class string with `\\`
$class = "Acme\\Deep\\FromString";
new $class();
// below the second of two psr-4 directories of one package
new Thing();
// in excludePaths' analyseAndScan
new \Acme\Deep\Excluded();
new Missing();
