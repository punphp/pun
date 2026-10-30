<?php

namespace App;

use Rector\Tests\Php80\Rector\Class_\AnnotationToAttributeRector\Source\GenericAnnotation;

final class Tabs
{
	/**
	 * @GenericAnnotation
	 */
	public $value;
}
-----
<?php

namespace App;

use Rector\Tests\Php80\Rector\Class_\AnnotationToAttributeRector\Source\GenericAnnotation;

final class Tabs
{
	#[GenericAnnotation]
    public $value;
}
