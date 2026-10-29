<?php

namespace App;

use Rector\Tests\Php80\Rector\Class_\AnnotationToAttributeRector\Source\GenericAnnotation;

/**
 * @GenericAnnotation(x={true::X=1})
 */
final class C1
{
}

/**
 * @GenericAnnotation(x="1.5e-5", y="-2.5e30", z="1234567.5")
 */
final class C2
{
}

/**
 * @GenericAnnotation(x=[1, [2]], y={null::Y: 2})
 */
final class C3
{
}
-----
<?php

namespace App;

use Rector\Tests\Php80\Rector\Class_\AnnotationToAttributeRector\Source\GenericAnnotation;

#[GenericAnnotation(x: [true::X => 1])]
final class C1
{
}

#[GenericAnnotation(x: 1.5E-5, y: -2.5E+30, z: 1234567.5)]
final class C2
{
}

#[GenericAnnotation(x: '[')]
final class C3
{
}
