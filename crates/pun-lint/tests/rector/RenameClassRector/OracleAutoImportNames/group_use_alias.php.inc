<?php

namespace App;

use Foo\{Bar as Baz};

new Baz();
new \Other\Bar();
new \Rector\Tests\Renaming\Rector\Name\RenameClassRector\Source\OldClass();
-----
<?php

namespace App;

use Rector\Tests\Renaming\Rector\Name\RenameClassRector\Source\NewClass;
use Foo\{Bar as Baz};

new Baz();
new \Other\Bar();
new NewClass();
