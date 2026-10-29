<b>head</b>
<?php
use Foo\Unused;
new \Rector\Tests\Renaming\Rector\Name\RenameClassRector\Source\OldClass();
?>
<i>tail</i>
-----
<?php

?>
<b>head</b>
<?php 
new \Rector\Tests\Renaming\Rector\Name\RenameClassRector\Source\NewClass();
?>
<i>tail</i>
<?php 
