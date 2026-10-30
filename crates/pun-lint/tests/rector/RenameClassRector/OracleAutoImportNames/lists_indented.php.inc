<?php

namespace App;

    class A
    {
        /**
         * @var \Rector\Tests\Renaming\Rector\Name\RenameClassRector\Source\OldClass
         */
        public $a;

        public function f()
        {
            $s = "a
    b";
            $h = <<<EOT
        x
            y
        EOT;
            return new \Rector\Tests\Renaming\Rector\Name\RenameClassRector\Source\OldClass();
        }
    }
-----
<?php

namespace App;

use Rector\Tests\Renaming\Rector\Name\RenameClassRector\Source\NewClass;

class A
{
    /**
     * @var NewClass
     */
    public $a;

    public function f()
    {
        $s = "a
    b";
        $h = <<<EOT
        x
            y
        EOT;
        return new NewClass();
    }
}
