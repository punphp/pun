<?php

namespace App;

use Doctrine\ORM\Mapping as ORM;

class A
{
    /**
     * @ORM\JoinTable(x={{@ORM\OneToMany(targetEntity="Rector\Tests\Renaming\Rector\Name\RenameClassRector\Source\OldClass")}=1})
     */
    public $a;

    /**
     * @ORM\JoinTable({{@ORM\OneToMany(targetEntity="Rector\Tests\Renaming\Rector\Name\RenameClassRector\Source\OldClass")}=1})
     */
    public $b;

    /**
     * @ORM\JoinTable(x={{"a", @ORM\OneToMany(targetEntity="Rector\Tests\Renaming\Rector\Name\RenameClassRector\Source\OldClass")}: 2})
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
     * @ORM\JoinTable(x={@ORM\OneToMany(targetEntity="Rector\Tests\Renaming\Rector\Name\RenameClassRector\Source\NewClass")})
     */
    public $a;

    /**
     * @ORM\JoinTable({@ORM\OneToMany(targetEntity="Rector\Tests\Renaming\Rector\Name\RenameClassRector\Source\NewClass")})
     */
    public $b;

    /**
     * @ORM\JoinTable(x={"a"@ORM\OneToMany(targetEntity="Rector\Tests\Renaming\Rector\Name\RenameClassRector\Source\NewClass")})
     */
    public $c;
}
