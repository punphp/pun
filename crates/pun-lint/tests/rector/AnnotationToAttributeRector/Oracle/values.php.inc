<?php

namespace App;

use Rector\Tests\Php80\Rector\Class_\AnnotationToAttributeRector\Source\GenericAnnotation;

/**
 * @GenericAnnotation(ratio="1.5", big="1e3", list={2 = "a", "b", 5 = "c"}, fetch=Foo::bar, odd="Foo::1x", plain=SomeName, null=null, flag=false, quoted="it's", nl="a
 * b")
 */
final class Values
{
}
-----
<?php

namespace App;

use Rector\Tests\Php80\Rector\Class_\AnnotationToAttributeRector\Source\GenericAnnotation;

#[GenericAnnotation(ratio: 1.5, big: 1000, list: [2 => 'a', 'b', 5 => 'c'], fetch: Foo::bar, odd: 'Foo::1x', plain: 'SomeName', null: null, flag: false, quoted: "it's", nl: 'a
 b')]
final class Values
{
}
