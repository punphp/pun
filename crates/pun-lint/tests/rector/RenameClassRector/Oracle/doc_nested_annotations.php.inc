<?php

namespace App;

use Doctrine\ORM\Mapping as ORM;
use Symfony\Component\Validator\Constraints as Assert;

class A
{
    /**
     * @ORM\JoinTable(name="t", joinColumns={@ORM\OneToMany(targetEntity="Rector\Tests\Renaming\Rector\Name\RenameClassRector\Source\OldClass")})
     */
    public $a;

    /**
     * @ORM\JoinColumns({@ORM\JoinColumn(name="x"), {@ORM\ManyToMany(targetEntity="Rector\Tests\Renaming\Rector\Name\RenameClassRector\Source\OldClass", mappedBy="m")}})
     */
    public $b;

    /**
     * @ORM\Embedded(class="Rector\Tests\Renaming\Rector\Name\RenameClassRector\Source\OldClass", columnPrefix=false)
     */
    public $c;

    /**
     * @Assert\All({@Assert\Choice(callback={"Rector\Tests\Renaming\Rector\Name\RenameClassRector\Source\OldClass", "values"}, multiple=true)})
     */
    public $d;

    /**
     * @ORM\JoinTable(@ORM\OneToMany(targetEntity="Rector\Tests\Renaming\Rector\Name\RenameClassRector\Source\OldClass")=1)
     */
    public $e;

    /**
     * @ORM\Column(type="string")
     * @var Rector\Tests\Renaming\Rector\Name\RenameClassRector\Source\OldClass
     */
    public $f;
}
-----
<?php

namespace App;

use Doctrine\ORM\Mapping as ORM;
use Symfony\Component\Validator\Constraints as Assert;

class A
{
    /**
     * @ORM\JoinTable(name="t", joinColumns={@ORM\OneToMany(targetEntity="Rector\Tests\Renaming\Rector\Name\RenameClassRector\Source\NewClass")})
     */
    public $a;

    /**
     * @ORM\JoinColumns({@ORM\JoinColumn(name="x"), {@ORM\ManyToMany(targetEntity="Rector\Tests\Renaming\Rector\Name\RenameClassRector\Source\OldClass", mappedBy="m")}})
     */
    public $b;

    /**
     * @ORM\Embedded(class="Rector\Tests\Renaming\Rector\Name\RenameClassRector\Source\OldClass", columnPrefix=false)
     */
    public $c;

    /**
     * @Assert\All({@Assert\Choice(callback={"Rector\Tests\Renaming\Rector\Name\RenameClassRector\Source\NewClass", "values"}, multiple=true)})
     */
    public $d;

    /**
     * @ORM\JoinTable(@ORM\OneToMany(targetEntity="Rector\Tests\Renaming\Rector\Name\RenameClassRector\Source\NewClass"))
     */
    public $e;

    /**
     * @ORM\Column(type="string")
     * @var \Rector\Tests\Renaming\Rector\Name\RenameClassRector\Source\NewClass
     */
    public $f;
}
