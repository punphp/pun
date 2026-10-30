<?php
use Foo\Unused;

namespace\Foo\bar();
?>
<p><?= 1 ?></p>
<?php
new \Rector\Tests\Renaming\Rector\Name\RenameClassRector\Source\OldClass();
-----
<?php
namespace\Foo\bar();
?>
<p><?= 1 ?></p>
<?php
new \Rector\Tests\Renaming\Rector\Name\RenameClassRector\Source\NewClass();
