<?php

namespace App;

use Rector\Tests\Php80\Rector\Class_\AnnotationToAttributeRector\Source\GenericAnnotation;

/**
 * @GenericAnnotation x={
 *
 * })
 */
final class C1
{
}

/**
 * @GenericAnnotation({@GenericAnnotation(a=1),
 *
 * @GenericAnnotation(b=2)
 *
 * })
 */
final class C2
{
}

/**
 * @GenericAnnotation({
 *
 *     @GenericAnnotation(a=1)
 *
 * })
 */
final class C3
{
}

/**
 * @GenericAnnotation({"a",
 *
 * "b"})
 */
final class C4
{
}
-----
<?php

namespace App;

use Rector\Tests\Php80\Rector\Class_\AnnotationToAttributeRector\Source\GenericAnnotation;

#[GenericAnnotation]
final class C1
{
}

#[GenericAnnotation([new GenericAnnotation(a: 1), new GenericAnnotation(b: 2)])]
final class C2
{
}

#[GenericAnnotation([new GenericAnnotation(a: 1)])]
final class C3
{
}

#[GenericAnnotation(['a', 'b'])]
final class C4
{
}
