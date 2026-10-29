<?php

namespace App;

class A
{
    /**
     * @var callable<T of \Rector\Tests\Renaming\Rector\Name\RenameClassRector\Source\OldClass>(\Rector\Tests\Renaming\Rector\Name\RenameClassRector\Source\OldClass $x): void
     */
    public $d;

    /**
     * @var \Closure<T of \Rector\Tests\Renaming\Rector\Name\RenameClassRector\Source\OldClass>(T): \Rector\Tests\Renaming\Rector\Name\RenameClassRector\Source\OldClass
     */
    public $e;

    /**
     * @return callable<T>(T): void
     */
    public function f(): \Rector\Tests\Renaming\Rector\Name\RenameClassRector\Source\OldClass {}
}
-----
<?php

namespace App;

class A
{
    /**
     * @var callable(\Rector\Tests\Renaming\Rector\Name\RenameClassRector\Source\NewClass $x):void
     */
    public $d;

    /**
     * @var \Closure(T):\Rector\Tests\Renaming\Rector\Name\RenameClassRector\Source\NewClass
     */
    public $e;

    /**
     * @return callable<T>(T): void
     */
    public function f(): \Rector\Tests\Renaming\Rector\Name\RenameClassRector\Source\NewClass {}
}
