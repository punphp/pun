<?php
use Foo\Unused;
use Foo\Used;
?>
<b>html</b>
<?php
new Used();
new \Rector\Tests\Renaming\Rector\Name\RenameClassRector\Source\OldClass();
-----
<?php
use Foo\Used;
?>
<b>html</b>
<?php
new Used();
new \Rector\Tests\Renaming\Rector\Name\RenameClassRector\Source\NewClass();
