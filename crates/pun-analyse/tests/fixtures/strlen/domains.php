<?php

namespace StrlenReturns;

use function PHPStan\dumpType;

/**
 * @param 'a'|'bb' $contiguous
 * @param 'a'|'ccc' $holes
 * @param 'a'|'b' $duplicate
 * @param 1|2|5|10|123|'1234'|false $scalars
 * @param ''|false|null $empty
 * @param ''|bool|null $boolean
 * @param non-empty-string|int|float $positive
 * @param non-empty-string|false $possiblyEmpty
 * @param 'abc'|int $broad
 * @param int<9, 11> $finiteRange
 * @param int<100, 999> $sameLengthRange
 * @param non-empty-string $nonempty
 * @param non-falsy-string $truthy
 * @param numeric-string $numeric
 * @param class-string $class
 * @param literal-string $literal
 * @param lowercase-string $lowercase
 */
function domains($contiguous, $holes, $duplicate, $scalars, $empty, $boolean, $positive,
    $possiblyEmpty, $broad, $finiteRange, $sameLengthRange, $nonempty, $truthy,
    $numeric, $class, $literal, $lowercase, bool $bool, int $int, float $float,
    string $string, mixed $mixed, array $array, \stdClass $object): void
{
    dumpType(strlen($contiguous));
    dumpType(strlen($holes));
    dumpType(strlen($duplicate));
    dumpType(strlen($scalars));
    dumpType(strlen($empty));
    dumpType(strlen($boolean));
    dumpType(strlen($positive));
    dumpType(strlen($possiblyEmpty));
    dumpType(strlen($broad));
    dumpType(strlen($finiteRange));
    dumpType(strlen($sameLengthRange));
    dumpType(strlen($nonempty));
    dumpType(strlen($truthy));
    dumpType(strlen($numeric));
    dumpType(strlen($class));
    dumpType(strlen($literal));
    dumpType(strlen($lowercase));
    dumpType(strlen($bool));
    dumpType(strlen($int));
    dumpType(strlen($float));
    dumpType(strlen($string));
    dumpType(strlen($mixed));
    dumpType(strlen($array));
    dumpType(strlen($object));
}

// Grouping bounds docblock type-construction recursion, not this extension's 129-scalar extraction.
/** @param (100|101|102|103|104|105|106|107|108|109|110|111|112|113|114|115)|(116|117|118|119|120|121|122|123|124|125|126|127|128|129|130|131)|(132|133|134|135|136|137|138|139|140|141|142|143|144|145|146|147)|(148|149|150|151|152|153|154|155|156|157|158|159|160|161|162|163)|(164|165|166|167|168|169|170|171|172|173|174|175|176|177|178|179)|(180|181|182|183|184|185|186|187|188|189|190|191|192|193|194|195)|(196|197|198|199|200|201|202|203|204|205|206|207|208|209|210|211)|(212|213|214|215|216|217|218|219|220|221|222|223|224|225|226|227)|228 $value */
function moreThan128Scalars($value): void
{
    dumpType(strlen($value));
}
