<?php

namespace App;

use JMS\Serializer\Annotation as Serializer;

class A
{
    /**
     * @Serializer\Type("array<xRector\Tests\Renaming\Rector\Name\RenameClassRector\Source\OldClassb>")
     */
    public $a;

    /**
     * @Serializer\Type("array<Rector\Tests\Renaming\Rector\Name\RenameClassRector\Source\OldClassb, xRector\Tests\Renaming\Rector\Name\RenameClassRector\Source\OldClassb>")
     */
    public $b;
}
-----
<?php

namespace App;

use JMS\Serializer\Annotation as Serializer;

class A
{
    /**
     * @Serializer\Type("array<xRector\Tests\Renaming\Rector\Name\RenameClassRector\Source\OldClassb>")
     */
    public $a;

    /**
     * @Serializer\Type("array<Rector\Tests\Renaming\Rector\Name\RenameClassRector\Source\NewClass, xRector\Tests\Renaming\Rector\Name\RenameClassRector\Source\OldClassb>")
     */
    public $b;
}
