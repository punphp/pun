<?php

namespace App\CF;

use Foo\Kept;
use Foo\Gone;
use Foo\Grouped\{Inner};

final class Holder
{
    /**
     * @var \Abs::X
     */
    public $a;

    /**
     * @var Missing::Y
     */
    public $b;

    /**
     * @var Kept::Z|Inner::W
     */
    public $c;
}

$o = new \Rector\Tests\Renaming\Rector\Name\RenameClassRector\Source\OldClass();
-----
<?php

namespace App\CF;

use Foo\Kept;
use Foo\Grouped\{Inner};

final class Holder
{
    /**
     * @var \Abs::X
     */
    public $a;

    /**
     * @var Missing::Y
     */
    public $b;

    /**
     * @var Kept::Z|Inner::W
     */
    public $c;
}

$o = new \Rector\Tests\Renaming\Rector\Name\RenameClassRector\Source\NewClass();
