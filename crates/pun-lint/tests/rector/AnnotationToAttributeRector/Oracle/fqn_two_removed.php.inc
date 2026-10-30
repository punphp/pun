<?php

namespace App;

final class P
{
    /**
     * Some text @\Rector\Tests\Php80\Rector\Class_\AnnotationToAttributeRector\Source\GenericAnnotation(a=1) @\Rector\Tests\Php80\Rector\Class_\AnnotationToAttributeRector\Source\GenericAnnotation(b=2)
     */
    public function run1(int $a): int
    {
        return $a;
    }

    /**
     * Some text
     * @\Rector\Tests\Php80\Rector\Class_\AnnotationToAttributeRector\Source\GenericAnnotation(a=1) @\Rector\Tests\Php80\Rector\Class_\AnnotationToAttributeRector\Source\GenericAnnotation(b=2) more
     */
    public function run2(int $a): int
    {
        return $a;
    }

    /**
     * @\Rector\Tests\Php80\Rector\Class_\AnnotationToAttributeRector\Source\GenericAnnotation(a=1) @\Rector\Tests\Php80\Rector\Class_\AnnotationToAttributeRector\Source\GenericAnnotation(b=2)
     */
    public function run3(int $a): int
    {
        return $a;
    }

    /**
     * Some text
     *
     * @\Rector\Tests\Php80\Rector\Class_\AnnotationToAttributeRector\Source\GenericAnnotation(a=1)
     * @\Rector\Tests\Php80\Rector\Class_\AnnotationToAttributeRector\Source\GenericAnnotation(b=2)
     */
    public function run4(int $a): int
    {
        return $a;
    }
}
-----
<?php

namespace App;

final class P
{
    /**
     * Some text @\Rector\Tests\Php80\Rector\Class_\AnnotationToAttributeRector\Source\GenericAnnotation(a=1) @\Rector\Tests\Php80\Rector\Class_\AnnotationToAttributeRector\Source\GenericAnnotation(b=2)
     */
    #[\Rector\Tests\Php80\Rector\Class_\AnnotationToAttributeRector\Source\GenericAnnotation(a: 1)]
    #[\Rector\Tests\Php80\Rector\Class_\AnnotationToAttributeRector\Source\GenericAnnotation(b: 2)]
    public function run1(int $a): int
    {
        return $a;
    }

    /**
     * Some text @\Rector\Tests\Php80\Rector\Class_\AnnotationToAttributeRector\Source\GenericAnnotation(b=2) more
     */
    #[\Rector\Tests\Php80\Rector\Class_\AnnotationToAttributeRector\Source\GenericAnnotation(a: 1)]
    public function run2(int $a): int
    {
        return $a;
    }

    #[\Rector\Tests\Php80\Rector\Class_\AnnotationToAttributeRector\Source\GenericAnnotation(a: 1)]
    public function run3(int $a): int
    {
        return $a;
    }

    /**
     * Some text
     */
    #[\Rector\Tests\Php80\Rector\Class_\AnnotationToAttributeRector\Source\GenericAnnotation(a: 1)]
    #[\Rector\Tests\Php80\Rector\Class_\AnnotationToAttributeRector\Source\GenericAnnotation(b: 2)]
    public function run4(int $a): int
    {
        return $a;
    }
}
