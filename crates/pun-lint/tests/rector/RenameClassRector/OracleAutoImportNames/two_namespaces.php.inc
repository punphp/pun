<?php

namespace App\One {
    $a = new \Rector\Tests\Renaming\Rector\Name\RenameClassRector\Source\OldClass();
}

namespace App\Two {
    $b = new \Rector\Tests\Renaming\Rector\Name\RenameClassRector\Source\OldClass();
}
-----
<?php

namespace App\One {
    $a = new \Rector\Tests\Renaming\Rector\Name\RenameClassRector\Source\NewClass();
}

namespace App\Two {
    $b = new \Rector\Tests\Renaming\Rector\Name\RenameClassRector\Source\NewClass();
}
