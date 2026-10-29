<?php

namespace Rector\Tests\Renaming\Rector\Name\RenameClassRector;

namespace\Source\OldClass::x();
$a = new namespace\Source\OldClass();
namespace\Other::y();
-----
<?php

namespace Rector\Tests\Renaming\Rector\Name\RenameClassRector;

\Rector\Tests\Renaming\Rector\Name\RenameClassRector\Source\NewClass::x();
$a = new \Rector\Tests\Renaming\Rector\Name\RenameClassRector\Source\NewClass();
namespace\Other::y();
