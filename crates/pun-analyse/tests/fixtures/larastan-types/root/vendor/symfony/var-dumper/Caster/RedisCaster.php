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

use Relay\Relay;
use Symfony\Component\VarDumper\Cloner\Stub;

/**
 * Casts Redis class from ext-redis to array representation.
 *
 * @author Nicolas Grekas <p@tchwork.com>
 *
 * @final
 *
 * @internal
 */
class RedisCaster
{
    private const SERIALIZERS = [
        0 => 'NONE', // Redis::SERIALIZER_NONE
        1 => 'PHP', // Redis::SERIALIZER_PHP
        2 => 'IGBINARY', // Optional Redis::SERIALIZER_IGBINARY
    ];

    private const MODES = [
        0 => 'ATOMIC', // Redis::ATOMIC
        1 => 'MULTI', // Redis::MULTI
        2 => 'PIPELINE', // Redis::PIPELINE
    ];

    private const COMPRESSION_MODES = [
        0 => 'NONE', // Redis::COMPRESSION_NONE
        1 => 'LZF',  // Redis::COMPRESSION_LZF
    ];

    private const FAILOVER_OPTIONS = [
        \RedisCluster::FAILOVER_NONE => 'NONE',
        \RedisCluster::FAILOVER_ERROR => 'ERROR',
        \RedisCluster::FAILOVER_DISTRIBUTE => 'DISTRIBUTE',
        \RedisCluster::FAILOVER_DISTRIBUTE_SLAVES => 'DISTRIBUTE_SLAVES',
    ];

    public static function castRedis(\Redis|Relay $c, array $a, Stub $stub, bool $isNested): array
    {}

    public static function castRedisArray(\RedisArray $c, array $a, Stub $stub, bool $isNested): array
    {}

    public static function castRedisCluster(\RedisCluster $c, array $a, Stub $stub, bool $isNested): array
    {}

    private static function getRedisOptions(\Redis|Relay|\RedisArray|\RedisCluster $redis, array $options = []): EnumStub
    {}
}
