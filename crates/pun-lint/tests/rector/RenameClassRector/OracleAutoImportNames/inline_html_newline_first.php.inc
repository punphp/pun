
<?php
use Foo\Unused;
new \Rector\Tests\Renaming\Rector\Name\RenameClassRector\Source\OldClass();
?>
<i>tail</i>
-----
<?php

?>

<?php 
new \Rector\Tests\Renaming\Rector\Name\RenameClassRector\Source\NewClass();
?>
<i>tail</i>
<?php 
