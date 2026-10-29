<?php

// Rector 2.6.1 prints the first namespace, whose only statement goes, as `namespace App;` beside the braced
// `namespace B { }`, a fatal error (bracketed and unbracketed namespace declarations do not mix). Pun leaves the file
// alone.

namespace App {
    use Foo\Unused;
}

namespace B {
    use Foo\Unused2;

    new \Rector\Tests\Renaming\Rector\Name\RenameClassRector\Source\OldClass();
}
