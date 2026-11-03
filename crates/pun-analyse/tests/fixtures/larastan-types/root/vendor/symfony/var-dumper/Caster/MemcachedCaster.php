<?php

/*
 * This file is part of the Symfony package.
 *
 * (c) Fabien Potencier <fabien@symfony.com>
 *
 * For the full copyright and license information, please view the LICENSE
 * file that was distributed with this source code.
 */

namespace Symfony\Component\VarDumper\Caster;

use Symfony\Component\VarDumper\Cloner\Stub;

/**
 * @author Jan Schädlich <jan.schaedlich@sensiolabs.de>
 *
 * @final
 *
 * @internal
 */
class MemcachedCaster
{
    private static array $optionConstants;
    private static array $defaultOptions;

    public static function castMemcached(\Memcached $c, array $a, Stub $stub, bool $isNested): array
    {}

    private static function getNonDefaultOptions(\Memcached $c): array
    {}

    private static function discoverDefaultOptions(): array
    {}

    private static function getOptionConstants(): array
    {}
}
