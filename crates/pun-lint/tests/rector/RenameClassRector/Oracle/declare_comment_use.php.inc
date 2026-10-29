<?php

declare(strict_types=1);

// c
use Foo\Bar as Baz;

new \Rector\Tests\Renaming\Rector\Name\RenameClassRector\Source\OldClass();
new Baz();
-----
<?php

declare(strict_types=1);

// c
use Foo\Bar as Baz;

new \Rector\Tests\Renaming\Rector\Name\RenameClassRector\Source\NewClass();
new Baz();
