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
 * Casts Fiber related classes to array representation.
 *
 * @author Grégoire Pineau <lyrixx@lyrixx.info>
 */
final class FiberCaster
{
    public static function castFiber(\Fiber $fiber, array $a, Stub $stub, bool $isNested, int $filter = 0): array
    {}
}
