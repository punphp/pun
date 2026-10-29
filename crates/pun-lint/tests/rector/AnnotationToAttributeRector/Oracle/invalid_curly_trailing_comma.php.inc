<?php

namespace App;

use Rector\Tests\Php80\Rector\Class_\AnnotationToAttributeRector\Source\GenericAnnotation;

/**
 * @GenericAnnotation({"a",}, -)
 */
final class C1
{
}

/**
 * @GenericAnnotation({"a", "b",}, -)
 */
final class C2
{
}
-----
<?php

namespace App;

use Rector\Tests\Php80\Rector\Class_\AnnotationToAttributeRector\Source\GenericAnnotation;

#[GenericAnnotation(['a'])]
final class C1
{
}

#[GenericAnnotation(['a', 'b'], '}')]
final class C2
{
}
