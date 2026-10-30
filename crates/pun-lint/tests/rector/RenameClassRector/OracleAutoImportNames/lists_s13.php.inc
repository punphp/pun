<?php

namespace App;

/** a */
/** @var \Foo\B */
$x = 1;
$y = [/** @var \Foo\C */ 1];
/** */
$z = 2;
new \Rector\Tests\Renaming\Rector\Name\RenameClassRector\Source\OldClass();
/** @var \Foo\D */
-----
<?php

namespace App;

use Rector\Tests\Renaming\Rector\Name\RenameClassRector\Source\NewClass;

/** a */
/** @var \Foo\B */
$x = 1;
$y = [/** @var \Foo\C */ 1];
/** */
$z = 2;
new NewClass();
/** @var \Foo\D */
