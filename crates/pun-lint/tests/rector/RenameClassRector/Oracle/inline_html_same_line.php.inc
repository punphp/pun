<?php
use Foo\Unused;
?><b>x</b><?php
new \Rector\Tests\Renaming\Rector\Name\RenameClassRector\Source\OldClass();
-----
<?php

?><b>x</b><?php 
new \Rector\Tests\Renaming\Rector\Name\RenameClassRector\Source\NewClass();
