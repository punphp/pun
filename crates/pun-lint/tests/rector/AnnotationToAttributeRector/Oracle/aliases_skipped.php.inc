<?php

namespace App;

use Rector\Tests\Php80\Rector\Class_\AnnotationToAttributeRector\Source\GenericAnnotation;
use Foo\Bar as Z;
use Rector\Tests\Php80\Rector\Class_\AnnotationToAttributeRector\Source\Annotation as O;
use Rector\Tests\Php80\Rector\Class_\AnnotationToAttributeRector\Source\Annotation\OpenApi\Annotation as OA;

final class AliasesSkipped
{
    /**
     * @OA\NestedPastAnnotation
     */
    public $value;
}
-----
<?php

namespace App;

use Rector\Tests\Php80\Rector\Class_\AnnotationToAttributeRector\Source\GenericAnnotation;
use Foo\Bar as Z;
use Rector\Tests\Php80\Rector\Class_\AnnotationToAttributeRector\Source\Annotation as O;
use Rector\Tests\Php80\Rector\Class_\AnnotationToAttributeRector\Source\Attribute\OpenApi\Attribute as OA;

final class AliasesSkipped
{
    #[OA\NestedFutureAttribute]
    public $value;
}
