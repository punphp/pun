<?php

namespace App;

use Rector\Tests\Php80\Rector\Class_\AnnotationToAttributeRector\Source\GenericAnnotation;

/**
 * @GenericAnnotation(some={@GenericAnnotation(other="x")})
 * @GenericAnnotation(key="kept")
 */
final class NestedStays
{
}
-----
<?php

namespace App;

use Rector\Tests\Php80\Rector\Class_\AnnotationToAttributeRector\Source\GenericAnnotation;

#[GenericAnnotation(some: [new GenericAnnotation(other: 'x')])]
#[GenericAnnotation(key: 'kept')]
final class NestedStays
{
}
