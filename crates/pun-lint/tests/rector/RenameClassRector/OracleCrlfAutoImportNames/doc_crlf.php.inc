<?php

namespace App;

class A
{
    /**
     * @var \Rector\Tests\Renaming\Rector\Name\RenameClassRector\Source\OldClass some
     *      more text
     *
     * @Doctrine\ORM\Mapping\Column(name="a",
     *     type="b")
     */
    public $a;

    /**
     * @param \Rector\Tests\Renaming\Rector\Name\RenameClassRector\Source\OldClass $b first
     * second
     */
    public function f($b)
    {
    }
}
-----
<?php

namespace App;

use Rector\Tests\Renaming\Rector\Name\RenameClassRector\Source\NewClass;
use Doctrine\ORM\Mapping\Column;

class A
{
    /**
     * @var NewClass some
     *      more text
     *
     * @Column(name="a", type="b")
     */
    public $a;

    /**
     * @param NewClass $b first
     * second
     */
    public function f($b)
    {
    }
}
