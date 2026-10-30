<?php

namespace App;

use Rector\Tests\Php80\Rector\Class_\AnnotationToAttributeRector\Source\GenericAnnotation;

/**
 * @foo x={
 * @phpstan-type Foo int
 * })
 * @GenericAnnotation(a=1)
 */
final class C1
{
}

/**
 * @GenericAnnotation(a=1)
 * @foo x={
 * @phpstan-type Foo int
 * })
 * text
 */
final class C2
{
}

/**
 * @Foo\Bar({
 * @phpstan-type Foo int
 *   })
 * @GenericAnnotation(a=1)
 */
final class C3
{
}
-----
<?php

namespace App;

use Rector\Tests\Php80\Rector\Class_\AnnotationToAttributeRector\Source\GenericAnnotation;

/**
 * @foo x={
 * @phpstan-type Foo int
 * })
 * @GenericAnnotation(a=1)
 */
final class C1
{
}

/**
 * @foo x={
 * @phpstan-type Foo int
 * })
 * text
 * })
 * text
 */
#[GenericAnnotation(a: 1)]
final class C2
{
}

/**
 * @Foo\Bar({
 * @phpstan-type Foo int
 *   })
 */
#[GenericAnnotation(a: 1)]
final class C3
{
}
