<?php

/*
 * This file is part of the Symfony package.
 *
 * (c) Fabien Potencier <fabien@symfony.com>
 *
 * For the full copyright and license information, please view the LICENSE
 * file that was distributed with this source code.
 */

namespace Symfony\Component\Console\ArgumentResolver\Exception;

final class ResolverNotFoundException extends \RuntimeException
{
    /**
     * @param string[] $alternatives
     */
    public function __construct(string $name, array $alternatives = [])
    {}
}
