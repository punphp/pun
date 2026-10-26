<?php

namespace Tests;

abstract class LocalCase
{
}

uses(LocalCase::class, $x::class)->in('Feature');
-----
<?php

namespace Tests;

abstract class LocalCase
{
}

pest()->extend(LocalCase::class)->in('Feature');
