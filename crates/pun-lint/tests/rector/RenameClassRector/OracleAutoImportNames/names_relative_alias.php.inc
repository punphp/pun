<?php

namespace Bar;

use Foo\Bar;

new Bar\Baz\Qux();
new \Rector\Tests\Renaming\Rector\Name\RenameClassRector\Source\OldClass();
-----
<?php

namespace Bar;

use Foo\Bar\Baz\Qux;
use Rector\Tests\Renaming\Rector\Name\RenameClassRector\Source\NewClass;
new Qux();
new NewClass();
