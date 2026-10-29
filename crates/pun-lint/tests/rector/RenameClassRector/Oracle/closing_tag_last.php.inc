<?php
use Foo\Unused;
new \Rector\Tests\Renaming\Rector\Name\RenameClassRector\Source\OldClass();
?>
-----
<?php
new \Rector\Tests\Renaming\Rector\Name\RenameClassRector\Source\NewClass();
?>
