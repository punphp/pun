<?php

namespace App;

use Doctrine\ORM\Mapping as ORM;
use Rector\Tests\Php80\Rector\Class_\AnnotationToAttributeRector\Source\GenericAnnotation;

/**
 * @GenericAnnotation(some='a\'b')
 */
final class C1
{
}

/**
 * @GenericAnnotation(foo=[1 => 2, 3])
 */
final class C2
{
}

/**
 * @GenericAnnotation(foo="a", "b")
 */
final class C3
{
}

/**
 * @GenericAnnotation(foo={@GenericAnnotation=1})
 */
final class C4
{
}

/**
 * @GenericAnnotation(foo=)
 */
final class C5
{
}

/**
 * @GenericAnnotation(foo=Foo::BAR::BAZ)
 */
final class C6
{
}

/**
 * @GenericAnnotation(foo={"x"="a", "b"})
 */
final class C7
{
}

/**
 * @GenericAnnotation("a", "b")
 */
final class C8
{
}

/**
 * @GenericAnnotation(foo=[1, 2])
 */
final class C9
{
}

/**
 * @GenericAnnotation(foo={1 => 2})
 */
final class C10
{
}

/**
 * @GenericAnnotation(foo={[1 => 2]})
 */
final class C11
{
}

/**
 * @GenericAnnotation("a" 
 */
final class C12
{
}
-----
<?php

namespace App;

use Doctrine\ORM\Mapping as ORM;
use Rector\Tests\Php80\Rector\Class_\AnnotationToAttributeRector\Source\GenericAnnotation;

#[GenericAnnotation(some: "a\\'b")]
final class C1
{
}

#[GenericAnnotation(foo: '[')]
final class C2
{
}

#[GenericAnnotation(foo: 'a')]
final class C3
{
}

#[GenericAnnotation(foo: ['@GenericAnnotation' => 1])]
final class C4
{
}

#[GenericAnnotation(foo: ')')]
final class C5
{
}

#[GenericAnnotation(foo: 'Foo::BAR::BAZ')]
final class C6
{
}

#[GenericAnnotation(foo: ['x' => 'a', 'b'])]
final class C7
{
}

#[GenericAnnotation('a')]
final class C8
{
}

#[GenericAnnotation(foo: '[')]
final class C9
{
}

#[GenericAnnotation(foo: [1])]
final class C10
{
}

#[GenericAnnotation(foo: ['['])]
final class C11
{
}

#[GenericAnnotation('a')]
final class C12
{
}
