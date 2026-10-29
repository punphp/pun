<?php

namespace App;

use Rector\Tests\Php80\Rector\Class_\AnnotationToAttributeRector\Source\GenericAnnotation;

/**
 * @GenericAnnotation
 */
function some(
    /** @GenericAnnotation(x=1) */
    $value
) {
}

/**
 * @GenericAnnotation
 */
interface SomeInterface
{
}
-----
<?php

namespace App;

use Rector\Tests\Php80\Rector\Class_\AnnotationToAttributeRector\Source\GenericAnnotation;

#[GenericAnnotation]
function some(
    #[GenericAnnotation(x: 1)]
    $value
) {
}

#[GenericAnnotation]
interface SomeInterface
{
}
