<?php

namespace App;

class A1
{
    public function run()
    {
        $f = function ($x) {
            /** @var \Rector\Tests\Renaming\Rector\Name\RenameClassRector\Source\SomeFinalClass $y */
            $y = $x;

            return $y;
        };

        return new \Rector\Tests\Renaming\Rector\Name\RenameClassRector\Source\OldClass();
    }
}

class A2
{
    public function run()
    {
        /** @var \Rector\Tests\Renaming\Rector\Name\RenameClassRector\Source\SomeFinalClass $y */
        return new \Rector\Tests\Renaming\Rector\Name\RenameClassRector\Source\OldClass();
    }
}

class A3
{
    /** @var \Rector\Tests\Renaming\Rector\Name\RenameClassRector\Source\SomeFinalClass */
    const X = 1;

    public function run()
    {
        return new \Rector\Tests\Renaming\Rector\Name\RenameClassRector\Source\OldClass();
    }
}

class A4
{
    public function run()
    {
        return fn (/** @var \Rector\Tests\Renaming\Rector\Name\RenameClassRector\Source\SomeFinalClass */ $a) => new \Rector\Tests\Renaming\Rector\Name\RenameClassRector\Source\OldClass();
    }
}
-----
<?php

namespace App;

use Rector\Tests\Renaming\Rector\Name\RenameClassRector\Source\NewClass;
use Rector\Tests\Renaming\Rector\Name\RenameClassRector\Source\SomeFinalClass;

class A1
{
    public function run()
    {
        $f = function ($x) {
            /** @var SomeFinalClass $y */
            $y = $x;

            return $y;
        };

        return new NewClass();
    }
}

class A2
{
    public function run()
    {
        /** @var SomeFinalClass $y */
        return new NewClass();
    }
}

class A3
{
    /** @var SomeFinalClass */
    const X = 1;

    public function run()
    {
        return new NewClass();
    }
}

class A4
{
    public function run()
    {
        return fn (/** @var SomeFinalClass */
        $a) => new NewClass();
    }
}
