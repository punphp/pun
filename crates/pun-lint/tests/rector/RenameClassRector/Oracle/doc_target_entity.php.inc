<?php

namespace App;

use Doctrine\ORM\Mapping as ORM;

class A
{
    /**
     * @ORM\OneToMany(mappedBy="x")
     * @var \Rector\Tests\Renaming\Rector\Name\RenameClassRector\Source\OldClass
     */
    public $a;

    /**
     * @ORM\OneToMany(targetEntity=\Rector\Tests\Renaming\Rector\Name\RenameClassRector\Source\OldClass::class)
     * @var \Rector\Tests\Renaming\Rector\Name\RenameClassRector\Source\OldClass
     */
    public $b;

    /**
     * @ORM\OneToMany(targetEntity="App\Other")
     * @var \Rector\Tests\Renaming\Rector\Name\RenameClassRector\Source\OldClass
     */
    public $c;
}
-----
<?php

namespace App;

use Doctrine\ORM\Mapping as ORM;

class A
{
    /**
     * @ORM\OneToMany(mappedBy="x")
     * @var \Rector\Tests\Renaming\Rector\Name\RenameClassRector\Source\NewClass
     */
    public $a;

    /**
     * @ORM\OneToMany(targetEntity=\Rector\Tests\Renaming\Rector\Name\RenameClassRector\Source\OldClass::class)
     * @var \Rector\Tests\Renaming\Rector\Name\RenameClassRector\Source\NewClass
     */
    public $b;

    /**
     * @ORM\OneToMany(targetEntity="App\Other")
     * @var \Rector\Tests\Renaming\Rector\Name\RenameClassRector\Source\NewClass
     */
    public $c;
}
