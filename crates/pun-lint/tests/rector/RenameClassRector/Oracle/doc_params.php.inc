<?php

namespace App;

class A
{
    /**
     * @param \Rector\Tests\Renaming\Rector\Name\RenameClassRector\Source\OldClass $thisValue
     * @param-out \Rector\Tests\Renaming\Rector\Name\RenameClassRector\Source\OldClass $this
     * @param \Rector\Tests\Renaming\Rector\Name\RenameClassRector\Source\OldClass &...$a
     * @param $b
     */
    public function f($thisValue, ...$a) {}

    /**
     * @template T desc
     * more @\Doctrine\ORM\Mapping\Index(name="x") tail
     * @template
     * @return \Rector\Tests\Renaming\Rector\Name\RenameClassRector\Source\OldClass,x
     * @return \Rector\Tests\Renaming\Rector\Name\RenameClassRector\Source\OldClass
     */
    public function g() {}

    /**
     * @deprecated since
     * then @\Doctrine\ORM\Mapping\Index(name="y") end
     * @param-immediately-invoked-callable $a some
     * more @\Doctrine\ORM\Mapping\Index(name="z") end
     * @param $b some
     * more @\Doctrine\ORM\Mapping\Index(name="w") end
     * @return \Rector\Tests\Renaming\Rector\Name\RenameClassRector\Source\OldClass
     */
    public function h($a, $b) {}
}
-----
<?php

namespace App;

class A
{
    /**
     * @param \Rector\Tests\Renaming\Rector\Name\RenameClassRector\Source\NewClass $thisValue
     * @param-out \Rector\Tests\Renaming\Rector\Name\RenameClassRector\Source\OldClass $this
     * @param \Rector\Tests\Renaming\Rector\Name\RenameClassRector\Source\NewClass &...$a
     * @param $b
     */
    public function f($thisValue, ...$a) {}

    /**
     * @template T desc
     * more @\Doctrine\ORM\Mapping\Index(name="x") tail
     * @template
     * @return \Rector\Tests\Renaming\Rector\Name\RenameClassRector\Source\OldClass,x
     * @return \Rector\Tests\Renaming\Rector\Name\RenameClassRector\Source\NewClass
     * @template T desc
     * mor
     */
    public function g() {}

    /**
     * @deprecated since
     * then @\Doctrine\ORM\Mapping\Index(name="y") end
     * @param-immediately-invoked-callable $a some
     * more @\Doctrine\ORM\Mapping\Index(name="z") end
     * @\Doctrine\ORM\Mapping\Index(name="y")
     * @return \Rector\Tests\Renaming\Rector\Name\RenameClassRector\Source\NewClass
     * @\Doctrine\ORM\Mapping\Index(name="w")
     * @deprecated since
     * th
     * @param-immediately-invoked-callable $a some
     * mo
     * @\Doctrine\ORM\Mapping\Index(name="z")
     * @param $b some
     * mo
     */
    public function h($a, $b) {}
}
