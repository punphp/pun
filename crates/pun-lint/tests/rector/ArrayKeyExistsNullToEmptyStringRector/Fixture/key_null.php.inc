<?php

namespace Rector\Tests\Php85\Rector\FuncCall\ArrayKeyExistsNullToEmptyStringRector\Fixture;

array_key_exists(null, $array);

?>
-----
<?php

namespace Rector\Tests\Php85\Rector\FuncCall\ArrayKeyExistsNullToEmptyStringRector\Fixture;

array_key_exists('', $array);

?>