<?php

namespace App;

use Doctrine\ORM\Mapping as ORM;

class A
{
    /**
     * The items.
     *
     * @ORM\OneToMany(targetEntity="Rector\Tests\Renaming\Rector\Name\RenameClassRector\Source\OldClass")
     */
    public $a;
}
-----
<?php

namespace App;

use Doctrine\ORM\Mapping as ORM;

class A
{
    /**
     * The items.
     *
     * @ORM\OneToMany(targetEntity="Rector\Tests\Renaming\Rector\Name\RenameClassRector\Source\NewClass")
     */
    public $a;
}
