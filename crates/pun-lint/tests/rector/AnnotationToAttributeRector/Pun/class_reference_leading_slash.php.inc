<?php

// Rector 2.6.1 makes `foo="\Some\Name"` of a class reference field `\\Some\Name::class` (it puts a `\` before the
// string's own), a parse error. Pun writes the class once fully qualified, as Rector does for a string without one.

namespace App;

/**
 * @Rector\Tests\Php80\Rector\Class_\AnnotationToAttributeRector\Source\Attribute\EmptyAttribute(foo="\Some\Name")
 */
final class ClassReferenceLeadingSlash
{
}
-----
<?php

// Rector 2.6.1 makes `foo="\Some\Name"` of a class reference field `\\Some\Name::class` (it puts a `\` before the
// string's own), a parse error. Pun writes the class once fully qualified, as Rector does for a string without one.

namespace App;

#[Rector\Tests\Php80\Rector\Class_\AnnotationToAttributeRector\Source\Attribute\EmptyAttribute(foo: \Some\Name::class)]
final class ClassReferenceLeadingSlash
{
}
