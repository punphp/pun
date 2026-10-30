<?php

namespace App;

use Rector\Tests\Php80\Rector\Class_\AnnotationToAttributeRector\Source\GenericAnnotation;

final class OtherComments
{
    // a comment
    /**
     * @GenericAnnotation
     */
    #[\Existing]
    public $value;

    /** @GenericAnnotation(a=1) */
    #[\Existing]
    public $other;
}
-----
<?php

namespace App;

use Rector\Tests\Php80\Rector\Class_\AnnotationToAttributeRector\Source\GenericAnnotation;

final class OtherComments
{
    // a comment
    #[\Existing]
    #[GenericAnnotation]
    public $value;

    #[\Existing]
    #[GenericAnnotation(a: 1)]
    public $other;
}
