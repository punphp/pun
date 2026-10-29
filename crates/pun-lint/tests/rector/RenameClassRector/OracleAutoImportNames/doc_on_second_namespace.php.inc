<?php

namespace App;

use Foo\Bar;

new Bar();

/** @var \Rector\Tests\Renaming\Rector\Name\RenameClassRector\Source\OldClass */
namespace Other;

// comment
use Foo\Baz;
-----
<?php

namespace App;

use Foo\Bar;

new Bar();

/** @var \Rector\Tests\Renaming\Rector\Name\RenameClassRector\Source\OldClass */
namespace Other;

// comment

