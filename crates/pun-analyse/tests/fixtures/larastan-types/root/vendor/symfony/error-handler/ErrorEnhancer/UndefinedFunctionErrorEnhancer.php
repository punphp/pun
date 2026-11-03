<?php

/*
 * This file is part of the Symfony package.
 *
 * (c) Fabien Potencier <fabien@symfony.com>
 *
 * For the full copyright and license information, please view the LICENSE
 * file that was distributed with this source code.
 */

namespace Symfony\Component\ErrorHandler\ErrorEnhancer;

use Symfony\Component\ErrorHandler\Error\FatalError;
use Symfony\Component\ErrorHandler\Error\UndefinedFunctionError;

/**
 * @author Fabien Potencier <fabien@symfony.com>
 */
class UndefinedFunctionErrorEnhancer implements ErrorEnhancerInterface
{
    public function enhance(\Throwable $error): ?\Throwable
    {}
}
