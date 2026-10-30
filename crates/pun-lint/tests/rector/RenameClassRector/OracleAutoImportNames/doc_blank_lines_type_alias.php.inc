<?php

namespace App;

/**
 * @var int
 *
 *
 * @return \Rector\Tests\Renaming\Rector\Name\RenameClassRector\Source\OldClass
 */
function f()
{
}

/**
 * @phpstan-type Broken array{
 * @phpstan-type Fine = int extra text
 * @return \Rector\Tests\Renaming\Rector\Name\RenameClassRector\Source\OldClass
 */
function g()
{
}
-----
<?php

namespace App;

use Rector\Tests\Renaming\Rector\Name\RenameClassRector\Source\NewClass;

/**
 * @var int
 *
 *
 * @return NewClass
 */
function f()
{
}

/**
 * @phpstan-type Broken array{
 * @phpstan-type Fine = int extra text
 * @return NewClass
 */
function g()
{
}
