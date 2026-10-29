<?php

namespace App;

class A
{
    /**
     * @var "a\x"|\Rector\Tests\Renaming\Rector\Name\RenameClassRector\Source\OldClass
     */
    public $a1;

    /**
     * @var "\400"|\Rector\Tests\Renaming\Rector\Name\RenameClassRector\Source\OldClass
     */
    public $a2;

    /**
     * @var "\u{}"|\Rector\Tests\Renaming\Rector\Name\RenameClassRector\Source\OldClass
     */
    public $a3;

    /**
     * @var "\xC3"|\Rector\Tests\Renaming\Rector\Name\RenameClassRector\Source\OldClass
     */
    public $a4;

    /**
     * @var 'a\\b'|\Rector\Tests\Renaming\Rector\Name\RenameClassRector\Source\OldClass
     */
    public $a5;

    /**
     * @var "\e\f"|\Rector\Tests\Renaming\Rector\Name\RenameClassRector\Source\OldClass
     */
    public $a6;

    /**
     * @var "\8"|\Rector\Tests\Renaming\Rector\Name\RenameClassRector\Source\OldClass
     */
    public $a7;
}
-----
<?php

namespace App;

use Rector\Tests\Renaming\Rector\Name\RenameClassRector\Source\NewClass;

class A
{
    /**
     * @var "a\\x"|NewClass
     */
    public $a1;

    /**
     * @var "\x00"|NewClass
     */
    public $a2;

    /**
     * @var "\\u{}"|NewClass
     */
    public $a3;

    /**
     * @var "\xc3"|NewClass
     */
    public $a4;

    /**
     * @var 'a\\b'|NewClass
     */
    public $a5;

    /**
     * @var "\x1b\f"|NewClass
     */
    public $a6;

    /**
     * @var "\\8"|NewClass
     */
    public $a7;
}
