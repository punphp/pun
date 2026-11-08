<?php

namespace App;

use Rector\Tests\Php80\Rector\Class_\AnnotationToAttributeRector\Source\GenericAnnotation;

/**
 * @GenericAnnotation(tiny="1e-5", huge="1e20", third="0.30000000000000004", quote='it''s', list={@GenericAnnotation, "x"}, fetch="::X", bad=Foo::1, keyed={Foo::BAR = 1})
 */
final class ValuesMore
{
}
