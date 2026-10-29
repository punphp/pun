<?php

namespace App\Kinds;

trait T
{
    public function a() {}
}

trait U
{
    public function a() {}
}

enum E: string implements \Countable
{
    case A = 'a';

    public function count(): int
    {
        return 0;
    }
}

final class K
{
    use T, U {
        T::a insteadof U;
    }
}

$x = new class extends \Rector\Tests\Renaming\Rector\Name\RenameClassRector\Source\OldClass implements \Rector\Tests\Renaming\Rector\Name\RenameClassRector\Source\Contract\FirstInterface {};
echo \Rector\Tests\Renaming\Rector\Name\RenameClassRector\Source\OldClass::$prop;
$f = \Rector\Tests\Renaming\Rector\Name\RenameClassRector\Source\OldClass::make(...);
$g = \strlen(...);
$h = \App\fn_name(...);
echo \App\CONSTANT;
-----
<?php

namespace App\Kinds;

use Countable;
use Rector\Tests\Renaming\Rector\Name\RenameClassRector\Source\NewClass;
use Rector\Tests\Renaming\Rector\Name\RenameClassRector\Source\Contract\FirstInterface;
use const App\CONSTANT;
use function App\fn_name;

trait T
{
    public function a() {}
}

trait U
{
    public function a() {}
}

enum E: string implements Countable
{
    case A = 'a';

    public function count(): int
    {
        return 0;
    }
}

final class K
{
    use T, U {
        T::a insteadof U;
    }
}

$x = new class extends NewClass implements FirstInterface {};
echo NewClass::$prop;
$f = NewClass::make(...);
$g = \strlen(...);
$h = fn_name(...);
echo CONSTANT;
