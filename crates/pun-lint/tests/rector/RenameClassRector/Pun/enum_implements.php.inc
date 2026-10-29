<?php

// Rector 2.6.1 writes `enum E: string implements \stdClass`, a fatal error (an enum implements only interfaces):
// its class-to-interface check looks at a class's `implements` only. Pun keeps `\Countable`, the rest as Rector wrote it.

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

// Rector 2.6.1 writes `enum E: string implements \stdClass`, a fatal error (an enum implements only interfaces):
// its class-to-interface check looks at a class's `implements` only. Pun keeps `\Countable`, the rest as Rector wrote it.

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

$x = new class extends \Rector\Tests\Renaming\Rector\Name\RenameClassRector\Source\NewClass implements \Rector\Tests\Renaming\Rector\Name\RenameClassRector\Source\Contract\ThirdInterface {};
echo \Rector\Tests\Renaming\Rector\Name\RenameClassRector\Source\NewClass::$prop;
$f = \Rector\Tests\Renaming\Rector\Name\RenameClassRector\Source\NewClass::make(...);
$g = \strlen(...);
$h = \App\fn_name(...);
echo \App\CONSTANT;
