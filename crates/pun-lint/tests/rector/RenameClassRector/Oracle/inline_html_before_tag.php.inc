<b>x</b><?php
use Foo\Unused;
new \Rector\Tests\Renaming\Rector\Name\RenameClassRector\Source\OldClass();
-----
<?php

?>
<b>x</b><?php 
new \Rector\Tests\Renaming\Rector\Name\RenameClassRector\Source\NewClass();
