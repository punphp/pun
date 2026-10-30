<?php

class A
{
    /**
     * @Foo(\Rector\Tests\Renaming\Rector\Name\RenameClassRector\Source\OldClass::BAR, x=\Rector\Tests\Renaming\Rector\Name\RenameClassRector\Source\OldClass)
     * @var \Rector\Tests\Renaming\Rector\Name\RenameClassRector\Source\OldClass
     */
    public $a;

    /**
     * @Target(\Rector\Tests\Renaming\Rector\Name\RenameClassRector\Source\OldClass::BAR)
     */
    public $b;

    /**
     * See {@link x} and {@see \Rector\Tests\Renaming\Rector\Name\RenameClassRector\Source\OldClass}
     * @var \Rector\Tests\Renaming\Rector\Name\RenameClassRector\Source\OldClass
     */
    public $c;
}
-----
<?php

class A
{
    /**
     * @Foo(\Rector\Tests\Renaming\Rector\Name\RenameClassRector\Source\OldClass::BAR, x=\Rector\Tests\Renaming\Rector\Name\RenameClassRector\Source\OldClass)
     * @var \Rector\Tests\Renaming\Rector\Name\RenameClassRector\Source\NewClass
     */
    public $a;

    /**
     * @Target(\Rector\Tests\Renaming\Rector\Name\RenameClassRector\Source\OldClass::BAR)
     */
    public $b;

    /**
     * See {@link x} and {@see \Rector\Tests\Renaming\Rector\Name\RenameClassRector\Source\OldClass}
     * @var \Rector\Tests\Renaming\Rector\Name\RenameClassRector\Source\NewClass
     */
    public $c;
}
