<?php

namespace App {
    use Rector\Tests\Php80\Rector\Class_\AnnotationToAttributeRector\Source\Annotation\OpenApi as OA;

    final class First
    {
        /**
         * @OA\PastAnnotation
         */
        public $value;
    }
}

namespace Other {
    /**
     * @Rector\Tests\Php80\Rector\Class_\AnnotationToAttributeRector\Source\GenericAnnotation
     */
    final class Second
    {
    }
}
-----
<?php

namespace App {
    use Rector\Tests\Php80\Rector\Class_\AnnotationToAttributeRector\Source\Annotation\OpenApi as OA;

    final class First
    {
        /**
         * @OA\PastAnnotation
         */
        public $value;
    }
}

namespace Other {
    #[Rector\Tests\Php80\Rector\Class_\AnnotationToAttributeRector\Source\GenericAnnotation]
    final class Second
    {
    }
}
