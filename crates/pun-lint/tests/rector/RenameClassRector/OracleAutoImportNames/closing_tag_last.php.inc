<?php
use Foo\Unused;
new \Rector\Tests\Renaming\Rector\Name\RenameClassRector\Source\OldClass();
?>
-----
<?php
use Rector\Tests\Renaming\Rector\Name\RenameClassRector\Source\NewClass;
new NewClass();
?>
