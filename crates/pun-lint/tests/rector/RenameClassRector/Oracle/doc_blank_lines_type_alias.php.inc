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

/**
 * @var int
 *
 *
 * @return \Rector\Tests\Renaming\Rector\Name\RenameClassRector\Source\NewClass
 */
function f()
{
}

/**
 * @phpstan-type Broken array{
 * @phpstan-type Fine = int extra text
 * @return \Rector\Tests\Renaming\Rector\Name\RenameClassRector\Source\NewClass
 */
function g()
{
}
