<?php

namespace Rector\Tests\DeadCode\Rector\FunctionLike\NarrowWideUnionReturnTypeRector\Oracle;

$object = new class {
    public function run(): string|int|null
    {
        return 'text';
    }
};
-----
<?php

namespace Rector\Tests\DeadCode\Rector\FunctionLike\NarrowWideUnionReturnTypeRector\Oracle;

$object = new class {
    public function run(): string
    {
        return 'text';
    }
};
