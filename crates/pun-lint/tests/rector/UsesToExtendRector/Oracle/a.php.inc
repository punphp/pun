<?php

namespace Tests;

trait LocalTrait
{
}

abstract class LocalCase
{
}

uses(LocalCase::class, LocalTrait::class, Unknown::class)->in('Feature');
uses(LocalTrait::class)->group('a')->in(
    'Unit',
);
\uses(LocalTrait::class);
USES(LocalCase::class, $notAClass, 'string', static::class);
uses('string');
uses();
pest()->group('x')->uses(LocalTrait::class)->in('Feature');
pest()->uses(LocalCase::class)->uses(LocalTrait::class);
pest()->extend(LocalCase::class);
uses(LocalTrait::class)->group;
$x = uses(LocalCase::class, \Tests\LocalTrait::class, LocalCase::CONSTANT);
foo()->uses(LocalCase::class);
-----
<?php

namespace Tests;

trait LocalTrait
{
}

abstract class LocalCase
{
}

pest()->extend(LocalCase::class, Unknown::class)->use(LocalTrait::class)->in('Feature');
pest()->use(LocalTrait::class)->group('a')->in('Unit');
pest()->use(LocalTrait::class);
pest()->extend(LocalCase::class, static::class);
uses('string');
uses();
pest()->use(LocalTrait::class)->in('Feature');
pest()->use(LocalTrait::class);
pest()->extend(LocalCase::class);
pest()->use(LocalTrait::class)->group;
$x = pest()->extend(LocalCase::class, LocalCase::CONSTANT)->use(\Tests\LocalTrait::class);
foo()->uses(LocalCase::class);
