<?php

namespace Rector\Tests\TypeDeclaration\Rector\ClassMethod\ReturnTypeFromStrictNewArrayRector\Oracle;

function collect(int $i)
{
    $values = [];
    $values[] = $i;

    return $values;
}
-----
<?php

namespace Rector\Tests\TypeDeclaration\Rector\ClassMethod\ReturnTypeFromStrictNewArrayRector\Oracle;

/**
 * @return list<int>
 */
function collect(int $i): array
{
    $values = [];
    $values[] = $i;

    return $values;
}
