<?php

namespace App;

use Rector\Tests\Php80\Rector\Class_\AnnotationToAttributeRector\Source\GenericAnnotation;

/**
 * @GenericAnnotation({
 *     @GenericAnnotation(a=1)
 * })
 */
final class C1
{
}

/**
 * @GenericAnnotation(x={
 *     "a",
 *     @GenericAnnotation(a=1),
 * })
 */
final class C2
{
}

/**
 * @GenericAnnotation(x={
 * @GenericAnnotation(a=1)
 * text })
 */
final class C3
{
}

/**
 * @GenericAnnotation(x={"a",
 *  })
 * @GenericAnnotation
 */
final class C4
{
}

/**
 * @GenericAnnotation(x={@GenericAnnotation(a="b
 * c")})
 */
final class C5
{
}

/**
 * @GenericAnnotation(x="a
 * b c", y="d")
 */
final class C6
{
}
-----
<?php

namespace App;

use Rector\Tests\Php80\Rector\Class_\AnnotationToAttributeRector\Source\GenericAnnotation;

#[GenericAnnotation([new GenericAnnotation(a: 1)])]
final class C1
{
}

#[GenericAnnotation(x: ['a', new GenericAnnotation(a: 1)])]
final class C2
{
}

/**
 * text })
 */
#[GenericAnnotation(x: [''])]
#[GenericAnnotation(a: 1)]
final class C3
{
}

#[GenericAnnotation(x: ['a'])]
#[GenericAnnotation]
final class C4
{
}

#[GenericAnnotation(x: [new GenericAnnotation(a: 'b
 c')])]
final class C5
{
}

#[GenericAnnotation(x: 'a
 b c', y: 'd')]
final class C6
{
}
