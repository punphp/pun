<?php

namespace App;

use Rector\Tests\Php80\Rector\Class_\AnnotationToAttributeRector\Source\GenericAnnotation;

/**
 * @foo x={
 *
 * })
 * @GenericAnnotation(a=1)
 */
final class C1
{
}

/**
 * @foo x={
 *
 *   "a"})
 *
 * @GenericAnnotation(a=1)
 */
final class C2
{
}

/**
 * @GenericAnnotation(a=1)
 * @foo {
 *
 * })
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
 *
 * })
 */
#[GenericAnnotation(a: 1)]
final class C1
{
}

/**
 * @foo x={
 *
 *   "a"})
 */
#[GenericAnnotation(a: 1)]
final class C2
{
}

/**
 * @foo {
 *
 * })
 */
#[GenericAnnotation(a: 1)]
final class C3
{
}
