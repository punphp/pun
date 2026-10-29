<?php

// Rector 2.6.1 reprints the `@var` string "\u{D800}" as the bytes ED A0 80 (phpdoc-parser's `codePointToUtf8()`
// encodes the surrogate, and `escapeDoubleQuotedString()` lets them through), which makes the file no longer UTF-8.
// Pun leaves the file alone.

namespace App;

class A
{
    /**
     * @var "\u{D800}"|\Rector\Tests\Renaming\Rector\Name\RenameClassRector\Source\OldClass
     */
    public $a;
}
