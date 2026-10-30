<?php

namespace App;

class A
{
    /** Some text @\Doctrine\ORM\Mapping\OneToMany(targetEntity="Rector\Tests\Renaming\Rector\Name\RenameClassRector\Source\OldClass") */
    public $a;

    /**
     * @var \Rector\Tests\Renaming\Rector\Name\RenameClassRector\Source\OldClass callable  (x)
     */
    public $b;

    /**
     * @var \Rector\Tests\Renaming\Rector\Name\RenameClassRector\Source\OldClass
     *
     * @see x
     */
    public $c;

    /**
     * @var callable<T of \Rector\Tests\Renaming\Rector\Name\RenameClassRector\Source\OldClass>(T): void
     */
    public $d;

    /**
     * @var array{a: \Rector\Tests\Renaming\Rector\Name\RenameClassRector\Source\OldClass, ...<string, \Rector\Tests\Renaming\Rector\Name\RenameClassRector\Source\OldClass>}
     */
    public $e;

    /**
     * @Doctrine\ORM\Mapping\JoinTable(@Doctrine\ORM\Mapping\JoinColumns(@Doctrine\ORM\Mapping\OneToMany(targetEntity="Rector\Tests\Renaming\Rector\Name\RenameClassRector\Source\OldClass")))
     */
    public $f;
}
-----
<?php

namespace App;

use Rector\Tests\Renaming\Rector\Name\RenameClassRector\Source\NewClass;
use Doctrine\ORM\Mapping\JoinTable;
use Doctrine\ORM\Mapping\JoinColumns;

class A
{
    /** Some text @\Doctrine\ORM\Mapping\OneToMany(targetEntity="Rector\Tests\Renaming\Rector\Name\RenameClassRector\Source\OldClass") @\Doctrine\ORM\Mapping\OneToMany(targetEntity="Rector\Tests\Renaming\Rector\Name\RenameClassRector\Source\NewClass")
     */
    public $a;

    /**
     * @var NewClass callable(x)
     */
    public $b;

    /**
     * @var NewClass
     *
     * @see x
     */
    public $c;

    /**
     * @var callable<T of \Rector\Tests\Renaming\Rector\Name\RenameClassRector\Source\OldClass>(T): void
     */
    public $d;

    /**
     * @var array{a: NewClass, ...<string, NewClass>}
     */
    public $e;

    /**
     * @JoinTable(@JoinColumns(@Doctrine\ORM\Mapping\OneToMany(targetEntity="Rector\Tests\Renaming\Rector\Name\RenameClassRector\Source\NewClass")))
     */
    public $f;
}
