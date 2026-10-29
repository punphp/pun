<?php

namespace App\Ser;

use JMS\Serializer\Annotation as Serializer;
use Doctrine\ORM\Mapping as ORM;
use Symfony\Component\Validator\Constraints as Assert;

final class S
{
    /**
     * @Serializer\Type("array<Rector\Tests\Renaming\Rector\Name\RenameClassRector\Source\OldClassb>")
     */
    public $a;

    /**
     * @Serializer\Type(name="x", type="Rector\Tests\Renaming\Rector\Name\RenameClassRector\Source\OldClass")
     */
    public $b;

    /**
     * @ORM\Embedded(class="Rector\Tests\Renaming\Rector\Name\RenameClassRector\Source\OldClass")
     */
    public $c;

    /**
     * @Assert\Choice(callback={"Rector\Tests\Renaming\Rector\Name\RenameClassRector\Source\OldClass", "values"})
     */
    public $d;

    /**
     * @ORM\ManyToMany(targetEntity="\Rector\Tests\Renaming\Rector\Name\RenameClassRector\Source\OldClass")
     */
    public $e;
}
-----
<?php

namespace App\Ser;

use JMS\Serializer\Annotation as Serializer;
use Doctrine\ORM\Mapping as ORM;
use Symfony\Component\Validator\Constraints as Assert;

final class S
{
    /**
     * @Serializer\Type("array<Rector\Tests\Renaming\Rector\Name\RenameClassRector\Source\NewClass>")
     */
    public $a;

    /**
     * @Serializer\Type(name="x", type="Rector\Tests\Renaming\Rector\Name\RenameClassRector\Source\OldClass")
     */
    public $b;

    /**
     * @ORM\Embedded(class="Rector\Tests\Renaming\Rector\Name\RenameClassRector\Source\OldClass")
     */
    public $c;

    /**
     * @Assert\Choice(callback={"Rector\Tests\Renaming\Rector\Name\RenameClassRector\Source\NewClass", "values"})
     */
    public $d;

    /**
     * @ORM\ManyToMany(targetEntity="\Rector\Tests\Renaming\Rector\Name\RenameClassRector\Source\OldClass")
     */
    public $e;
}
