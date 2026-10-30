<?php

namespace App;

use Rector\Tests\Php80\Rector\Class_\AnnotationToAttributeRector\Source\Annotation\OpenApi\PastAnnotation as PA;

/**
 * @PA
 */
final class Q
{
}
-----
<?php

namespace App;

use Rector\Tests\Php80\Rector\Class_\AnnotationToAttributeRector\Source\Attribute\OpenApi\FutureAttribute as PA;

#[PA\Rector\Tests\Php80\Rector\Class_\AnnotationToAttributeRector\Source\Attribute\OpenApi\FutureAttribute]
final class Q
{
}
