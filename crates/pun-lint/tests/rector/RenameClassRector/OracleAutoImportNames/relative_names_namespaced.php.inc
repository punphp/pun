<?php

namespace Rector\Tests\Renaming\Rector\Name\RenameClassRector;

namespace\Source\OldClass::x();
$a = new namespace\Source\OldClass();
namespace\Other::y();
-----
<?php

namespace Rector\Tests\Renaming\Rector\Name\RenameClassRector;

use Rector\Tests\Renaming\Rector\Name\RenameClassRector\Source\NewClass;

NewClass::x();
$a = new NewClass();
namespace\Other::y();
