<?php

namespace App;

use Rector\Tests\Php80\Rector\Class_\AnnotationToAttributeRector\Source\GenericAnnotation;

/**
 * @GenericAnnotation({"a", "b"
 * })
 */
final class C1
{
}

/**
 * @GenericAnnotation({"a",
 * })
 */
final class C2
{
}

/**
 * @GenericAnnotation(x={"a", "b"
 * }, y="c")
 */
final class C3
{
}
-----
<?php

namespace App;

use Rector\Tests\Php80\Rector\Class_\AnnotationToAttributeRector\Source\GenericAnnotation;

#[GenericAnnotation(['a', 'b'])]
final class C1
{
}

#[GenericAnnotation(['a'])]
final class C2
{
}

#[GenericAnnotation(x: ['a', 'b'], y: 'c')]
final class C3
{
}
