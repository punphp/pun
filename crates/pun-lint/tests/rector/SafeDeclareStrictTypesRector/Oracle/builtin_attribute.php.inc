<?php

namespace Pun\Oracle\SafeStrict;

#[\Attribute]
class A
{
}
-----
<?php

declare(strict_types=1);

namespace Pun\Oracle\SafeStrict;

#[\Attribute]
class A
{
}
