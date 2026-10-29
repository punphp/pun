<?php

namespace App;

use Rector\Tests\Php80\Rector\Class_\AnnotationToAttributeRector\Source\GenericAnnotation;

/**
 * @GenericAnnotation(list={@GenericAnnotation, "x"}, fetch="::X", bad=Foo::1, keyed={Foo::BAR = 1}, apos="it's")
 */
final class ValuesList
{
}

/**
 * @GenericAnnotation(nested={@GenericAnnotation(a=1)})
 */
final class NestedRemoved
{
}
-----
<?php

namespace App;

use Rector\Tests\Php80\Rector\Class_\AnnotationToAttributeRector\Source\GenericAnnotation;

#[GenericAnnotation(list: [new GenericAnnotation(), 'x'], fetch: '::X', bad: 'Foo::')]
final class ValuesList
{
}

#[GenericAnnotation(nested: [new GenericAnnotation(a: 1)])]
final class NestedRemoved
{
}
