<?php

namespace App\H;

$s = <<<EOT
a

b
EOT;
$t = "x

y";
$a = new \Rector\Tests\Renaming\Rector\Name\RenameClassRector\Source\OldClass();
-----
<?php

namespace App\H;

use Rector\Tests\Renaming\Rector\Name\RenameClassRector\Source\NewClass;

$s = <<<EOT
a

b
EOT;
$t = "x

y";
$a = new NewClass();
