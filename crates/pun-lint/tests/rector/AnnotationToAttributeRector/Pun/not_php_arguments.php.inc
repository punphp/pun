<?php

// Rector 2.6.1 prints these attributes as code PHP does not parse: `array(1, 2)` reads as a nested annotation and
// gives `new array(1)`, and a key that is a number names an argument (`1: 'a'`, `foo: 'a', 1: 'b'` and in the nested
// `new GenericAnnotation(bar: 'a', 1: 'b')`). Pun leaves such a doc as it is.

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
