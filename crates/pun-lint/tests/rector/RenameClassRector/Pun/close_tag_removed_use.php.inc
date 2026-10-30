<?php

// Rector 2.6.1 removes the unused import together with the closing tag that ends it, so the HTML after it becomes
// PHP code, a parse error. Pun leaves the file alone.
use Foo\Unused ?>
<b>x</b>
<?php
new \Rector\Tests\Renaming\Rector\Name\RenameClassRector\Source\OldClass();
