<?php

namespace App;

use Rector\Tests\Php80\Rector\Class_\AnnotationToAttributeRector\Source\GenericAnnotation;

final class Tabs
{
	/**
	 * @GenericAnnotation
	 */
	public $value;

	public function f()
	{
		$x = function (
			/** @GenericAnnotation */
			$a
		) {};
	}

	/** @GenericAnnotation */
	public $kept;
}
-----
<?php

namespace App;

use Rector\Tests\Php80\Rector\Class_\AnnotationToAttributeRector\Source\GenericAnnotation;

final class Tabs
{
	#[GenericAnnotation]
    public $value;

	public function f()
	{
		$x = function (
			#[GenericAnnotation]
            $a
		) {};
	}

	#[GenericAnnotation]
    public $kept;
}
