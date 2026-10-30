<?php

namespace App;

use Doctrine\ORM\Mapping as ORM;
use Rector\Tests\Php80\Rector\Class_\AnnotationToAttributeRector\Source\GenericAnnotation;

/**
 * @GenericAnnotation(foo="a", 1="b")
 */
final class C1
{
}
