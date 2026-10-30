<?php

namespace App;

class A
{
    /**
     * @var array{"a\nb": \Rector\Tests\Renaming\Rector\Name\RenameClassRector\Source\OldClass, 'c\'d\\e': int, "f\x41\u{1F600}\101\$": int, "g\xff\xc3": int}|"t\tu"|'v\w'
     */
    public $a;

    /**
     * @method \Rector\Tests\Renaming\Rector\Name\RenameClassRector\Source\OldClass foo($a = "x\ny", $b = 'p\'q', $c = "\e\v")
     */
    public $b;

    /**
     * @var object{"k\n": \Rector\Tests\Renaming\Rector\Name\RenameClassRector\Source\OldClass, 'j': int}
     */
    public $c;
}
-----
<?php

namespace App;

class A
{
    /**
     * @var array{"a\nb": \Rector\Tests\Renaming\Rector\Name\RenameClassRector\Source\NewClass, 'c\'d\\e': int, "fA😀A\\\$": int, "g\xff\xc3": int}|"t\tu"|'v\\w'
     */
    public $a;

    /**
     * @method \Rector\Tests\Renaming\Rector\Name\RenameClassRector\Source\OldClass foo($a = "x\ny", $b = 'p\'q', $c = "\e\v")
     */
    public $b;

    /**
     * @var object{"k\n": \Rector\Tests\Renaming\Rector\Name\RenameClassRector\Source\NewClass, 'j': int}
     */
    public $c;
}
