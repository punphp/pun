<?php

namespace App;

use Doctrine\ORM\Mapping as ORM;

class A
{
    /**
     * @ORM\JoinTable(joinColumns={@ORM\JoinColumn(name="x"), @ORM\OneToMany(targetEntity="Rector\Tests\Renaming\Rector\Name\RenameClassRector\Source\OldClass")})
     */
    public $a;

    /**
     * @ORM\JoinTable(a={"x"}, b={@ORM\OneToMany(targetEntity="Rector\Tests\Renaming\Rector\Name\RenameClassRector\Source\OldClass")})
     */
    public $b;
}
-----
<?php

namespace App;

use Doctrine\ORM\Mapping as ORM;

class A
{
    /**
     * @ORM\JoinTable(joinColumns={@ORM\JoinColumn(name="x"), @ORM\OneToMany(targetEntity="Rector\Tests\Renaming\Rector\Name\RenameClassRector\Source\NewClass")})
     */
    public $a;

    /**
     * @ORM\JoinTable(a={"x"}, b={@ORM\OneToMany(targetEntity="Rector\Tests\Renaming\Rector\Name\RenameClassRector\Source\NewClass")})
     */
    public $b;
}
