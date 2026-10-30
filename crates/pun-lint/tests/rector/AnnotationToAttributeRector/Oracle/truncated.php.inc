<?php

namespace App;

use Rector\Tests\Php80\Rector\Class_\AnnotationToAttributeRector\Source\GenericAnnotation;

/**
 * @GenericAnnotation(x={"a", "b"
 */
final class C1
{
}

/**
 * @GenericAnnotation(x={"a",
 */
final class C2
{
}

/**
 * @GenericAnnotation(x={"a"
 */
final class C3
{
}

/**
 * @GenericAnnotation(x={
 */
final class C4
{
}

/**
 * @GenericAnnotation(x=
 */
final class C5
{
}

/**
 * @GenericAnnotation({"a", "b"})
 */
final class C6
{
}

/**
 * @GenericAnnotation(x={"a" "b"})
 */
final class C7
{
}

/**
 * @GenericAnnotation(x="a b c" d)
 */
final class C8
{
}

/**
 * @GenericAnnotation(x="a "b" c")
 */
final class C9
{
}
-----
<?php

namespace App;

use Rector\Tests\Php80\Rector\Class_\AnnotationToAttributeRector\Source\GenericAnnotation;

#[GenericAnnotation(x: ['a', 'b'])]
final class C1
{
}

#[GenericAnnotation(x: ['a', ''])]
final class C2
{
}

#[GenericAnnotation(x: ['a'])]
final class C3
{
}

#[GenericAnnotation(x: [''])]
final class C4
{
}

#[GenericAnnotation(x: '')]
final class C5
{
}

#[GenericAnnotation(['a', 'b'])]
final class C6
{
}

#[GenericAnnotation(x: ['a'])]
final class C7
{
}

#[GenericAnnotation(x: '"a b c"d')]
final class C8
{
}

#[GenericAnnotation(x: '"a "b')]
final class C9
{
}
