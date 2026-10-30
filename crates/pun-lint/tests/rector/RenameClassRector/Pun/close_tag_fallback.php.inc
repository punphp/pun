<?php

// Rector 2.6.1 falls back to printing the statements (the removed import sits after an open tag) and writes a second
// closing tag after the first statement, which ends with one, so the page shows it and a blank line before the HTML.
// Pun leaves the file alone.
new \Rector\Tests\Renaming\Rector\Name\RenameClassRector\Source\OldClass() ?>
<b>x</b>
<?php /* c */ use Foo\Unused;
new \Foo\Bar();
