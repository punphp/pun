<?php

namespace App;

use Rector\Tests\Php80\Rector\Class_\AnnotationToAttributeRector\Source\GenericAnnotation;

/**
 * @GenericAnnotation(x="a""b c")
 */
final class C1
{
}

/**
 * @GenericAnnotation(x="a\" b")
 */
final class C2
{
}

/**
 * @GenericAnnotation(x=\"a b")
 */
final class C3
{
}

/**
 * @GenericAnnotation(x="a" "b c")
 */
final class C4
{
}

/**
 * @GenericAnnotation(x='a b')
 */
final class C5
{
}

/**
 * @GenericAnnotation(x=[])
 */
final class C6
{
}

/**
 * @GenericAnnotation({"a",})
 */
final class C7
{
}

/**
 * @GenericAnnotation({"a", "b",})
 */
final class C8
{
}

/**
 * @GenericAnnotation(x={"a"}, y="a b
 * c")
 */
final class C9
{
}

/**
 * @GenericAnnotation(x=a"b c")
 */
final class C10
{
}

/**
 * @GenericAnnotation(x=a b")
 */
final class C11
{
}
-----
<?php

namespace App;

use Rector\Tests\Php80\Rector\Class_\AnnotationToAttributeRector\Source\GenericAnnotation;

#[GenericAnnotation(x: 'a')]
final class C1
{
}

#[GenericAnnotation(x: '"a\""b')]
final class C2
{
}

#[GenericAnnotation(x: '\"ab')]
final class C3
{
}

#[GenericAnnotation(x: 'a')]
final class C4
{
}

#[GenericAnnotation(x: 'a b')]
final class C5
{
}

#[GenericAnnotation(x: '[')]
final class C6
{
}

#[GenericAnnotation(['a'])]
final class C7
{
}

#[GenericAnnotation(['a', 'b'])]
final class C8
{
}

#[GenericAnnotation(x: ['a'], y: 'a b
 c')]
final class C9
{
}

#[GenericAnnotation(x: 'a')]
final class C10
{
}

#[GenericAnnotation(x: 'ab')]
final class C11
{
}
