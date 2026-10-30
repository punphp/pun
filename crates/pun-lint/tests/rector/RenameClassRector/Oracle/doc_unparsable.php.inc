<?php

namespace App;

class A
{
    /**
     * @Doctrine\ORM\Mapping\Column(name="abc def)
     * @var \Rector\Tests\Renaming\Rector\Name\RenameClassRector\Source\OldClass
     */
    public $a;
}
