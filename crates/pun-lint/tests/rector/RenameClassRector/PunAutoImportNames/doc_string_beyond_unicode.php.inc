<?php

// Rector 2.6.1 reprints the `@method` default "\u{110000}" as the bytes F4 90 80 80 (phpdoc-parser's
// `codePointToUtf8()` encodes the code point beyond U+10FFFF, and `escapeDoubleQuotedString()` lets them through),
// which makes the file no longer UTF-8. Pun keeps that string as written and prints the rest as Rector does.

namespace App;

/**
 * @method \Rector\Tests\Renaming\Rector\Name\RenameClassRector\Source\OldClass foo($a = "x\u{110000}", $b = array(1))
 */
class A
{
}
-----
<?php

// Rector 2.6.1 reprints the `@method` default "\u{110000}" as the bytes F4 90 80 80 (phpdoc-parser's
// `codePointToUtf8()` encodes the code point beyond U+10FFFF, and `escapeDoubleQuotedString()` lets them through),
// which makes the file no longer UTF-8. Pun keeps that string as written and prints the rest as Rector does.

namespace App;

use Rector\Tests\Renaming\Rector\Name\RenameClassRector\Source\OldClass;

/**
 * @method OldClass foo($a = "x\u{110000}", $b = [1])
 */
class A
{
}
