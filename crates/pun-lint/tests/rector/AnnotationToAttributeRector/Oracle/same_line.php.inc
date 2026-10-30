<?php

namespace App;

use Rector\Tests\Php80\Rector\Class_\AnnotationToAttributeRector\Source\GenericAnnotation;

/**
 * @GenericAnnotation(a=1) @GenericAnnotation(b=2)
 */
final class C1
{
}

/**
 * @GenericAnnotation @GenericAnnotation
 */
final class C2
{
}

/**
 * text @GenericAnnotation(a=1)
 */
final class C3
{
}

/**
 * @GenericAnnotation(a=1) text @GenericAnnotation(b=2)
 */
final class C4
{
}

/**
 * @GenericAnnotation({@GenericAnnotation(a=1)}) @GenericAnnotation(b=2)
 */
final class C5
{
}

/**@GenericAnnotation(a=1)
 *@GenericAnnotation(b=2)
 */
final class C6
{
}
-----
<?php

namespace App;

use Rector\Tests\Php80\Rector\Class_\AnnotationToAttributeRector\Source\GenericAnnotation;

#[GenericAnnotation(a: 1)]
final class C1
{
}

#[GenericAnnotation]
final class C2
{
}

/**
 * text @GenericAnnotation(a=1)
 */
final class C3
{
}

/**
 * text @GenericAnnotation(b=2)
 */
#[GenericAnnotation(a: 1)]
final class C4
{
}

#[GenericAnnotation([new GenericAnnotation(a: 1)])]
final class C5
{
}

/**@GenericAnnotation(a=1)
 *@GenericAnnotation(b=2)
 */
final class C6
{
}
