<?php

namespace App;

/**
 * @method \Rector\Tests\Renaming\Rector\Name\RenameClassRector\Source\OldClass foo($a = false, $b = array(1, 2), $c = [1 => 2, 'k' => "v"], $d = "\r\f", $e = \Foo::BAR)
 */
class A
{
}
-----
<?php

namespace App;

use Rector\Tests\Renaming\Rector\Name\RenameClassRector\Source\OldClass;

/**
 * @method OldClass foo($a = false, $b = [1, 2], $c = [1 => 2, 'k' => "v"], $d = "\r\f", $e = \Foo::BAR)
 */
class A
{
}
