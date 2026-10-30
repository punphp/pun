<?php

namespace App;

use Rector\Tests\Php80\Rector\Class_\AnnotationToAttributeRector\Source\GenericAnnotation;

/**
 * @GenericAnnotation(some={@GenericAnnotation(other="x")})
 */
final class NestedStays
{
}

/**
 * @GenericAnnotation(key="kept")
 */
final class Converted
{
}
-----
<?php

namespace App;

use Rector\Tests\Php80\Rector\Class_\AnnotationToAttributeRector\Source\GenericAnnotation;

/**
 * @GenericAnnotation(some={@GenericAnnotation(other="x")})
 */
final class NestedStays
{
}

#[GenericAnnotation(key: 'kept')]
final class Converted
{
}
