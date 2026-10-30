<?php

namespace App;

use Rector\Tests\Php80\Rector\Class_\AnnotationToAttributeRector\Source\GenericAnnotation;

$a = /** @GenericAnnotation */ function () {};
$b = array_map(
    /**
     * @GenericAnnotation(x=1)
     */
    fn ($v) => $v,
    []
);
$c = /**
 * text
 * @GenericAnnotation(y=2)
 * @return int
 */ static fn () => 1;
$d = /** @GenericAnnotation */ #[\SomeAttr] function () {};
$e = [
    /** @GenericAnnotation */
    'k' => function () {},
];
-----
<?php

namespace App;

use Rector\Tests\Php80\Rector\Class_\AnnotationToAttributeRector\Source\GenericAnnotation;

$a = /** @GenericAnnotation */ #[GenericAnnotation] function () {};
$b = array_map(
    /**
     * @GenericAnnotation(x=1)
     */
    fn ($v) => $v,
    []
);
$c = /**
 * text
 * @GenericAnnotation(y=2)
 * @return int
 */ #[GenericAnnotation(y: 2)] static fn () => 1;
$d = /** @GenericAnnotation */ #[\SomeAttr] #[GenericAnnotation] function () {};
$e = [
    /** @GenericAnnotation */
    'k' => function () {},
];
