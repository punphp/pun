<?php

namespace App;

use Rector\Tests\Php80\Rector\Class_\AnnotationToAttributeRector\Source\GenericAnnotation;

/**
 * @GenericAnnotation ({"a",})
 */
final class C1
{
}

/**
 * @GenericAnnotation ({"a", "b"
 * })
 */
final class C2
{
}

/**
 * @GenericAnnotation {"a",}
 */
final class C3
{
}

/**
 * @GenericAnnotation x={"a", "b"
 * }
 */
final class C4
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

#[GenericAnnotation(['a', 'b'])]
final class C2
{
}

#[GenericAnnotation]
final class C3
{
}

#[GenericAnnotation]
final class C4
{
}
