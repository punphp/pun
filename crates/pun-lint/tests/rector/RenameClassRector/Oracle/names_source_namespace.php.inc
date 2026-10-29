<?php

namespace Rector\Tests\Renaming\Rector\Name\RenameClassRector\Source;

use Doctrine\ORM\Mapping\Column as Col;
use Doctrine\ORM\Mapping as ORM;

class Z
{
    /** @var OldClass */
    public $a;

    /**
     * @Col(name="x")
     * @var OldClass
     */
    public $b;

    /**
     * @OldClass
     * @var OldClass
     */
    public $c;

    /**
     * @ORM\JoinTable(@\Doctrine\ORM\Mapping\JoinColumn(name="x"))
     * @var OldClass
     */
    public $d;

    /**
     * @ORM
     * @var OldClass
     */
    public $e;
}
-----
<?php

namespace Rector\Tests\Renaming\Rector\Name\RenameClassRector\Source;

use Doctrine\ORM\Mapping\Column as Col;
use Doctrine\ORM\Mapping as ORM;

class Z
{
    /** @var \Rector\Tests\Renaming\Rector\Name\RenameClassRector\Source\NewClass */
    public $a;

    /**
     * @Col(name="x")
     * @var \Rector\Tests\Renaming\Rector\Name\RenameClassRector\Source\NewClass
     */
    public $b;

    /**
     * @OldClass
     * @var \Rector\Tests\Renaming\Rector\Name\RenameClassRector\Source\NewClass
     */
    public $c;

    /**
     * @ORM\JoinTable(@\Doctrine\ORM\Mapping\JoinColumn(name="x"))
     * @var \Rector\Tests\Renaming\Rector\Name\RenameClassRector\Source\NewClass
     */
    public $d;

    /**
     * @ORM
     * @var \Rector\Tests\Renaming\Rector\Name\RenameClassRector\Source\NewClass
     */
    public $e;
}
