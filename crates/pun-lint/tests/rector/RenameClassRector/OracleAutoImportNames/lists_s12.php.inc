<?php

namespace App;

use Foo\{Bar, function baz, const QUX};
use Foo\Baz as Q;

new \Other\Baz();
/** @var \Other\Baz */
$x = 1;
/** @var \Iterator */
$y = 2;
/** @var Some\Missing */
$z = 3;
new \Rector\Tests\Renaming\Rector\Name\RenameClassRector\Source\OldClass();
new Bar();
baz(QUX);
-----
<?php

namespace App;

use Rector\Tests\Renaming\Rector\Name\RenameClassRector\Source\NewClass;
use Iterator;
use Foo\{Bar, function baz, const QUX};

new \Other\Baz();
/** @var \Other\Baz */
$x = 1;
/** @var Iterator */
$y = 2;
/** @var Some\Missing */
$z = 3;
new NewClass();
new Bar();
baz(QUX);
