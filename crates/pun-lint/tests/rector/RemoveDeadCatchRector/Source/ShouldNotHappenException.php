<?php

// What Rector's own class (src/Exception/ShouldNotHappenException.php) declares, which its test run autoloads.
namespace Rector\Exception;

use Exception;

final class ShouldNotHappenException extends Exception
{
}
