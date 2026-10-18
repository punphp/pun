<?php

namespace UsedEnum;

use Some\Thing;
use \Other\Lead;
use function Some\fn1, Some\fn2;
use const Some\C1;
use Some\{Grouped, function groupedFn, const GROUPED_C};
use function Some\{typedFn};

enum Thing
{
}

class Lead
{
}

class Fn1
{
}

class C1
{
}

class GroupedFn
{
}

use Another\Unrelated as FN1;
use Another\{Other as C1};
