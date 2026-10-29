<?php

namespace App;

/**
 * @\Doctrine\ORM\Mapping\Entity(repositoryClass="")
 * @Rector\Tests\Php80\Rector\Class_\AnnotationToAttributeRector\Source\Attribute\EmptyAttribute(foo="Some\Name")
 */
final class ClassReferenceEmpty
{
}
-----
<?php

namespace App;

#[\Doctrine\ORM\Mapping\Entity(repositoryClass: '')]
#[Rector\Tests\Php80\Rector\Class_\AnnotationToAttributeRector\Source\Attribute\EmptyAttribute(foo: \Some\Name::class)]
final class ClassReferenceEmpty
{
}
