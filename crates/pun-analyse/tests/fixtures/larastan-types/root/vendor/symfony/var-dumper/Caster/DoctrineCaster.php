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

use Doctrine\Common\Proxy\Proxy as CommonProxy;
use Doctrine\ORM\PersistentCollection;
use Doctrine\ORM\Proxy\Proxy as OrmProxy;
use Symfony\Component\VarDumper\Cloner\Stub;

/**
 * Casts Doctrine related classes to array representation.
 *
 * @author Nicolas Grekas <p@tchwork.com>
 *
 * @final
 *
 * @internal
 */
class DoctrineCaster
{
    public static function castCommonProxy(CommonProxy $proxy, array $a, Stub $stub, bool $isNested): array
    {}

    public static function castOrmProxy(OrmProxy $proxy, array $a, Stub $stub, bool $isNested): array
    {}

    public static function castPersistentCollection(PersistentCollection $coll, array $a, Stub $stub, bool $isNested): array
    {}
}
