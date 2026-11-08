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

class A
{
    /** Some text @\Doctrine\ORM\Mapping\OneToMany(targetEntity="Rector\Tests\Renaming\Rector\Name\RenameClassRector\Source\OldClass") @\Doctrine\ORM\Mapping\OneToMany(targetEntity="Rector\Tests\Renaming\Rector\Name\RenameClassRector\Source\NewClass")
     */
    public $a;

    /**
     * @var \Rector\Tests\Renaming\Rector\Name\RenameClassRector\Source\NewClass callable(x)
     */
    public $b;

    /**
     * @var \Rector\Tests\Renaming\Rector\Name\RenameClassRector\Source\NewClass
     *
     * @see x
     */
    public $c;

    /**
     * @var callable<T of \Rector\Tests\Renaming\Rector\Name\RenameClassRector\Source\NewClass>(T): void
     */
    public $d;

    /**
     * @var array{a: \Rector\Tests\Renaming\Rector\Name\RenameClassRector\Source\NewClass, ...<string, \Rector\Tests\Renaming\Rector\Name\RenameClassRector\Source\NewClass>}
     */
    public $e;

    /**
     * @Doctrine\ORM\Mapping\JoinTable(@Doctrine\ORM\Mapping\JoinColumns(@Doctrine\ORM\Mapping\OneToMany(targetEntity="Rector\Tests\Renaming\Rector\Name\RenameClassRector\Source\NewClass")))
     */
    public $f;
}
