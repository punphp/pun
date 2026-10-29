<?php

namespace App;

use OldName1 as O1;

final class AliasOfSinglePart
{
    /**
     * @O1()
     */
    public $value;

    /**
     * @O1
     */
    public $other;
}
-----
<?php

namespace App;

use OldName1 as O1;

final class AliasOfSinglePart
{
    #[\Rector\Tests\Php80\Rector\Class_\AnnotationToAttributeRector\Source\Attribute\NewName1]
    public $value;

    /**
     * @O1
     */
    public $other;
}
