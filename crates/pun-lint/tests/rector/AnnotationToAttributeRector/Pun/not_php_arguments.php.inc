<?php

// Rector 2.6.1 prints these attributes as code PHP does not take: `array(1, 2)` reads as a nested annotation and
// gives `new array(1)` (`array()` gives `new array()`, `[array(1)]` gives `new [array(1)`), a key that is a number
// names an argument (`1: 'a'`, `foo: 'a', 1: 'b'` and in the nested `new GenericAnnotation(bar: 'a', 1: 'b')`), and a
// value without a key after a named one is a positional argument after a named one (`foo: 'a', 'b', c: 'd'`,
// `foo: 'a', 'b'` and `new GenericAnnotation(bar: 'a', 'b')`), a compile error. Pun leaves such a doc as it is.

namespace App;

use Doctrine\ORM\Mapping as ORM;
use Rector\Tests\Php80\Rector\Class_\AnnotationToAttributeRector\Source\GenericAnnotation;

/**
 * @GenericAnnotation(foo=array(1, 2))
 */
final class C1
{
}

/**
 * @GenericAnnotation("1"="a")
 */
final class C2
{
}

/**
 * @GenericAnnotation(foo="a", "1"="b")
 */
final class C3
{
}

/**
 * @GenericAnnotation(foo=@GenericAnnotation(bar="a", "1"="b"))
 */
final class C4
{
}

/**
 * @GenericAnnotation(foo="a", "b", c="d")
 */
final class C5
{
}

/**
 * @GenericAnnotation(foo="a", "b", "c")
 */
final class C6
{
}

/**
 * @GenericAnnotation(foo=@GenericAnnotation(bar="a", "b", "c"))
 */
final class C7
{
}

/**
 * @GenericAnnotation(foo=[array(1)])
 */
final class C8
{
}

/**
 * @GenericAnnotation(x=array())
 */
final class C9
{
}
