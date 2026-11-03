<?php

/*
 * This file is part of the Symfony package.
 *
 * (c) Fabien Potencier <fabien@symfony.com>
 *
 * For the full copyright and license information, please view the LICENSE
 * file that was distributed with this source code.
 */

namespace Symfony\Component\ErrorHandler;

use Symfony\Component\ErrorHandler\Exception\SilencedErrorContext;

/**
 * @internal
 */
class ThrowableUtils
{
    public static function getSeverity(SilencedErrorContext|\Throwable $throwable): int
    {}
}
