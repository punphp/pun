<?php
use Foo\Unused;
?>
<b>html</b>
<?php
new \Rector\Tests\Renaming\Rector\Name\RenameClassRector\Source\OldClass();
-----
<?php

?>
<b>html</b>
<?php 
new \Rector\Tests\Renaming\Rector\Name\RenameClassRector\Source\NewClass();
